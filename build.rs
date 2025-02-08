extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("src");
    config.compile_protos(&["src/payload.proto"], &["src/"]).unwrap();
}