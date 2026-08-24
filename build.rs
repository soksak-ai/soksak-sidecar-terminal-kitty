use std::path::{Path, PathBuf};

fn main() {
    let target = std::env::var("TARGET").expect("Cargo target");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("Cargo target OS");
    if target_os != "macos" && target_os != "linux" {
        panic!("Kitty provider supports macOS and Linux; target OS is {target_os}");
    }
    let root = PathBuf::from(
        std::env::var("SOKSAK_BUILD_DEPENDENCY_ROOT")
            .expect("make build supplies SOKSAK_BUILD_DEPENDENCY_ROOT"),
    );
    assert!(root.is_absolute(), "build dependency root must be absolute");
    assert_eq!(
        std::fs::canonicalize(&root).expect("canonical build dependency root"),
        root,
        "build dependency root must not use a symbolic path"
    );
    let receipt_path = root.join("receipts").join(format!("{target}.json"));
    let receipt: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&receipt_path).expect("read Kitty build dependency receipt"),
    )
    .expect("parse Kitty build dependency receipt");
    assert_eq!(receipt["schema"], "soksak-build-dependency-receipt-v1");
    assert_eq!(receipt["dependency"], "kitty-provider-sdk");
    assert_eq!(receipt["target"], target);
    let tree = receipt["outputs"]
        .as_array()
        .expect("receipt outputs")
        .iter()
        .find(|output| output["type"] == "tree")
        .and_then(|output| output["path"].as_str())
        .expect("Kitty receipt tree output");
    let sdk = root.join(Path::new(tree));
    assert_eq!(
        std::fs::canonicalize(&sdk).expect("canonical Kitty SDK"),
        sdk,
        "Kitty SDK must not use a symbolic path"
    );

    let archive = sdk.join("lib/libkitty_provider.a");
    let extension = sdk.join("python/kitty/fast_data_types.so");
    let config = sdk.join("python-config.json");
    for path in [&archive, &extension, &config] {
        assert!(
            path.is_file(),
            "Kitty provider SDK file is missing: {}",
            path.display()
        );
        let canonical = std::fs::canonicalize(path).expect("canonical Kitty SDK file");
        assert_eq!(
            canonical.as_path(),
            path.as_path(),
            "Kitty SDK file must not use a symbolic path"
        );
        println!("cargo:rerun-if-changed={}", path.display());
    }
    let value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&config).expect("read Kitty Python config"))
            .expect("parse Kitty Python config");
    let library = value["library"].as_str().expect("Python library");
    let library_dir = sdk.join("runtime/lib");
    assert!(
        library_dir.is_dir(),
        "Kitty SDK runtime library directory is missing"
    );

    println!(
        "cargo:rustc-link-search=native={}",
        sdk.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=kitty_provider");
    println!("cargo:rustc-link-search=native={}", library_dir.display());
    println!("cargo:rustc-link-lib={library}");
    let origin = if target_os == "macos" {
        "@loader_path"
    } else {
        "$ORIGIN"
    };
    println!("cargo:rustc-link-arg=-Wl,-rpath,{origin}/kitty-provider/runtime/lib");
    println!("cargo:rerun-if-changed={}", receipt_path.display());
    println!("cargo:rerun-if-env-changed=SOKSAK_BUILD_DEPENDENCY_ROOT");
}
