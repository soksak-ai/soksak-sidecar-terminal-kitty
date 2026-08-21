#[test]
fn absent_service_fails() {
    soksak_kit_sidecar_terminal::integration::assert_absent_service_fails(
        std::path::Path::new("/tmp/soksak-absent-terminal-service"),
        "soksak-sidecar-terminal-kitty",
    );
}
