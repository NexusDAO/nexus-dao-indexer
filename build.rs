fn main() {
    println!("cargo:rerun-if-changed=proto");
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/substreams.proto"], &["proto"])
        .expect("Failed to compile Substreams proto(s)");

    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/record.proto"], &["proto"])
        .expect("Failed to compile Substreams record proto(s)");

    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/nexus_dao.mapping.proto"], &["proto"])
        .expect("Failed to compile Substreams record proto(s)");
}
