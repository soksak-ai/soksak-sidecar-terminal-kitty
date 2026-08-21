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
    fn frame(&self) -> soksak_kit_sidecar_terminal::mirror::TerminalFrame { Mirror::frame(self) }
    fn alt_active(&self) -> bool {
        Mirror::alt_active(self)
    }
    fn suppressed_replies(&self) -> u64 {
        Mirror::suppressed_replies(self)
    }
}
