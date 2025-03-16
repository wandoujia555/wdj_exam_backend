use std::path::PathBuf;
fn main() {
    let path = PathBuf::from("./src");

    let config = dubbo_build::prost::configure();

    let mut prost_config = prost_build::Config::new();
    prost_config
        .out_dir(path)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config
        .compile_with_config(prost_config, &["proto/greeter.proto"], &["proto/"])
        .unwrap();
}
