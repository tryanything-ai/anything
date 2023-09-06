fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files: &[&str] = &["event.proto"];

    proto_files.into_iter().for_each(|name| {
        let proto_file = &format!("proto/{}", name);
        tonic_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional") // For compatibility with protobuf-compiler 3.12 to 3.14 (e.g. on Ubuntu)
            .compile(&[proto_file], &["./", "./proto"])
            .unwrap_or_else(|e| panic!("protobuf compile for user_service error: {}", e));

        println!("cargo:rerun-if-changed={}", proto_file);
    });

    Ok(())
}
