use std::path::PathBuf;

fn main() {
    let sdk = std::env::var("SOKSAK_KITTY_PROVIDER_SDK")
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .expect("SOKSAK_KITTY_PROVIDER_SDK must declare the Kitty provider SDK directory");
    let archive = sdk.join("lib/libkitty_provider.a");
    let extension = sdk.join("python/kitty/fast_data_types.so");
    let config = sdk.join("python-config.json");
    for path in [&archive, &extension, &config] {
        assert!(
            path.is_file(),
            "Kitty provider SDK file is missing: {}",
            path.display()
        );
        println!("cargo:rerun-if-changed={}", path.display());
    }
    let value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&config).expect("read Kitty Python config"))
            .expect("parse Kitty Python config");
    let library_dir = value["library_dir"].as_str().expect("Python library_dir");
    let library = value["library"].as_str().expect("Python library");
    println!(
        "cargo:rustc-link-search=native={}",
        sdk.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=kitty_provider");
    println!("cargo:rustc-link-search=native={library_dir}");
    println!("cargo:rustc-link-lib={library}");
    if std::env::var_os("SOKSAK_KITTY_BUNDLE_BUILD").is_some() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/kitty-provider/runtime/lib");
    } else {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{library_dir}");
    }
    println!("cargo:rerun-if-env-changed=SOKSAK_KITTY_PROVIDER_SDK");
    println!("cargo:rerun-if-env-changed=SOKSAK_KITTY_BUNDLE_BUILD");
}
