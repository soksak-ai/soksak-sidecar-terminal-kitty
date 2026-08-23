use std::path::Path;

#[test]
fn sidecar_manifest_declares_the_staged_process() {
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string("sidecar.json").expect("read sidecar.json"))
            .expect("parse sidecar.json");
    assert!(manifest.get("spec").is_none());
    assert_eq!(manifest["id"], "soksak-sidecar-terminal-kitty");
    assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(manifest["interface"]["version"], "0.0.1");
    let process = manifest["process"].as_str().expect("process path");
    assert_eq!(process, "dist/soksak-sidecar-terminal-kitty");
    assert!(
        Path::new(process).is_file(),
        "stage.sh must create the declared process"
    );
}
