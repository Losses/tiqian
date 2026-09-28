#![cfg(test)]

use crate::org::tiqian::protocol::plan::Plan;
use crate::org::tiqian::protocol::plan::PlanCell;
use crate::org::tiqian::protocol::plan::PlanEmphasisRange;
use crate::org::tiqian::protocol::plan::PlanInlineEdge;
use crate::org::tiqian::protocol::plan::PlanLine;
use crate::org::tiqian::protocol::plan_end_reason::PlanEndReason;
use crate::org::tiqian::protocol::plan_packed::PlanPacked;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UString;


#[test]
fn round_trip_preserves_plan_structure() {
    testlib::run("org.tiqian.protocol.PlanPackedTest.roundTripPreservesPlanStructure", "org.tiqian.protocol.PlanPackedTest.roundTripPreservesPlanStructure", || {
        let inline_edges = vec![PlanInlineEdge { offset: 10, inline_start: Some(4.0f64), inline_end: None }];
        let plan = Plan { width: 300.0f64, height: 20.0f64, lines: vec![
    (PlanLine { range_start: 0, range_end: 2, top: 0.0f64, bottom: 20.0f64, baseline: 18.0f64, indent: 0.0f64, visual_width: 24.0f64, hyphen_advance: 0.0f64, end_reason: PlanEndReason::ParagraphEnd, cells: vec![
    (PlanCell { range_start: 0, range_end: 1, source: UString::from("你").to_ustring(), display: UString::from("你").to_ustring(), draw_x: 0.0f64, natural_width: 12.0f64, leading_layout_advance: 12.0f64, shaping_boundary: false, open_type_features: vec![], render_font_family: None.clone(), dash_strategy: None.clone(), shaping_language: None.clone(), resolved_face: None.clone(), glyph_ids: None.clone(), shaping_evidence: None.clone(), punctuation_ink_floor: None, punctuation_body_width: None, latin: false, advance: None, inline_object: None, style_delta: None }).clone(),
] }).clone(),
], emphasis_ranges: vec![PlanEmphasisRange { start: 0, end: 1 }], inline_edges: inline_edges, ruby_decisions: vec![], bopomofo_decisions: vec![], font_size: None, overlay_width: None, decoration_segments: vec![], emphasis_dots: vec![] };
        let bytes = PlanPacked::plan_packed_encode((plan).clone());
        let decoded = PlanPacked::plan_packed_decode(&bytes);
        let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((plan.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from(((decoded.as_ref().unwrap().lines).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), Some(UString::from("line count"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from(((plan.lines[0usize]).clone().cells.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from(((((decoded.as_ref().unwrap().lines).clone()[0usize]).clone().cells).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), Some(UString::from("cell count"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string((((plan.lines[0usize]).clone().cells[0usize]).clone().source).to_ustring().as_ustr(), (((((decoded.as_ref().unwrap().lines).clone()[0usize]).clone().cells).clone()[0usize]).clone().source).to_ustring().as_ustr(), Some(UString::from("cell source"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string((((plan.lines[0usize]).clone().cells[0usize]).clone().display).to_ustring().as_ustr(), (((((decoded.as_ref().unwrap().lines).clone()[0usize]).clone().cells).clone()[0usize]).clone().display).to_ustring().as_ustr(), Some(UString::from("cell display"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((plan.emphasis_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from(((decoded.as_ref().unwrap().emphasis_ranges).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), Some(UString::from("emphasis count"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals(u32::try_from((plan.inline_edges.len()) & 0xFFFF_FFFF).unwrap_or(0), u32::try_from(((decoded.as_ref().unwrap().inline_edges).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), Some(UString::from("inline edge count"))).unwrap();
        if i32::from_ne_bytes(((u32::try_from(((decoded.as_ref().unwrap().inline_edges).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            let _ = TracedAssertions::traced_assertions_assert_equals(plan.inline_edges[0usize].offset, (decoded.as_ref().unwrap().inline_edges).clone()[0usize].offset, Some(UString::from("inline offset"))).unwrap();
        }
    });
}
