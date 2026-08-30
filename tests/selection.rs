use soksak_kit_sidecar_terminal::mirror::{
    CellSide, EngineSelectionPoint, SelectionKind, SelectionModifiers, TerminalEngine,
};
use soksak_sidecar_terminal_kitty::engine::Engine;

#[test]
fn simple_drag_uses_kitty_screen_selection_and_text() {
    let marker = "SELECT_KITTY_1234567890";
    let mut engine = Engine::new(40, 3);
    engine.feed(marker.as_bytes());

    TerminalEngine::selection_begin(
        &mut engine,
        SelectionKind::Simple,
        EngineSelectionPoint {
            line: 0,
            col: 0,
            side: CellSide::Left,
        },
        SelectionModifiers::default(),
    )
    .expect("begin Kitty selection");
    TerminalEngine::selection_update(
        &mut engine,
        EngineSelectionPoint {
            line: 0,
            col: u16::try_from(marker.len()).unwrap(),
            side: CellSide::Left,
        },
        SelectionModifiers::default(),
    )
    .expect("update Kitty selection");

    assert_eq!(
        TerminalEngine::selection_text(&engine).as_deref(),
        Some(marker),
    );
    assert_eq!(
        TerminalEngine::selection_range(&engine, 0),
        Some((0, u16::try_from(marker.len() - 1).unwrap())),
    );
}

#[test]
fn semantic_and_line_kinds_use_kitty_selection_modes() {
    let mut semantic = Engine::new(40, 3);
    semantic.feed(b"alpha beta");
    TerminalEngine::selection_begin(
        &mut semantic,
        SelectionKind::Semantic,
        EngineSelectionPoint {
            line: 0,
            col: 7,
            side: CellSide::Left,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    TerminalEngine::selection_update(
        &mut semantic,
        EngineSelectionPoint {
            line: 0,
            col: 7,
            side: CellSide::Right,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    assert_eq!(TerminalEngine::selection_text(&semantic).as_deref(), Some("beta"));

    let mut line = Engine::new(40, 3);
    line.feed(b"  hello line  ");
    TerminalEngine::selection_begin(
        &mut line,
        SelectionKind::Line,
        EngineSelectionPoint {
            line: 0,
            col: 5,
            side: CellSide::Left,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    TerminalEngine::selection_update(
        &mut line,
        EngineSelectionPoint {
            line: 0,
            col: 5,
            side: CellSide::Right,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    assert_eq!(
        TerminalEngine::selection_text(&line).as_deref(),
        Some("hello line"),
    );
}

#[test]
fn kitty_selection_extends_the_existing_native_range() {
    let mut engine = Engine::new(40, 3);
    engine.feed(b"alpha beta");
    TerminalEngine::selection_begin(
        &mut engine,
        SelectionKind::Simple,
        EngineSelectionPoint {
            line: 0,
            col: 0,
            side: CellSide::Left,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    TerminalEngine::selection_update(
        &mut engine,
        EngineSelectionPoint {
            line: 0,
            col: 5,
            side: CellSide::Left,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    TerminalEngine::selection_begin(
        &mut engine,
        SelectionKind::Extend,
        EngineSelectionPoint {
            line: 0,
            col: 10,
            side: CellSide::Right,
        },
        SelectionModifiers::default(),
    )
    .unwrap();
    assert_eq!(
        TerminalEngine::selection_text(&engine).as_deref(),
        Some("alpha beta"),
    );
}
