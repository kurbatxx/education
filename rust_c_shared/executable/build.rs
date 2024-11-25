fn main() {
    println!("cargo:rustc-link-lib=dylib=awesome_math");
    println!("cargo:rustc-link-search=native=./../library");
    println!("cargo:rustc-link-arg=-Wl,-rpath,./../library");
}
