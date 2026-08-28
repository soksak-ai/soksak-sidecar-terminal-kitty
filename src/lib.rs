pub mod engine;
pub mod mirror;
pub use mirror::Mirror;

impl soksak_kit_sidecar_terminal::TerminalStateMirror for Mirror {
    fn feed(&mut self, bytes: &[u8]) {
        Mirror::feed(self, bytes);
    }
    fn resize(&mut self, cols: u16, rows: u16) {
        Mirror::resize(self, cols, rows);
    }
    fn rehydrate(&self) -> Vec<u8> {
        Mirror::rehydrate(self)
    }
    fn cold_paint(&self) -> Vec<u8> {
        Mirror::cold_paint(self)
    }
    fn frame_at(&self, offset: usize) -> soksak_kit_sidecar_terminal::mirror::TerminalFrame {
        Mirror::frame_at(self, offset)
    }
    fn cols(&self) -> u16 {
        Mirror::cols(self)
    }
    fn rows(&self) -> u16 {
        Mirror::rows(self)
    }
    fn cursor(&self) -> (usize, usize) {
        Mirror::cursor(self)
    }
    fn cursor_style(&self) -> soksak_kit_sidecar_terminal::mirror::TerminalCursorStyle {
        Mirror::cursor_style(self)
    }
    fn cursor_animation(&self) -> soksak_kit_sidecar_terminal::mirror::TerminalCursorAnimation {
        Mirror::cursor_animation(self)
    }
    fn line_cells(&self, line: i32) -> Vec<soksak_kit_sidecar_terminal::mirror::TerminalCell> {
        Mirror::line_cells(self, line)
    }
    fn history_size(&self) -> usize {
        Mirror::history_size(self)
    }
    fn modes(&self) -> soksak_kit_sidecar_terminal::mirror::TerminalModes {
        Mirror::modes(self)
    }
    fn capabilities(&self) -> soksak_kit_sidecar_terminal::mirror::MirrorCapabilities {
        Mirror::capabilities(self)
    }
    fn alt_active(&self) -> bool {
        Mirror::alt_active(self)
    }
    fn suppressed_replies(&self) -> u64 {
        Mirror::suppressed_replies(self)
    }
    fn theme_overrides(&self) -> soksak_kit_sidecar_terminal::mirror::TerminalThemeOverrides {
        Mirror::theme_overrides(self)
    }
}
