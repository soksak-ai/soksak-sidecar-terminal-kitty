use soksak_contract_terminal as contract;
use soksak_contract_terminal::MirrorUnderTest;
use soksak_sidecar_terminal_kitty::Mirror;
use soksak_sidecar_terminal_kitty::engine::{ColorSnap, GridCell, ModeSnap};

pub struct SidecarMirror(Mirror);

impl MirrorUnderTest for SidecarMirror {
    fn new(cols: u16, rows: u16) -> Self {
        Self(Mirror::new(cols, rows))
    }
    fn feed(&mut self, bytes: &[u8]) {
        self.0.feed(bytes);
    }
    fn resize(&mut self, cols: u16, rows: u16) {
        self.0.resize(cols, rows);
    }
    fn rehydrate(&self) -> Vec<u8> {
        self.0.rehydrate()
    }
    fn cold_paint(&self) -> Vec<u8> {
        self.0.cold_paint()
    }
    fn cursor_style(&self) -> contract::CursorStyle {
        let style = self.0.cursor_style();
        contract::CursorStyle {
            shape: match style.shape {
                soksak_kit_sidecar_terminal::mirror::TerminalCursorShape::Block => {
                    contract::CursorShape::Block
                }
                soksak_kit_sidecar_terminal::mirror::TerminalCursorShape::Underline => {
                    contract::CursorShape::Underline
                }
                soksak_kit_sidecar_terminal::mirror::TerminalCursorShape::Bar => {
                    contract::CursorShape::Bar
                }
            },
            blinking: style.blinking,
        }
    }
    fn suppressed_replies(&self) -> u64 {
        self.0.suppressed_replies()
    }

    fn screen_state(&self) -> contract::ScreenState {
        let (row, column) = self.0.cursor();
        let history = self.0.history_size() as i32;
        contract::ScreenState {
            cols: self.0.cols(),
            rows: self.0.rows(),
            alt: self.0.alt_active(),
            cursor: (column as u16, row as u16),
            modes: modes(self.0.modes()),
            history: (-history..0)
                .map(|line| row_state(self.0.line_cells(line)))
                .collect(),
            visible: (0..self.0.rows() as i32)
                .map(|line| row_state(self.0.line_cells(line)))
                .collect(),
        }
    }
}

fn row_state(cells: Vec<GridCell>) -> contract::Row {
    let cells = cells
        .into_iter()
        .filter(|cell| !cell.spacer)
        .map(|cell| {
            let mut text = String::new();
            text.push(cell.ch);
            text.extend(cell.zerowidth);
            contract::Cell {
                text,
                fg: color(cell.fg),
                bg: color(cell.bg),
                attrs: contract::Attrs {
                    bold: cell.bold,
                    dim: cell.dim,
                    italic: cell.italic,
                    underline: cell.underline,
                    inverse: cell.inverse,
                    strikeout: cell.strikeout,
                    hidden: cell.hidden,
                },
                wide: cell.wide,
            }
        })
        .collect();
    contract::Row::normalized(cells)
}

fn color(value: ColorSnap) -> contract::Color {
    match value {
        ColorSnap::Default => contract::Color::Default,
        ColorSnap::Named(index) | ColorSnap::Indexed(index) => contract::Color::Palette(index),
        ColorSnap::Rgb(red, green, blue) => contract::Color::Rgb(red, green, blue),
    }
}

fn modes(value: ModeSnap) -> contract::Modes {
    contract::Modes {
        bracketed_paste: value.bracketed_paste,
        app_cursor: value.app_cursor,
        app_keypad: value.app_keypad,
        mouse_click: value.mouse_click,
        mouse_drag: value.mouse_drag,
        mouse_motion: value.mouse_motion,
        sgr_mouse: value.sgr_mouse,
        utf8_mouse: value.utf8_mouse,
        focus_in_out: value.focus_in_out,
        alternate_scroll: value.alternate_scroll,
        show_cursor: value.show_cursor,
        line_wrap: value.line_wrap,
        insert: value.insert,
    }
}
