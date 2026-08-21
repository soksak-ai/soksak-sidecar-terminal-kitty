use std::{fs, path::PathBuf};

#[test]
fn stage_uses_the_declared_cargo_target_directory() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script = fs::read_to_string(root.join("stage.sh")).expect("read stage.sh");

    assert!(
        script.contains("${CARGO_TARGET_DIR:-target}/release/soksak-sidecar-terminal-kitty"),
        "stage.sh must copy the binary from CARGO_TARGET_DIR"
    );
}
