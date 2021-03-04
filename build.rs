fn main() {
    println!("cargo:rustc-env=RUST_LOG=rosu_v2=debug,error");
    println!("cargo:rustc-env=RUST_TEST_THREADS=1");
}
