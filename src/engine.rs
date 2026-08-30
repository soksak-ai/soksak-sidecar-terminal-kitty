use std::ffi::{CString, c_char, c_int, c_void};
use std::ptr::NonNull;
use std::sync::OnceLock;

use soksak_kit_sidecar_terminal::mirror::TerminalEngine;
pub use soksak_kit_sidecar_terminal::mirror::{
    EnginePointerInput, EngineSelectionPoint, EngineWheelInput, SelectionKind, SelectionModifiers,
    TerminalCell as GridCell, TerminalColor as ColorSnap, TerminalCursorAnimation,
    TerminalCursorShape, TerminalCursorStyle, TerminalModes as ModeSnap, TerminalRgb,
    TerminalThemeOverrides,
};

const ATTR_BOLD: u16 = 1 << 0;
const ATTR_DIM: u16 = 1 << 1;
const ATTR_ITALIC: u16 = 1 << 2;
const ATTR_UNDERLINE: u16 = 1 << 3;
const ATTR_INVERSE: u16 = 1 << 4;
const ATTR_STRIKE: u16 = 1 << 5;

#[repr(C)]
#[derive(Default)]
struct Snapshot {
    columns: u32,
    rows: u32,
    history: u32,
    cursor_x: u32,
    cursor_y: u32,
    modes: u32,
    cursor_shape: u32,
    cursor_blinking: u32,
    suppressed_replies: u64,
}

#[repr(C)]
#[derive(Default)]
struct Cell {
    fg: u32,
    bg: u32,
    attrs: u16,
    wide: u8,
    spacer: u8,
    wrapline: u8,
}

#[repr(C)]
struct ProviderThemeOverrides {
    foreground: u32,
    background: u32,
    cursor: u32,
    palette: [u32; 256],
    has_foreground: u8,
    has_background: u8,
    has_cursor: u8,
    has_palette: [u8; 256],
}

impl Default for ProviderThemeOverrides {
    fn default() -> Self {
        Self {
            foreground: 0,
            background: 0,
            cursor: 0,
            palette: [0; 256],
            has_foreground: 0,
            has_background: 0,
            has_cursor: 0,
            has_palette: [0; 256],
        }
    }
}

unsafe extern "C" {
    fn kitty_provider_runtime_init(sdk_root: *const c_char) -> c_int;
    fn kitty_provider_new(columns: u16, rows: u16, scrollback: u32) -> *mut c_void;
    fn kitty_provider_free(provider: *mut c_void);
    fn kitty_provider_feed(provider: *mut c_void, data: *const u8, length: usize) -> c_int;
    fn kitty_provider_resize(provider: *mut c_void, columns: u16, rows: u16) -> c_int;
    fn kitty_provider_snapshot(provider: *mut c_void, snapshot: *mut Snapshot) -> c_int;
    fn kitty_provider_theme_overrides(
        provider: *mut c_void,
        overrides: *mut ProviderThemeOverrides,
    ) -> c_int;
    fn kitty_provider_pointer(
        provider: *mut c_void,
        column: i32,
        row: i32,
        button: i32,
        action: i32,
        modifiers: i32,
        output: *mut u8,
        capacity: usize,
        required: *mut usize,
    ) -> c_int;
    fn kitty_provider_cell(
        provider: *mut c_void,
        row: i32,
        column: u16,
        cell: *mut Cell,
        codepoints: *mut u32,
        capacity: usize,
        required: *mut usize,
    ) -> c_int;
    fn kitty_provider_selection_start(
        provider: *mut c_void,
        column: i32,
        row: i32,
        side: i32,
        kind: i32,
    ) -> c_int;
    fn kitty_provider_selection_update(
        provider: *mut c_void,
        column: i32,
        row: i32,
        side: i32,
    ) -> c_int;
    fn kitty_provider_selection_clear(provider: *mut c_void) -> c_int;
    fn kitty_provider_selection_text(
        provider: *mut c_void,
        output: *mut u8,
        capacity: usize,
        required: *mut usize,
    ) -> c_int;
    fn kitty_provider_selection_range(
        provider: *mut c_void,
        row: i32,
        start: *mut u16,
        end: *mut u16,
    ) -> c_int;
}

static RUNTIME: OnceLock<()> = OnceLock::new();

pub struct Engine {
    provider: NonNull<c_void>,
}
unsafe impl Send for Engine {}

impl Engine {
    pub fn new(columns: u16, rows: u16) -> Self {
        RUNTIME.get_or_init(|| {
            let sdk = std::env::current_exe()
                .expect("current executable")
                .parent()
                .expect("sidecar executable directory")
                .join("kitty-provider")
                .to_string_lossy()
                .into_owned();
            let python =
                CString::new(format!("{sdk}/python")).expect("Kitty SDK path contains NUL");
            assert_eq!(
                unsafe { kitty_provider_runtime_init(python.as_ptr()) },
                0,
                "Kitty runtime initialization failed"
            );
        });
        let provider = unsafe { kitty_provider_new(columns, rows, 1000) };
        Self {
            provider: NonNull::new(provider).expect("Kitty Screen creation failed"),
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) {
        assert_eq!(
            unsafe { kitty_provider_feed(self.provider.as_ptr(), bytes.as_ptr(), bytes.len()) },
            0
        );
    }
    pub fn resize(&mut self, columns: u16, rows: u16) {
        assert_eq!(
            unsafe { kitty_provider_resize(self.provider.as_ptr(), columns, rows) },
            0
        );
    }
    fn snapshot(&self) -> Snapshot {
        let mut value = Snapshot::default();
        assert_eq!(
            unsafe { kitty_provider_snapshot(self.provider.as_ptr(), &mut value) },
            0
        );
        value
    }
    pub fn cols(&self) -> u16 {
        self.snapshot().columns as u16
    }
    pub fn rows(&self) -> u16 {
        self.snapshot().rows as u16
    }
    pub fn cursor(&self) -> (usize, usize) {
        let s = self.snapshot();
        (s.cursor_y as usize, s.cursor_x as usize)
    }
    pub fn cursor_style(&self) -> TerminalCursorStyle {
        let snapshot = self.snapshot();
        let shape = match snapshot.cursor_shape {
            0 | 1 | 4 => TerminalCursorShape::Block,
            2 => TerminalCursorShape::Bar,
            3 => TerminalCursorShape::Underline,
            value => panic!("unknown Kitty cursor shape: {value}"),
        };
        TerminalCursorStyle {
            shape,
            blinking: snapshot.cursor_blinking != 0,
        }
    }
    pub fn cursor_animation(&self) -> TerminalCursorAnimation {
        TerminalCursorAnimation { interval_ms: 500 }
    }
    pub fn theme_overrides(&self) -> TerminalThemeOverrides {
        let mut raw = ProviderThemeOverrides::default();
        assert_eq!(
            unsafe { kitty_provider_theme_overrides(self.provider.as_ptr(), &mut raw) },
            0
        );
        let rgb = |value: u32| TerminalRgb {
            r: ((value >> 16) & 0xff) as u8,
            g: ((value >> 8) & 0xff) as u8,
            b: (value & 0xff) as u8,
        };
        let mut overrides = TerminalThemeOverrides::default();
        overrides.foreground = (raw.has_foreground != 0).then(|| rgb(raw.foreground));
        overrides.background = (raw.has_background != 0).then(|| rgb(raw.background));
        overrides.cursor = (raw.has_cursor != 0).then(|| rgb(raw.cursor));
        for (index, slot) in overrides.ansi.iter_mut().enumerate() {
            *slot = (raw.has_palette[index] != 0).then(|| rgb(raw.palette[index]));
        }
        overrides
    }
    pub fn history_size(&self) -> usize {
        self.snapshot().history as usize
    }
    pub fn alt_active(&self) -> bool {
        self.snapshot().modes & (1 << 13) != 0
    }
    pub fn suppressed_replies(&self) -> u64 {
        self.snapshot().suppressed_replies
    }
    pub fn modes(&self) -> ModeSnap {
        let m = self.snapshot().modes;
        ModeSnap {
            bracketed_paste: m & (1 << 0) != 0,
            app_cursor: m & (1 << 1) != 0,
            app_keypad: m & (1 << 2) != 0,
            mouse_click: m & (1 << 3) != 0,
            mouse_drag: m & (1 << 4) != 0,
            mouse_motion: m & (1 << 5) != 0,
            sgr_mouse: m & (1 << 6) != 0,
            utf8_mouse: m & (1 << 7) != 0,
            focus_in_out: m & (1 << 8) != 0,
            alternate_scroll: m & (1 << 9) != 0,
            show_cursor: m & (1 << 10) != 0,
            line_wrap: m & (1 << 11) != 0,
            insert: m & (1 << 12) != 0,
        }
    }
    pub fn line_cells(&self, row: i32) -> Vec<GridCell> {
        (0..self.cols())
            .map(|column| self.cell(row, column))
            .collect()
    }
    pub fn pointer_input(&mut self, input: EnginePointerInput) -> Result<Vec<u8>, String> {
        let button = match input.button {
            soksak_kit_sidecar_terminal::mirror::PointerButton::None => 0,
            soksak_kit_sidecar_terminal::mirror::PointerButton::Left => 1,
            soksak_kit_sidecar_terminal::mirror::PointerButton::Middle => 2,
            soksak_kit_sidecar_terminal::mirror::PointerButton::Right => 3,
        };
        let action = match input.phase {
            soksak_kit_sidecar_terminal::mirror::PointerPhase::Down => 0,
            soksak_kit_sidecar_terminal::mirror::PointerPhase::Up => 1,
            soksak_kit_sidecar_terminal::mirror::PointerPhase::Move if button != 0 => 2,
            soksak_kit_sidecar_terminal::mirror::PointerPhase::Move => 3,
        };
        let mut modifiers = 0i32;
        if input.modifiers.shift { modifiers |= 1; }
        if input.modifiers.control { modifiers |= 2; }
        if input.modifiers.alt { modifiers |= 4; }
        if input.modifiers.meta { modifiers |= 8; }
        let mut required = 0usize;
        let first = unsafe {
            kitty_provider_pointer(
                self.provider.as_ptr(), i32::from(input.col), i32::from(input.row),
                button, action, modifiers, std::ptr::null_mut(), 0, &mut required,
            )
        };
        if first != 0 && first != 1 {
            return Err(format!("Kitty mouse encoder failed: {first}"));
        }
        if required == 0 {
            return Ok(Vec::new());
        }
        let mut output = vec![0u8; required];
        let result = unsafe {
            kitty_provider_pointer(
                self.provider.as_ptr(), i32::from(input.col), i32::from(input.row),
                button, action, modifiers, output.as_mut_ptr(), output.len(), &mut required,
            )
        };
        if result != 0 {
            return Err(format!("Kitty mouse encoder retry failed: {result}"));
        }
        output.truncate(required);
        Ok(output)
    }
    pub fn selection_begin(
        &mut self,
        kind: SelectionKind,
        point: EngineSelectionPoint,
        _modifiers: SelectionModifiers,
    ) -> Result<(), String> {
        let kind = match kind {
            SelectionKind::Simple => 0,
            SelectionKind::Semantic => 1,
            SelectionKind::Line => 2,
            SelectionKind::Block => 3,
            SelectionKind::Extend => 4,
        };
        let side = match point.side {
            soksak_kit_sidecar_terminal::mirror::CellSide::Left => 0,
            soksak_kit_sidecar_terminal::mirror::CellSide::Right => 1,
        };
        let result = unsafe {
            kitty_provider_selection_start(
                self.provider.as_ptr(),
                i32::from(point.col),
                point.line,
                side,
                kind,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(format!("Kitty selection start failed: {result}"))
        }
    }
    pub fn selection_update(
        &mut self,
        point: EngineSelectionPoint,
        _modifiers: SelectionModifiers,
    ) -> Result<(), String> {
        let side = match point.side {
            soksak_kit_sidecar_terminal::mirror::CellSide::Left => 0,
            soksak_kit_sidecar_terminal::mirror::CellSide::Right => 1,
        };
        let result = unsafe {
            kitty_provider_selection_update(
                self.provider.as_ptr(),
                i32::from(point.col),
                point.line,
                side,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(format!("Kitty selection update failed: {result}"))
        }
    }
    pub fn selection_clear(&mut self) {
        assert_eq!(unsafe { kitty_provider_selection_clear(self.provider.as_ptr()) }, 0);
    }
    pub fn selection_text(&self) -> Option<String> {
        let mut required = 0usize;
        let first = unsafe {
            kitty_provider_selection_text(
                self.provider.as_ptr(),
                std::ptr::null_mut(),
                0,
                &mut required,
            )
        };
        if first == 2 {
            return None;
        }
        if first != 0 && first != 1 {
            return None;
        }
        let mut output = vec![0u8; required];
        if required != 0 {
            let result = unsafe {
                kitty_provider_selection_text(
                    self.provider.as_ptr(),
                    output.as_mut_ptr(),
                    output.len(),
                    &mut required,
                )
            };
            if result != 0 {
                return None;
            }
        }
        output.truncate(required);
        String::from_utf8(output).ok()
    }
    pub fn selection_range(&self, line: i32) -> Option<(u16, u16)> {
        let mut start = 0u16;
        let mut end = 0u16;
        let result = unsafe {
            kitty_provider_selection_range(
                self.provider.as_ptr(),
                line,
                &mut start,
                &mut end,
            )
        };
        (result == 0).then_some((start, end))
    }
    fn cell(&self, row: i32, column: u16) -> GridCell {
        let mut cell = Cell::default();
        let mut required = 0usize;
        let first = unsafe {
            kitty_provider_cell(
                self.provider.as_ptr(),
                row,
                column,
                &mut cell,
                std::ptr::null_mut(),
                0,
                &mut required,
            )
        };
        assert!(first == 0 || first == 1);
        let mut points = vec![0u32; required];
        if required != 0 {
            assert_eq!(
                unsafe {
                    kitty_provider_cell(
                        self.provider.as_ptr(),
                        row,
                        column,
                        &mut cell,
                        points.as_mut_ptr(),
                        points.len(),
                        &mut required,
                    )
                },
                0
            );
        }
        let mut chars = points
            .into_iter()
            .map(|point| char::from_u32(point).unwrap_or(char::REPLACEMENT_CHARACTER));
        GridCell {
            ch: chars.next().unwrap_or(' '),
            fg: color(cell.fg),
            bg: color(cell.bg),
            bold: cell.attrs & ATTR_BOLD != 0,
            dim: cell.attrs & ATTR_DIM != 0,
            italic: cell.attrs & ATTR_ITALIC != 0,
            underline: cell.attrs & ATTR_UNDERLINE != 0,
            inverse: cell.attrs & ATTR_INVERSE != 0,
            strikeout: cell.attrs & ATTR_STRIKE != 0,
            hidden: false,
            wide: cell.wide != 0,
            spacer: cell.spacer != 0,
            wrapline: cell.wrapline != 0,
            zerowidth: chars.collect(),
            // This engine does not track OSC 8; capabilities.hyperlinks stays false.
            link: None,
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe { kitty_provider_free(self.provider.as_ptr()) };
    }
}
impl TerminalEngine for Engine {
    fn new(c: u16, r: u16) -> Self {
        Engine::new(c, r)
    }
    fn feed(&mut self, b: &[u8]) {
        Engine::feed(self, b)
    }
    fn resize(&mut self, c: u16, r: u16) {
        Engine::resize(self, c, r)
    }
    fn cols(&self) -> u16 {
        Engine::cols(self)
    }
    fn rows(&self) -> u16 {
        Engine::rows(self)
    }
    fn cursor(&self) -> (usize, usize) {
        Engine::cursor(self)
    }
    fn cursor_style(&self) -> TerminalCursorStyle {
        Engine::cursor_style(self)
    }
    fn cursor_animation(&self) -> TerminalCursorAnimation {
        Engine::cursor_animation(self)
    }
    fn alt_active(&self) -> bool {
        Engine::alt_active(self)
    }
    fn history_size(&self) -> usize {
        Engine::history_size(self)
    }
    fn modes(&self) -> ModeSnap {
        Engine::modes(self)
    }
    fn line_cells(&self, l: i32) -> Vec<GridCell> {
        Engine::line_cells(self, l)
    }
    fn suppressed_replies(&self) -> u64 {
        Engine::suppressed_replies(self)
    }
    fn theme_overrides(&self) -> TerminalThemeOverrides {
        Engine::theme_overrides(self)
    }
    fn selection_begin(
        &mut self,
        kind: SelectionKind,
        point: EngineSelectionPoint,
        modifiers: SelectionModifiers,
    ) -> Result<(), String> {
        Engine::selection_begin(self, kind, point, modifiers)
    }
    fn selection_update(
        &mut self,
        point: EngineSelectionPoint,
        modifiers: SelectionModifiers,
    ) -> Result<(), String> {
        Engine::selection_update(self, point, modifiers)
    }
    fn selection_clear(&mut self) {
        Engine::selection_clear(self)
    }
    fn selection_text(&self) -> Option<String> {
        Engine::selection_text(self)
    }
    fn selection_range(&self, line: i32) -> Option<(u16, u16)> {
        Engine::selection_range(self, line)
    }
    fn wheel_input(&mut self, _input: EngineWheelInput) -> Result<Vec<u8>, String> {
        Err("Kitty wheel input is not implemented".into())
    }
    fn pointer_input(&mut self, input: EnginePointerInput) -> Result<Vec<u8>, String> {
        Engine::pointer_input(self, input)
    }
}

fn color(value: u32) -> ColorSnap {
    match value & 0xff {
        0 => ColorSnap::Default,
        1 => ColorSnap::Indexed(((value >> 8) & 0xff) as u8),
        2 => {
            let rgb = value >> 8;
            ColorSnap::Rgb(
                ((rgb >> 16) & 0xff) as u8,
                ((rgb >> 8) & 0xff) as u8,
                (rgb & 0xff) as u8,
            )
        }
        tag => panic!("unknown Kitty color tag {tag}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{ColorSnap, Engine, TerminalRgb};
    use soksak_kit_sidecar_terminal::mirror::TerminalEngine;

    #[test]
    fn rgb_sgr_is_identical_across_every_input_boundary() {
        let bytes = b"\x1b[38;2;12;34;56mA\x1b[48;2;78;90;123mB";
        let mut whole = Engine::new(4, 1);
        whole.feed(bytes);
        let expected = whole.line_cells(0);

        for boundary in 1..bytes.len() {
            let mut split = Engine::new(4, 1);
            split.feed(&bytes[..boundary]);
            split.feed(&bytes[boundary..]);
            assert_eq!(split.line_cells(0), expected, "boundary {boundary}");
        }
        assert_eq!(expected[0].fg, ColorSnap::Rgb(12, 34, 56));
        assert_eq!(expected[1].bg, ColorSnap::Rgb(78, 90, 123));
    }

    #[test]
    fn rgb_values_outside_the_fast_path_keep_the_production_parser_rule() {
        let mut engine = Engine::new(2, 1);
        engine.feed(b"\x1b[38;2;256;511;1024mX");
        assert_eq!(engine.line_cells(0)[0].fg, ColorSnap::Rgb(0, 255, 0));
    }

    #[test]
    fn engine_exposes_raw_osc_color_overrides() {
        let mut engine = Engine::new(4, 1);
        engine.feed(
            b"\x1b]4;1;#123456\x07\x1b]10;#abcdef\x07\x1b]11;#223344\x07\x1b]12;#654321\x07",
        );
        let colors = TerminalEngine::theme_overrides(&engine);
        assert_eq!(
            colors.ansi[1],
            Some(TerminalRgb {
                r: 0x12,
                g: 0x34,
                b: 0x56
            })
        );
        assert_eq!(
            colors.foreground,
            Some(TerminalRgb {
                r: 0xab,
                g: 0xcd,
                b: 0xef
            })
        );
        assert_eq!(
            colors.background,
            Some(TerminalRgb {
                r: 0x22,
                g: 0x33,
                b: 0x44
            })
        );
        assert_eq!(
            colors.cursor,
            Some(TerminalRgb {
                r: 0x65,
                g: 0x43,
                b: 0x21
            })
        );

        engine.feed(b"\x1b]104;1\x07\x1b]110\x07\x1b]111\x07\x1b]112\x07");
        let reset = TerminalEngine::theme_overrides(&engine);
        assert_eq!(reset.ansi[1], None);
        assert_eq!(
            (reset.foreground, reset.background, reset.cursor),
            (None, None, None)
        );
    }
}
