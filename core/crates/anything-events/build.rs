use std::{collections::HashSet, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap_or(".".to_string()));
    let mut protos = Protos::new(out_dir);

    protos.add_file("events.proto");

    protos.emit_build();
    protos.emit_rerun();

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

struct Protos {
    out_dir: PathBuf,
    includes: Vec<&'static str>,
    files: HashSet<String>,
}

impl Protos {
    pub fn new(out_dir: PathBuf) -> Self {
        Self {
            out_dir,
            includes: Vec::from(vec![".", "./proto"]),
            files: HashSet::new(),
        }
    }

    #[allow(unused)]
    pub fn add_include(&mut self, include_dir: &'static str) {
        self.includes.push(include_dir);
    }

    pub fn add_file(&mut self, file: impl Into<String>) {
        self.files.insert(file.into());
    }

    pub fn emit_build(&self) {
        for file in &self.files {
            let proto_file = format!("./proto/{}", file);
            tonic_build::configure()
                .protoc_arg("--experimental_allow_proto3_optional")
                .build_client(true)
                .build_server(true)
                .file_descriptor_set_path(self.out_dir.join("event_descriptor.bin"))
                .out_dir(&self.out_dir)
                .compile(&[&proto_file], &self.includes)
                .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
        }
    }

    pub fn emit_rerun(&self) {
        for file in &self.files {
            println!("cargo:rerun-if-changed={}", file);
        }
    }
}
