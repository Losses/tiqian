#![cfg(test)]

use crate::org::tiqian::protocol::plan::Plan;
use crate::org::tiqian::protocol::plan::PlanCell;
use crate::org::tiqian::protocol::plan::PlanLine;
use crate::org::tiqian::protocol::plan_end_reason::PlanEndReason;
use crate::org::tiqian::protocol::plan_json::PlanJson;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault) -> Self {
        match value {
            PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault) -> Self {
        match value {
            PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PlanJsonTestShapingBoundaryAndFeaturesEmitOnlyWhenSetFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlanJsonTestPlainPlanEncodesToTheWireShapeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}

impl From<PlanJsonTestPlainPlanEncodesToTheWireShapeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PlanJsonTestPlainPlanEncodesToTheWireShapeFault) -> Self {
        match value {
            PlanJsonTestPlainPlanEncodesToTheWireShapeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PlanJsonTestPlainPlanEncodesToTheWireShapeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PlanJsonTestPlainPlanEncodesToTheWireShapeFault) -> Self {
        match value {
            PlanJsonTestPlainPlanEncodesToTheWireShapeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PlanJsonTestPlainPlanEncodesToTheWireShapeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PlanJsonTestPlainPlanEncodesToTheWireShapeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PlanJsonTestPlainPlanEncodesToTheWireShapeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PlanJsonTestPlainPlanEncodesToTheWireShapeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn plain_plan_encodes_to_the_wire_shape() {
    testlib::run("org.tiqian.protocol.PlanJsonTest.plainPlanEncodesToTheWireShape", "org.tiqian.protocol.PlanJsonTest.plainPlanEncodesToTheWireShape", || {
        let plan = Plan { width: 300.0f64, height: 20.0f64, lines: vec![
    (PlanLine { range_start: 0, range_end: 2, top: 0.0f64, bottom: 20.0f64, baseline: 18.0f64, indent: 0.0f64, visual_width: 24.0f64, hyphen_advance: 0.0f64, end_reason: PlanEndReason::ParagraphEnd, cells: vec![
    (PlanCell { range_start: 0, range_end: 1, source: "你".to_string(), display: "你".to_string(), draw_x: 0.0f64, natural_width: 12.0f64, leading_layout_advance: 12.0f64, shaping_boundary: false, open_type_features: vec![], render_font_family: None.clone(), dash_strategy:
None.clone(), shaping_language: None.clone(), resolved_face: None.clone(), glyph_ids: None.clone(), shaping_evidence: None.clone(), punctuation_ink_floor: None, punctuation_body_width: None, latin: false, advance: None, inline_object: None, style_delta: None }).clone(),
    (PlanCell { range_start: 1, range_end: 2, source: "好".to_string(), display: "好".to_string(), draw_x: 12.0f64, natural_width: 12.0f64, leading_layout_advance: 12.0f64, shaping_boundary: false, open_type_features: vec![], render_font_family: None.clone(), dash_strategy:
None.clone(), shaping_language: None.clone(), resolved_face: None.clone(), glyph_ids: None.clone(), shaping_evidence: None.clone(), punctuation_ink_floor: None, punctuation_body_width: None, latin: false, advance: None, inline_object: None, style_delta: None }).clone(),
] }).clone(),
], emphasis_ranges: vec![], inline_edges: vec![], ruby_decisions: vec![], bopomofo_decisions: vec![], font_size: None, overlay_width: None, decoration_segments: vec![], emphasis_dots: vec![] };
        let json = PlanJson::plan_json_encode((plan).clone()).unwrap();
        let expected =
"{\"schema\":1,\"layoutRevision\":\"tiqian-layout-v2\",\"width\":300,\"height\":20,\"lines\":[{\"rangeStart\":0,\"rangeEnd\":2,\"top\":0,\"bottom\":20,\"baseline\":18,\"indent\":0,\"visualWidth\":24,\"hyphenAdvance\":0,\"endReason\":\"ParagraphEnd\",\"cells\":[{\"rangeStart\":0,\"rangeEnd\":1,\"source\":\"你\",\"display\":\"你\",\"drawX\":0,\"naturalWidth\":12,\"leadingLayoutAdvance\":12},{\"rangeStart\":1,\"rangeEnd\":2,\"source\":\"好\",\"display\":\"好\",\"drawX\":12,\"naturalWidth\":12,\"leadingLayoutAdvance\":12}]}]}".to_string();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected.as_str(), json.as_str(), Some("plain plan wire JSON".to_string())).unwrap();
    });
}

#[test]
fn shaping_boundary_and_features_emit_only_when_set() {
    testlib::run("org.tiqian.protocol.PlanJsonTest.shapingBoundaryAndFeaturesEmitOnlyWhenSet", "org.tiqian.protocol.PlanJsonTest.shapingBoundaryAndFeaturesEmitOnlyWhenSet", || {
        let plan = Plan { width: 100.0f64, height: 20.0f64, lines: vec![
    (PlanLine { range_start: 0, range_end: 2, top: 0.0f64, bottom: 20.0f64, baseline: 18.0f64, indent: 0.0f64, visual_width: 12.0f64, hyphen_advance: 0.0f64, end_reason: PlanEndReason::AutoWrap, cells: vec![
    (PlanCell { range_start: 0, range_end: 2, source: "A".to_string(), display: "A".to_string(), draw_x: 0.0f64, natural_width: 12.0f64, leading_layout_advance: 12.0f64, shaping_boundary: true, open_type_features: vec!["kern".to_string(), "liga".to_string()], render_font_family:
None.clone(), dash_strategy: None.clone(), shaping_language: None.clone(), resolved_face: None.clone(), glyph_ids: None.clone(), shaping_evidence: None.clone(), punctuation_ink_floor: None, punctuation_body_width: None, latin: false, advance: None, inline_object: None,
style_delta: None }).clone(),
] }).clone(),
], emphasis_ranges: vec![], inline_edges: vec![], ruby_decisions: vec![], bopomofo_decisions: vec![], font_size: None, overlay_width: None, decoration_segments: vec![], emphasis_dots: vec![] };
        let json = PlanJson::plan_json_encode((plan).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"shapingBoundary\":true", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes((u_string::find_from(&json, "\"openTypeFeatures\":[\"kern\",\"liga\"]", 0)).to_ne_bytes())) <= 2147483647, Some((json).to_string())).unwrap();
    });
}
