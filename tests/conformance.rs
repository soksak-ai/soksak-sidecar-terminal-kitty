mod common;

use common::ProviderMirror;
use soksak_contract_terminal as contract;
use soksak_contract_terminal::Fixture;

#[test]
fn mid_escape_tail() {
    contract::assert_conforms::<ProviderMirror>(Fixture::MidEscapeTail);
}
#[test]
fn cjk_width() {
    contract::assert_conforms::<ProviderMirror>(Fixture::CjkWidth);
}
#[test]
fn alt_screen() {
    contract::assert_conforms::<ProviderMirror>(Fixture::AltScreen);
}
#[test]
fn private_modes() {
    contract::assert_conforms::<ProviderMirror>(Fixture::PrivateModes);
}
#[test]
fn replay_guard() {
    contract::assert_conforms::<ProviderMirror>(Fixture::ReplayGuard);
}
#[test]
fn cold_paint_alt() {
    contract::assert_conforms::<ProviderMirror>(Fixture::ColdPaintAlt);
}
#[test]
fn dec_line_drawing() {
    contract::assert_conforms::<ProviderMirror>(Fixture::DecLineDrawing);
}
#[test]
fn resize_reflow() {
    contract::assert_resize_reflow::<ProviderMirror>();
}
