use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo::rustc-check-cfg=cfg(arm7)");
    println!("cargo::rustc-check-cfg=cfg(arm9)");

    match std::env::var("TARGET")?.as_str() {
        "armv4t-nintendo-ds" | "armv4t-nintendo-dsi" => println!("cargo::rustc-cfg=arm7"),
        "armv5te-nintendo-ds" | "armv5te-nintendo-dsi" => println!("cargo::rustc-cfg=arm9"),
        _ => (),
    }

    Ok(())
}
