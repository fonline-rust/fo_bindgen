use fo_bindgen_generate::Generator;

fn main() {
    let string = std::fs::read_to_string("generator.toml").expect("Can't open generator.toml");
    let generator: Generator = toml::from_str(&string).expect("Can't parse generator.toml");
    generator.start();
}
