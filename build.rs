use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let shader_dir = PathBuf::from("src/shaders/compiled");

    println!("cargo:rerun-if-changed=src/shaders/compiled");

    for entry in fs::read_dir(&shader_dir).expect("Failed to read shader directory") {
        let entry = entry.expect("Failed to read shader file");
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "spv" {
                let file_name = path.file_name().unwrap();
                let dest_path = out_dir.join(file_name);
                fs::copy(&path, &dest_path).expect("Failed to copy shader file");
            }
        }
    }
}