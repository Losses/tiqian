//! The fault path of the host bridge's infallible metrics resolver (ADR 0050
//! Slice C).
//!
//! The generated `FontMetricsResolver::resolve` has no error channel, so
//! `SessionBackend::resolve` parks a `resolve_metrics` failure in the shared
//! fault cell and `precompute_paragraph` returns that message once layout
//! finishes. The substituted stub metric only keeps the layout it must throw
//! away well-formed; a plan computed from it must never reach the caller.
//!
//! The fixture registers two real faces under one family with disjoint
//! `unicode-range` coverage and different vertical metrics. Shaping selects the
//! face that covers the paragraph text and succeeds. `resolve_metrics` then
//! compares every weight-matched face of that family and rejects the pair as
//! non-uniform. That is the only shape the mechanism exists for: a layout that
//! would otherwise succeed on a substituted metric.

use std::path::PathBuf;

use tiqian_precompute::engine_bridge;
use tiqian_precompute::font_record::{FontFaceSpec, FontWeightSpec};
use tiqian_precompute::paragraph::ParagraphRequest;
use tiqian_precompute::session::{
    create_font_session, CaptureEvidence, SessionFaceSpec, SessionOptions,
};

/// One family, two faces: the CJK face covers the paragraph text, the Latin
/// face supplies a conflicting metric set for the uniformity check.
const FAULT_FAMILY: &str = "Tq Metrics Fault Fixture";
const CJK_FACE_FILE: &str = "DelaGothicOne-Regular.ttf";
const LATIN_FACE_FILE: &str = "Barlow-Regular.ttf";
const CJK_RANGE: &str = "U+4E00-9FFF";
const LATIN_RANGE: &str = "U+0020-007E";

fn fixture_font(file: &str) -> PathBuf {
    let home = std::env::var("HOME").expect("HOME names the font fixture directory");
    let path = PathBuf::from(home).join(".local/share/fonts").join(file);
    // A missing fixture is a broken test, not a passing one: the fault path
    // cannot be exercised without a real face, so fail loudly.
    assert!(
        path.is_file(),
        "font fixture {path:?} is missing; this test must not skip"
    );
    path
}

/// 「中文文字排版段落」 stays inside the CJK face's declared range and its
/// coverage, so shaping has a real face to resolve.
fn request(session_id: &str) -> ParagraphRequest {
    ParagraphRequest {
        font_session_id: session_id.to_string(),
        text: "中文文字排版段落".to_string(),
        max_width_px: 144.0,
        font_families: vec![FAULT_FAMILY.to_string()],
        font_size_px: 18.0,
        line_height_px: 27.0,
        locale: "zh-Hans".to_string(),
        font_weight: 400,
        italic: false,
        first_line_indent_ic: 0.0,
        line_length_grid_enabled: true,
        source_boundaries: Vec::new(),
        text_spans: Vec::new(),
        line_break_spans: Vec::new(),
        inline_boxes: Vec::new(),
    }
}

fn face_specs<'a>(
    cjk_bytes: &'a [u8],
    latin_bytes: Option<&'a [u8]>,
) -> Vec<SessionFaceSpec<'a>> {
    let mut specs = vec![SessionFaceSpec {
        spec: FontFaceSpec {
            family: FAULT_FAMILY,
            public_url: "/fonts/DelaGothicOne-Regular.ttf",
            source: cjk_bytes,
            face_index: None,
            weight: FontWeightSpec::Single(Some(400.0)),
            style: "normal",
            unicode_range: Some(CJK_RANGE),
            source_order: 0,
        },
        source_order: Some(0.0),
    }];
    if let Some(latin_bytes) = latin_bytes {
        specs.push(SessionFaceSpec {
            spec: FontFaceSpec {
                family: FAULT_FAMILY,
                public_url: "/fonts/Barlow-Regular.ttf",
                source: latin_bytes,
                face_index: None,
                weight: FontWeightSpec::Single(Some(400.0)),
                style: "normal",
                unicode_range: Some(LATIN_RANGE),
                source_order: 1,
            },
            source_order: Some(1.0),
        });
    }
    specs
}

/// `resolve_metrics` rejects the pair because both faces are weight-matched
/// members of the selected family and their vertical metrics differ.
fn expected_metrics_message() -> String {
    format!("NonUniformUnicodeRangeMetrics:family={FAULT_FAMILY};weight=400")
}

#[test]
fn metrics_resolution_failure_is_reported_instead_of_a_stub_metric_plan() {
    let cjk_bytes = std::fs::read(fixture_font(CJK_FACE_FILE)).expect("CJK fixture reads");
    let latin_bytes = std::fs::read(fixture_font(LATIN_FACE_FILE)).expect("Latin fixture reads");
    let session = create_font_session(
        face_specs(&cjk_bytes, Some(&latin_bytes)),
        SessionOptions::default(),
    )
    .expect("conflicting-metrics session builds");
    let request = request(&session.session_id);
    let mut evidence = CaptureEvidence::new();

    let outcome = engine_bridge::precompute_paragraph(&session, &mut evidence, &request);
    // Shaping succeeds on the CJK face, so without the fault check this call
    // would return Ok with a plan laid out on the stub metric.
    let error = match outcome {
        Ok(plan) => panic!(
            "a plan built on the substituted stub metric escaped: {} lines",
            plan.lines.len()
        ),
        Err(message) => message,
    };
    assert_eq!(
        error,
        expected_metrics_message(),
        "the call must report the metrics resolution failure, not a shape or \
         layout error"
    );
    assert!(
        error.starts_with("NonUniformUnicodeRangeMetrics:"),
        "the error must carry the resolve_metrics message, got {error:?}"
    );
}

/// Control: the same request and the same CJK face lay out successfully when
/// the conflicting face is absent. The failing call above therefore fails
/// because of the metrics fault, not because the request cannot be laid out.
#[test]
fn the_same_layout_succeeds_when_the_metric_conflict_is_absent() {
    let cjk_bytes = std::fs::read(fixture_font(CJK_FACE_FILE)).expect("CJK fixture reads");
    let session = create_font_session(face_specs(&cjk_bytes, None), SessionOptions::default())
        .expect("single-face session builds");
    let request = request(&session.session_id);
    let mut evidence = CaptureEvidence::new();

    let plan = engine_bridge::precompute_paragraph(&session, &mut evidence, &request)
        .expect("the control layout succeeds");
    assert!(!plan.lines.is_empty(), "the control layout produced no lines");
}
