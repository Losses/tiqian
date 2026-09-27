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
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsAssertEqualsIntFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault),
    TracedAssertionsAssertEqualsStringFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault),
    TracedAssertionsAssertEqualsFloatToleranceFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault),
    TracedAssertionsAssertTrueFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertEqualsIntFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertEqualsStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertTrueFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault) -> Self {
        match value {
            OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsIntFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertEqualsIntFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsStringFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertEqualsStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertEqualsFloatToleranceFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertEqualsFloatToleranceFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertTrueFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsAssertTrueFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        OpeningBracketLineStartTestTestOpeningBracketAtLineStartCompressionFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn test_opening_bracket_at_line_start_compression() {
    testlib::run("org.tiqian.layout.OpeningBracketLineStartTest.testOpeningBracketAtLineStartCompression", "org.tiqian.layout.OpeningBracketLineStartTest.testOpeningBracketAtLineStartCompression", || {
        let mut t = TestTraceRecorder::new("OpeningBracketLineStartTest");
        t.section(&"testOpeningBracketAtLineStartCompression");
        let text = concat!("这是第一行测试文字这是第一行测试\n",
"（Shaping & Font Metrics）这是第二行文字\n",
"（GPOS / GSUB 特性表查询）这是第三行文字").to_string();
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(true), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(672.0f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line1 = (result.clusters[usize::try_from((result.lines[1usize]).clone().cluster_range.start).unwrap_or(0)]).clone();
        let line2 = (result.clusters[usize::try_from((result.lines[2usize]).clone().cluster_range.start).unwrap_or(0)]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"（", (line1.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8.0f64, line1.advance, 0.01f64, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"（", (line2.text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float_tolerance(8.0f64, line2.advance, 0.01f64, None).unwrap();
        let mut count = 0u32;
        let mut all = true;
        for i in 0..match u32::try_from((result.debug).clone().line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let d = ((result.debug).clone().line_edge_trim_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            if d.reason.to_string() == "LineStartHalfWidthPunctuation" {
                count = u32::wrapping_add(count, 1);
                if !((d.side).to_string() == "leading" && d.trim_amount == 8.0f64) {
                    all = false;
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(all, None).unwrap();
    });
}
