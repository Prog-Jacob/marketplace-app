use glob::glob;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files: Vec<_> = glob("../shared/proto/*.proto")
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .collect();

    tonic_build::configure()
        .compile_well_known_types(true)
        .extern_path(".google.rpc.Code", "::prost::alloc::vec::Vec")
        .compile(&proto_files, &["../shared/proto"])
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}
