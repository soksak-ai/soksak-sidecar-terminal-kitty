mod common;

use common::SidecarMirror;
use soksak_contract_terminal as contract;
use soksak_contract_terminal::Fixture;

#[test]
fn mid_escape_tail() {
    contract::assert_conforms::<SidecarMirror>(Fixture::MidEscapeTail);
}
#[test]
fn cjk_width() {
    contract::assert_conforms::<SidecarMirror>(Fixture::CjkWidth);
}
#[test]
fn alt_screen() {
    contract::assert_conforms::<SidecarMirror>(Fixture::AltScreen);
}
#[test]
fn private_modes() {
    contract::assert_conforms::<SidecarMirror>(Fixture::PrivateModes);
}
#[test]
fn replay_guard() {
    contract::assert_conforms::<SidecarMirror>(Fixture::ReplayGuard);
}
#[test]
fn cold_paint_alt() {
    contract::assert_conforms::<SidecarMirror>(Fixture::ColdPaintAlt);
}
#[test]
fn dec_line_drawing() {
    contract::assert_conforms::<SidecarMirror>(Fixture::DecLineDrawing);
}
#[test]
fn resize_reflow() {
    contract::assert_resize_reflow::<SidecarMirror>();
}
