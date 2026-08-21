use std::{fs, path::PathBuf};

#[test]
fn stage_uses_the_declared_cargo_target_directory() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script = fs::read_to_string(root.join("stage.sh")).expect("read stage.sh");

    assert!(script.contains("release_dir=release"));
    assert!(script.contains("release_dir=\"$target/release\""));
    assert!(script.contains("${CARGO_TARGET_DIR:-target}/$release_dir/soksak-sidecar-terminal-kitty"));
    assert!(script.contains("-name '*.pyc' -o -name '*.pyo'"));
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
}
