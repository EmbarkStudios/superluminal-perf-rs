fn runtime() -> &'static str {
    if std::env::var("CARGO_CFG_TARGET_FEATURE")
        .unwrap_or_default()
        .contains("crt-static")
    {
        "MT"
    } else {
        "MD"
    }
}

fn main() {
    println!(
        "cargo:rustc-link-search={}/external/lib/x64/",
        std::env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=PerformanceAPI_{}", runtime());
}
