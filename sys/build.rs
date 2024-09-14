use std::env;

fn runtime() -> &'static str {
    if env::var("CARGO_CFG_TARGET_FEATURE")
        .unwrap_or_default()
        .contains("crt-static")
    {
        "MT"
    } else {
        "MD"
    }
}

fn arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86") => "x86",
        Ok("x86_64") => "x64",
        Ok("aarch64") => "arm64",
        Ok("arm64ec") => "arm64ec",
        Ok(_) | Err(_) => panic!("Unsupported architecture!"),
    }
}

fn main() {
    println!(
        "cargo:rustc-link-search={}/external/lib/{}/",
        std::env!("CARGO_MANIFEST_DIR"),
        arch(),
    );
    println!("cargo:rustc-link-lib=PerformanceAPI_{}", runtime());
}
