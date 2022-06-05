use bindgen::{builder, CodegenConfig};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Generator {
    pub args: Vec<String>,
    pub input: String,
    pub output: String,
}

impl Generator {
    fn generate(&self, server: bool, types: &[&str], prefix: &str, file: &str, opaque: bool) {
        if opaque {
            self.generate_custom(server, types, types, &[], prefix, file);
        } else {
            self.generate_custom(server, types, &[], &[], prefix, file);
        }
    }
    fn generate_custom(
        &self,
        server: bool,
        types: &[&str],
        opaque: &[&str],
        blocklist: &[&str],
        prefix: &str,
        file: &str,
    ) {
        let mut builder = builder()
            .clang_args(&self.args)
            .header(&self.input)
            .raw_line("#[allow(unused_imports)] use super::*;\n")
            .allowlist_recursively(false)
            .with_codegen_config(CodegenConfig::TYPES);

        if server {
            builder = builder.clang_arg("-D__SERVER");
        } else {
            builder = builder.clang_arg("-D__CLIENT");
        }

        for ty in types {
            builder = builder.allowlist_type(ty);
        }
        for ty in opaque {
            builder = builder.opaque_type(ty);
        }
        for ty in blocklist {
            builder = builder.blocklist_type(ty);
        }
        let bindings = builder.generate().unwrap();
        bindings
            .write_to_file(format!("{}{}{}", &self.output, prefix, file))
            .unwrap();
    }

    fn generate_conditional(&self, server: bool) {
        let prefix = if server { "server/" } else { "client/" };

        std::fs::create_dir_all(format!("{}{}", &self.output, prefix)).unwrap();
        self.generate_custom(
            server,
            &[
                "Critter",
                "Client",
                "Npc",
                "NpcPlane",
                "CritterTimeEvent",
                "GlobalMapGroup",
                "CritterCl",
            ],
            if server {
                &["CritterCl"][..]
            } else {
                &[
                    "Critter",
                    "Client",
                    "Npc",
                    "NpcPlane",
                    "CritterTimeEvent",
                    "GlobalMapGroup",
                ][..]
            },
            &[],
            prefix,
            "critter.rs",
        );

        self.generate(server, &["Item", "ProtoItem"], prefix, "item.rs", false);

        let map_types = [
            "Location",
            "Map",
            "ProtoMap",
            "SceneryToClient",
            "MapObject",
            "ProtoLocation",
            "MapEntire",
        ];
        let map_opaque = if server { &[][..] } else { &map_types[..] };
        self.generate_custom(
            server,
            &map_types,
            map_opaque,
            &["ProtoMap_TileVec"],
            prefix,
            "map.rs",
        );
        self.generate(
            server,
            &["Sprite", "SpriteInfo", "SpriteAnim"],
            prefix,
            "sprite.rs",
            server,
        );
        if server {
            self.generate_custom(
                server,
                &["Field", "Field_Tile"],
                &["Field", "Field_Tile"],
                &["Field_TileVec"],
                prefix,
                "field.rs",
            );
        } else {
            self.generate_custom(
                server,
                &["Field", "Field_Tile"],
                &[],
                &["Field_TileVec"],
                prefix,
                "field.rs",
            );
        }
        self.generate(
            server,
            &["GameOptions", "CritterMutual", "CritterType"],
            prefix,
            "state.rs",
            false,
        );
   }

    pub fn start(&self) {
        std::fs::create_dir_all(&self.output).unwrap();
        self.generate_conditional(true);
        self.generate_conditional(false);

        let opaque_types = ["Spinlock", "SyncObj", "Mutex"];
        self.generate_custom(
            true,
            &opaque_types,
            &opaque_types,
            &["ScriptArray_ArrayBuffer"],
            "",
            "opaque.rs",
        );

        let number_types = [
            "uint8", "uint16", "uint", "uint64", "int8", "int16", "int", "int64",
        ];
        self.generate(true, &number_types, "", "num.rs", false);
        self.generate(
            true,
            &[
                "ScriptString",
                "ScriptArray",
                "CScriptArray",
                "asIObjectType",
                "ArrayBuffer",
                "asDWORD",
                "asBYTE",
            ],
            "",
            "angelscript.rs",
            false,
        );
    }
}
