use std::{ffi::OsStr, path::PathBuf};
use walkdir::WalkDir;

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let protos_folder = "./protos/sources";
    let third_party_folder = "./protos/third_party";
    let mut files: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(protos_folder) {
        if let Ok(diren) = entry {
            if diren.path().is_file()
                && diren.path().extension().and_then(OsStr::to_str).unwrap() == "proto"
            {
                println!("{}", diren.path().display());
                files.push(diren.path().to_path_buf())
            }
        }
    }

    for entry in WalkDir::new(third_party_folder) {
        if let Ok(diren) = entry {
            if diren.path().is_file()
                && diren.path().extension().and_then(OsStr::to_str).unwrap() == "proto"
            {
                println!("{}", diren.path().display());
                files.push(diren.path().to_path_buf())
            }
        }
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("./src")
        .compile(&files, &[protos_folder, third_party_folder])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", protos_folder);
}
