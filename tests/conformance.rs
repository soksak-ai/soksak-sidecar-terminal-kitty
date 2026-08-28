mod common;

use common::SidecarMirror;
use soksak_contract_terminal as contract;
use soksak_contract_terminal::Fixture;
use soksak_kit_sidecar_terminal::frame::{FrameBaseline, delta};
use soksak_sidecar_terminal_kitty::Mirror;

#[test]
fn cursor_style() {
    contract::assert_cursor_style_conforms::<SidecarMirror>();
}

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

// frame 와이어 시리즈 — 스트림을 셋으로 잘라 먹이며 subscriber 하나가 받는 세 reply. kit 의 delta 가
// 만들고 계약의 apply 가 접는다; 접은 결과는 해석을 채점하는 그 reference state 와 같아야 한다.
fn frame_series(fixture: Fixture) -> contract::frame::FrameSeries {
    let stream = fixture.stream();
    let cuts = contract::frame::cut_points(stream.len());
    let mut mirror = Mirror::new(contract::COLS, contract::ROWS);
    let mut baseline: Option<FrameBaseline> = None;
    let mut fed = 0;
    let mut replies = Vec::new();
    for cut in cuts {
        mirror.feed(&stream[fed..cut]);
        fed = cut;
        let frame = mirror.frame_at(0);
        let (reply, next) = delta(baseline.as_ref(), &frame, cut as u64);
        baseline = Some(next);
        let wire = serde_json::to_value(&reply).expect("frame reply serializes");
        replies.push(serde_json::from_value(wire).expect("kit reply parses as the contract wire"));
    }
    contract::frame::FrameSeries {
        fixture: fixture.stem().to_string(),
        cols: contract::COLS,
        rows: contract::ROWS,
        cuts: cuts.to_vec(),
        replies,
    }
}

#[test]
fn frame_delta_reproduces_reference_states() {
    for fixture in Fixture::ALL {
        contract::frame::assert_series_reproduces(&frame_series(fixture), fixture);
    }
}

// 시리즈 부트스트랩 — 계약의 reference_states/frames/<stem>.frames.json 후보를 뱉는다(#[ignore]).
//   SOKSAK_FRAME_SERIES_OUT=<dir> cargo test --release --test conformance -- --ignored dump_frame_series
#[test]
#[ignore]
fn dump_frame_series() {
    let dir = std::env::var("SOKSAK_FRAME_SERIES_OUT")
        .expect("SOKSAK_FRAME_SERIES_OUT=<dir> 로 산출 경로를 준다");
    let dir = std::path::PathBuf::from(dir);
    std::fs::create_dir_all(&dir).expect("mkdir");
    for fixture in Fixture::ALL {
        let path = dir.join(format!("{}.frames.json", fixture.stem()));
        std::fs::write(&path, frame_series(fixture).to_json()).expect("write frame series");
        println!("wrote {}", path.display());
    }
}
