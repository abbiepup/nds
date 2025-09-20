fn main() {
    println!("cargo:rustc-link-search=native=target/armv5te-nintendo-dsi/release");

    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    println!("cargo:rustc-link-arg=target/armv5te-nintendo-dsi/release/libnds_rt.a");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");
}
