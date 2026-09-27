#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault) -> Self {
        match value {
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault) -> Self {
        match value {
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault) -> Self {
        match value {
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault) -> Self {
        match value {
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault) -> Self {
        match value {
            PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        PunctuationBodyFloorInvariantTestPunctuationNeverResolvesBelowItsBodyWidthFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn punctuation_never_resolves_below_its_body_width() {
    testlib::run("org.tiqian.layout.PunctuationBodyFloorInvariantTest.punctuationNeverResolvesBelowItsBodyWidth", "org.tiqian.layout.PunctuationBodyFloorInvariantTest.punctuationNeverResolvesBelowItsBodyWidth", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[80,117,110,99,116,117,97,116,105,111,110,66,111,100,121,70,108,111,111,114,73,110,118,97,114,105,97,110,116,84,101,115,116])));
        t.section(UStr::new(&[112,117,110,99,116,117,97,116,105,111,110,78,101,118,101,114,82,101,115,111,108,118,101,115,66,101,108,111,119,73,116,115,66,111,100,121,87,105,100,116,104]));
        let fixtures = vec![
    UString::from("中文，中文。").to_ustring(),
    UString::from("他说：“你好，世界。”！！").to_ustring(),
    UString::from("中（中文）文中文中文中").to_ustring(),
    UString::from("有人说：「先有咖啡馆，后有启蒙运动」。每座城市、每条街巷、每个清晨都有人在等一杯 espresso……这并不是巧合。").to_ustring(),
    UString::from("读报、辩论、下棋、写作——城市生活忽然多出一个公共客厅。").to_ustring(),
];
        let widths = vec![48.0f64, 64.0f64, 80.0f64, 100.0f64, 160.0f64, 320.0f64];
        let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback"))).unwrap())), Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()), Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Arc::new(Mutex::new(ExplainableStubTextShaper::new()))), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).unwrap();
        for text in &fixtures {
            {
                let mut _g = 0u32;
                while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((widths.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                    let max_width = widths[usize::try_from(_g).unwrap_or(0)];
                    _g = u32::wrapping_add(_g, 1);
                    let result = engine.layout(LayoutInput::new(TiqianTextContent::new(text.as_ustr(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(max_width, Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
                    for i in 0..match u32::try_from((result.debug).clone().geometry_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let geometry = ((result.debug).clone().geometry_decisions[usize::try_from(i).unwrap_or(0)]).clone();
                        let _ = TracedAssertions::traced_assertions_assert_true((geometry.resolved_advance) >= geometry.body_width - 1e-3f64, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Body floor violated for '")); __s += (geometry.source_text).to_ustring().as_ustr(); __s += &(UString::from("' (")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((geometry.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((geometry.range).clone().end)).as_str())); __s += &(UString::from(") in \"")); __s += text.as_ustr(); __s += &(UString::from("\" @maxWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(max_width)); __s += &(UString::from(": resolved=")); __s += TestTraceRender::test_trace_render_float_text(geometry.resolved_advance).unwrap().as_ustr(); __s += &(UString::from(" < body=")); __s += TestTraceRender::test_trace_render_float_text(geometry.body_width).unwrap().as_ustr(); __s }).as_str()))).unwrap();
                    }
                }
            }
        }
    });
}
