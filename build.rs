extern crate cbindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = "target".to_string();
    let profile = env::var("PROFILE").unwrap();
    //let name = env::var("CARGO_PKG_NAME").unwrap();
    let name = String::from("fiatluz");
    let file_ending = String::from(".h");
    let file_name = name + &file_ending;

    let out_path: PathBuf = Path::new(&target_dir).join(&profile).join(&file_name);

    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    config.parse.parse_deps = true;
    config.parse.include = vec![String::from("fiatluz")].into();
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(out_path);
}
