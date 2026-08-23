use std::path::PathBuf;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo target OS");
    if target_os != "macos" && target_os != "linux" {
        panic!("Kitty provider supports macOS and Linux; target OS is {target_os}");
    }
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
    let configured_library_dir = value["library_dir"].as_str().expect("Python library_dir");
    let bundled_library_dir = sdk.join("runtime/lib");
    let library_dir = if bundled_library_dir.is_dir() {
        bundled_library_dir.to_string_lossy().into_owned()
    } else {
        configured_library_dir.to_string()
    };
    let library = value["library"].as_str().expect("Python library");
    println!(
        "cargo:rustc-link-search=native={}",
        sdk.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=kitty_provider");
    println!("cargo:rustc-link-search=native={library_dir}");
    println!("cargo:rustc-link-lib={library}");
    if std::env::var_os("SOKSAK_KITTY_BUNDLE_BUILD").is_some() {
        let origin = if target_os == "macos" {
            "@loader_path"
        } else {
            "$ORIGIN"
        };
        println!("cargo:rustc-link-arg=-Wl,-rpath,{origin}/kitty-provider/runtime/lib");
    } else {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{library_dir}");
    }
    println!("cargo:rerun-if-env-changed=SOKSAK_KITTY_PROVIDER_SDK");
    println!("cargo:rerun-if-env-changed=SOKSAK_KITTY_BUNDLE_BUILD");
}
