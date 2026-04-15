fn main() {
    // Tell the Rust compiler to ALWAYS look in MacPorts for native C++ libraries
    println!("cargo:rustc-link-search=native=/opt/local/lib");
}
