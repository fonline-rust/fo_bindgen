#[macro_export]
macro_rules! exports {
    (common) => {
        pub mod critter;
        use critter::*;
        pub mod item;
        use item::*;

        pub mod sprite;
        use sprite::*;
        pub mod field;
        use field::*;
        pub mod state;

        use super::angelscript::*;
        use super::num::*;
        use super::opaque::*;
        use super::stl::*;

        $crate::stl!(common);
    };
    (server) => {
        $crate::exports!(common);
        pub mod map;
        use map::*;
        $crate::stl!(server);
    };
    (client) => {
        $crate::exports!(common);
        $crate::stl!(client);
    };
}

#[macro_export]
macro_rules! stl {
    (num) => {
        use std::os::raw::c_int as int;
        pub type UintPair = std_pair<uint, uint>;
        pub type Uint16Pair = std_pair<uint16, uint16>;

        pub type UintVec = std_vector<uint>;
        pub type Uint16Vec = std_vector<uint16>;
        pub type IntVec = std_vector<int>;
        pub type UintPairVec = std_vector<UintPair>;
        pub type Uint16PairVec = std_vector<Uint16Pair>;

        pub type IntSet = std_set<int>;
        pub type UintSet = std_set<uint>;
    };
    (common) => {
        pub type ItemVec = std_vector<*mut Item>;
    };
    (server) => {
        pub type NpcPlaneVec = std_vector<*mut NpcPlane>;
        pub type CrVec = std_vector<*mut Critter>;
        pub type ClVec = std_vector<*mut Client>;
        pub type PcVec = std_vector<*mut Npc>;
        pub type MapObjectVec = std_vector<*mut MapObject>;
        pub type MapVec = std_vector<*mut Map>;
        pub type LocVec = std_vector<*mut Location>;

        pub type CritterTimeEventVec = std_vector<CritterTimeEvent>;
        pub type EntiresVec = std_vector<MapEntire>;
        pub type SceneryToClientVec = std_vector<SceneryToClient>;
        pub type ProtoMap_TileVec = std_vector<ProtoMap_Tile>;

        pub type CrMap = std_map<uint, *mut Critter>;
    };
    (client) => {
        pub type CrClVec = std_vector<*mut CritterCl>;
        pub type Field_TileVec = std_vector<Field_Tile>;
    };
}
