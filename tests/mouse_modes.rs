use soksak_kit_sidecar_terminal::mirror::{
    EnginePointerInput, EngineWheelInput, EngineWheelRoute, PointerButton, PointerPhase,
    SelectionModifiers,
};
use soksak_sidecar_terminal_kitty::engine::Engine;

fn wheel() -> EngineWheelInput {
    EngineWheelInput {
        row: 0,
        col: 0,
        horizontal: 0,
        vertical: -1,
        modifiers: SelectionModifiers::default(),
        route: EngineWheelRoute::MouseReport,
    }
}

fn pointer() -> EnginePointerInput {
    EnginePointerInput {
        row: 0,
        col: 0,
        phase: PointerPhase::Down,
        button: PointerButton::Left,
        click_count: 1,
        modifiers: SelectionModifiers::default(),
    }
}

#[test]
fn unsupported_legacy_tracking_modes_are_not_aliased_or_admitted() {
    let mut engine = Engine::new(80, 24);
    engine.feed(b"\x1b[?9h\x1b[?1001h");

    let modes = engine.modes();
    assert!(!modes.mouse_x10);
    assert!(!modes.mouse_highlight);
    assert!(!modes.mouse_click);
    assert!(!modes.mouse_drag);
    assert!(!modes.mouse_motion);
    assert!(engine.wheel_input(wheel()).is_err());
    assert!(engine.pointer_input(pointer()).is_err());
}

#[test]
fn supported_tracking_modes_stay_distinct_from_unsupported_legacy_modes() {
    for (sequence, expected) in [
        (b"\x1b[?1000h".as_slice(), (true, false, false)),
        (b"\x1b[?1002h".as_slice(), (false, true, false)),
        (b"\x1b[?1003h".as_slice(), (false, false, true)),
    ] {
        let mut engine = Engine::new(80, 24);
        engine.feed(sequence);

        let modes = engine.modes();
        assert!(!modes.mouse_x10);
        assert!(!modes.mouse_highlight);
        assert_eq!(
            (modes.mouse_click, modes.mouse_drag, modes.mouse_motion),
            expected,
        );
        assert!(engine.wheel_input(wheel()).is_ok());
        assert!(engine.pointer_input(pointer()).is_ok());
    }
}
