fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate gRPC client code for JavaScript executor
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(
            &["../js-server/proto/js_executor.proto"],
            &["../js-server/proto"],
        )?;
    Ok(())
}
