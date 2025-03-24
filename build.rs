use std::path::PathBuf;
fn main() {
    let path = PathBuf::from("./src");

    let config = dubbo_build::prost::configure().output_dir(path.clone());
    let mut prost_config = prost_build::Config::new();
    prost_config
        .out_dir(path.clone())
        .btree_map(&["."])//匹配所有map为有序
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config
        .compile_with_config(prost_config, &["proto/greeter.proto"], &["proto/"])
        .unwrap();
}
