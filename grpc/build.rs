fn main() {
    tonic_build::compile_protos("proto/greeting.proto").unwrap();
}