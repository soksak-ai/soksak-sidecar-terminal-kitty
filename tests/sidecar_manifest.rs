use std::path::Path;

#[test]
fn sidecar_manifest_declares_the_staged_process() {
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string("sidecar.json").expect("read sidecar.json"))
            .expect("parse sidecar.json");
    assert!(manifest.get("spec").is_none());
    assert_eq!(manifest["id"], "soksak-sidecar-terminal-kitty");
    assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));
    let interface = manifest["interface"].as_array().expect("interface array");
    assert_eq!(interface.len(), 2);
    assert_eq!(interface[0]["id"], "soksak-spec-sidecar-terminal");
    assert_eq!(interface[0]["version"], "0.0.2");
    assert_eq!(interface[1]["id"], "soksak-spec-sidecar-surface");
    assert_eq!(interface[1]["version"], "0.0.2");
    let process = manifest["process"].as_str().expect("process path");
    assert_eq!(process, "dist/soksak-sidecar-terminal-kitty");
    let stage = std::env::var("SOKSAK_STAGE_OUT").expect("Make must declare the stage output");
    let stage = Path::new(&stage);
    assert!(stage.is_absolute(), "stage output must be absolute");
    let relative = process
        .strip_prefix("dist/")
        .expect("process must be inside dist");
    assert!(
        stage.join(relative).is_file(),
        "stage target must contain the declared process"
    );
}
