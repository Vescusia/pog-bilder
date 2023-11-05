fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:rerun-if-changed=../messages.proto");
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .cargo_out_dir("protos")
        .include("../")
        .input("../messages.proto")
        .run_from_script()
}