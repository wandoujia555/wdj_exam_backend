use std::path::PathBuf;
use prost_build::Config;
fn main() {
    let path = PathBuf::from("./src");

    let config = dubbo_build::prost::configure().output_dir(path);


    let mut prost_config = prost_build::Config::new();
    prost_config.out_dir(PathBuf::from("./src"))
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    
    config.compile_with_config(prost_config, &["proto/greeter.proto"], &["proto/"]).unwrap();

    // config
    //     .compile(&["proto/greeter.proto"], &["proto/"])
    //     .unwrap();


    // let path = std::path::PathBuf::from("./src");

    // let mut prost_config = prost_build::Config::new();
    // prost_config.out_dir(path)
    //     .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    //     .compile_protos(&["proto/greeter.proto"], &["proto/"])
    //     .unwrap();
}
