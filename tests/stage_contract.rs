use std::{fs, path::PathBuf};

#[test]
fn stage_uses_the_declared_cargo_target_directory() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script = fs::read_to_string(root.join("scripts/stage-built.sh")).expect("read stage-built");

    assert!(script.contains("target/$target/release/$name"));
    assert!(script.contains("SOKSAK_BUILD_DEPENDENCY_ROOT"));
    assert!(script.contains("targets/$target/kitty-provider"));
    assert!(script.contains("find \"$next_sdk\" -type l"));
    assert!(script.contains("STAGED_BUILD_NOT_DETERMINISTIC"));
    assert!(script.contains("$current_version\" != \"$next_version"));
    assert!(script.contains("KITTY_STAGED_UNCHANGED"));
}

#[test]
fn bundle_rpath_is_platform_specific() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let build = fs::read_to_string(root.join("build.rs")).expect("read build.rs");
    assert!(build.contains("let origin = if target_os == \"macos\""));
    assert!(build.contains("\"@loader_path\""));
    assert!(build.contains("\"$ORIGIN\""));
    assert!(build.contains("-Wl,-rpath,{origin}/kitty-provider/runtime/lib"));
    assert!(build.contains("sdk.join(\"runtime/lib\")"));
    assert!(!build.contains("configured_library_dir"));
}
