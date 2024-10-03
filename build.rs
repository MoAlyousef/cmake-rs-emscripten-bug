fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    cmake::Config::new("src/mylib").build();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=mylib");
}