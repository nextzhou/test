#[allow(non_snake_case)]
extern crate cc;

use std::path::PathBuf;

fn main() {
    bindgen_rocksdb();

    let mut build = cc::Build::new();
    build.file("redis-3.2.8/src/zmalloc.c");
    build.file("redis-3.2.8/src/sdsalloc.h");
    build.file("redis-3.2.8/src/sds.h");
    build.file("redis-3.2.8/src/sds.c");
    build.compile("redisserver");
}

fn bindgen_rocksdb() {
    let bindings = bindgen::Builder::default()
        .header("redis-3.2.8/src/sds.h")
        .raw_line("#[allow(non_snake_case)]")
        .blacklist_type("max_align_t") // https://github.com/rust-lang-nursery/rust-bindgen/issues/550
        .ctypes_prefix("libc")
        .generate()
        .expect("unable to generate rocksdb bindings");

    // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write rocksdb bindings");
}
