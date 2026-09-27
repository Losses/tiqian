#![cfg(test)]

use crate::org::tiqian::clreq::clreq_profile_resolver::BuiltInClreqProfileResolver;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::default_hyphenator::DefaultHyphenator;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::line_break_repair_engine_test_support::LineBreakRepairEngineTestSupport;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;


#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestOrdinaryNumericFormsDoNotBecomeBibliographicLocatorsFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestNumberWithSuffixSymbolNeverSplitsAcrossLinesFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
}

impl From<KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLongLatinSentenceWrapsAtWordBoundariesFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestLineEndKinsokuMovesDanglingOpenerToNextLineFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrinkFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelStrictForbidsDashAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLevelNoneLeavesForbiddenMarksAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStartFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareAClusterFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestKinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuationFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestHangingPunctuationFillsLineToMeasureAndOverflowsVisualFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault) -> Self {
        match value {
            KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        KinsokuAndCohesionRepairEngineTestBibliographicNumericLocatorExposesStructuralBreaksFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[test]
fn bibliographic_numeric_locator_exposes_structural_breaks() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.bibliographicNumericLocatorExposesStructuralBreaks", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.bibliographicNumericLocatorExposesStructuralBreaks", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"bibliographicNumericLocatorExposesStructuralBreaks");
        let text = "中文中文中文44(10):21-38.".to_string();
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(224 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let decision = ((result.debug).clone().break_opportunity_decisions[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(TextRange::new(6u32, 19u32).unwrap().to_string().as_str(), (decision.range).clone().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"44(10):21-38.", (decision.source_text).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int_array(&vec![8, 13], &decision.break_offsets, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"BibliographicNumericLocatorBreak", (decision.reason).to_string().as_str(), None).unwrap();
        let mut line_texts: Vec<String> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_texts.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let _ = TracedAssertions::traced_assertions_assert_true(((line_texts[0usize]).clone()).ends_with(&"44(10):"), Some((format!("{}{}",
            "locator should fill the preceding line: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"21-38.", (line_texts[usize::try_from(u32::wrapping_sub(u32::try_from((line_texts.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone().as_str(), None).unwrap();
        let mut no_end_open = true;
        for i in 0..match u32::try_from(line_texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if line_texts[usize::try_from(i).unwrap_or(0)].clone().ends_with(&"(") {
                no_end_open = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no_end_open, Some((format!("{}{}",
            "opening bracket cannot end a line: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)
        )).to_string())).unwrap();
        let mut no_start_close = true;
        for i in 0..match u32::try_from(line_texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if line_texts[usize::try_from(i).unwrap_or(0)].clone().starts_with(&")") {
                no_start_close = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(no_start_close, Some((format!("{}{}",
            "closing bracket cannot start a line: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)
        )).to_string())).unwrap();
    });
}

#[test]
fn hanging_punctuation_fills_line_to_measure_and_overflows_visual() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.hangingPunctuationFillsLineToMeasureAndOverflowsVisual", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.hangingPunctuationFillsLineToMeasureAndOverflowsVisual", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"hangingPunctuationFillsLineToMeasureAndOverflowsVisual");
        let mut engine = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(Some(KinsokuLevel::Basic), Some(HangingPunctuationStyle::PauseStops)).unwrap();
        let result = engine.layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中文中文，中文。", 64 as f64, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) >= 2, None).unwrap();
        let line0 = (result.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, (line0.range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, (line0.range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, line0.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((line0.visual_width) > (64 as f64), Some((format!("{}{}",
            "hung mark must overflow: ",
            line0.visual_width
        )).to_string())).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(line0.visual_width - line0.adjusted_width, line0.hanging_punctuation_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Hang", (((result.debug).clone().line_decisions[0usize]).clone().repair).as_deref().unwrap_or(""), None).unwrap();
        let plain = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(Some(KinsokuLevel::Basic), Some(HangingPunctuationStyle::Disabled)).unwrap().layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中文中文，中文。", 64 as
f64, None).unwrap()).unwrap();
        let mut none_overflow = true;
        for i in 0..match u32::try_from(plain.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if plain.lines[usize::try_from(i).unwrap_or(0)].visual_width > (64 as f64) {
                none_overflow = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_overflow, None).unwrap();
        let mut none_hang = true;
        for i in 0..match u32::try_from((plain.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if plain.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.as_ref().map_or(false, |v| v == &("Hang".to_string())) {
                none_hang = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_hang, None).unwrap();
    });
}

#[test]
fn kinsoku_carries_previous_cluster_when_line_would_start_with_forbidden_punctuation() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuation", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuation", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"kinsokuCarriesPreviousClusterWhenLineWouldStartWithForbiddenPunctuation");
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(None, None).unwrap().layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中文中文。", 64 as f64, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, ((result.lines[0usize]).clone().range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ((result.lines[0usize]).clone().range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, ((result.lines[1usize]).clone().range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, ((result.lines[1usize]).clone().range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(48 as f64, result.lines[0usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24 as f64, result.lines[1usize].adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), ((result.debug).clone().line_decisions[0usize]).clone().repair.clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CarryPrevious", (((result.debug).clone().line_decisions[1usize]).clone().repair).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(10, (result.debug).clone().line_decisions[1usize].repair_penalty, None).unwrap();
        let repair_decision = ((result.debug).clone().line_decisions[1usize]).clone().repair_decision;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CarryPrevious", (repair_decision.as_ref().unwrap().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ForbiddenAtLineStart", (repair_decision.as_ref().unwrap().reason_code).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, (repair_decision.as_ref().unwrap().offender_range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(5, (repair_decision.as_ref().unwrap().offender_range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, (repair_decision.as_ref().unwrap().carried_cluster_index).unwrap(), None).unwrap();
        let repair_candidates = (((result.debug).clone().line_decisions[1usize]).clone().repair_candidates).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", ((repair_candidates[0usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(false, repair_candidates[0usize].accepted, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"insufficient-capacity", ((repair_candidates[0usize]).clone().rejection_reason).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"CarryPrevious", ((repair_candidates[1usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(true, repair_candidates[1usize].accepted, None).unwrap();
        let mut notes_match = false;
        let notes = (((result.debug).clone().line_decisions[1usize]).clone().notes).clone();
        for i in 0..match u32::try_from(notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes((u_string::find_from(&(notes[usize::try_from(i).unwrap_or(0)]).clone(), "ForbiddenAtLineStart:。", 0)).to_ne_bytes()) <= 2147483647 && (u32::from_ne_bytes((u_string::find_from(&(notes[usize::try_from(i).unwrap_or(0)]).clone(), "carried=文",
0)).to_ne_bytes())) <= 2147483647 {
                notes_match = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(notes_match, None).unwrap();
    });
}

#[test]
fn kinsoku_falls_back_to_leave_ragged_when_previous_line_cannot_spare_a_cluster() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareACluster", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareACluster", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"kinsokuFallsBackToLeaveRaggedWhenPreviousLineCannotSpareACluster");
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(None, None).unwrap().layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"Coffee。", 96 as f64, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let mut coffee_text = String::new();
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "Coffee" {
                coffee_text = ((result.clusters[usize::try_from(i).unwrap_or(0)]).clone().text).to_string();
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"Coffee", coffee_text.as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"LeaveRagged", (((result.debug).clone().line_decisions[1usize]).clone().repair).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(20, (result.debug).clone().line_decisions[1usize].repair_penalty, None).unwrap();
        let mut notes_match = false;
        let notes = (((result.debug).clone().line_decisions[1usize]).clone().notes).clone();
        for i in 0..match u32::try_from(notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes((u_string::find_from(&(notes[usize::try_from(i).unwrap_or(0)]).clone(), "ForbiddenAtLineStart:。", 0)).to_ne_bytes()) <= 2147483647 && (u32::from_ne_bytes((u_string::find_from(&(notes[usize::try_from(i).unwrap_or(0)]).clone(), "no-room-to-carry",
0)).to_ne_bytes())) <= 2147483647 {
                notes_match = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(notes_match, None).unwrap();
    });
}

#[test]
fn kinsoku_leaves_greedy_break_alone_when_no_forbidden_punct_at_line_start() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStart", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStart", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"kinsokuLeavesGreedyBreakAloneWhenNoForbiddenPunctAtLineStart");
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(None, None).unwrap().layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中文中文哈哈", 64 as f64, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, ((result.lines[0usize]).clone().range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ((result.lines[0usize]).clone().range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, ((result.lines[1usize]).clone().range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(6, ((result.lines[1usize]).clone().range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), ((result.debug).clone().line_decisions[0usize]).clone().repair.clone(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_nullable_string(None.clone(), ((result.debug).clone().line_decisions[1usize]).clone().repair.clone(), None).unwrap();
    });
}

#[test]
fn kinsoku_level_none_leaves_forbidden_marks_at_line_start() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuLevelNoneLeavesForbiddenMarksAtLineStart", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuLevelNoneLeavesForbiddenMarksAtLineStart", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"kinsokuLevelNoneLeavesForbiddenMarksAtLineStart");
        let input = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中文中。中", 48 as f64, None).unwrap();
        let none = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(Some(KinsokuLevel::None), Some(HangingPunctuationStyle::Disabled)).unwrap().layout((input).clone()).unwrap();
        let mut all_null = true;
        for i in 0..match u32::try_from((none.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if none.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.is_some() {
                all_null = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_null, None).unwrap();
        let mut any_start3 = false;
        for i in 0..match u32::try_from(none.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if none.lines[usize::try_from(i).unwrap_or(0)].clone().range.clone().start == 3 {
                any_start3 = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_start3, None).unwrap();
        let basic = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(Some(KinsokuLevel::Basic), Some(HangingPunctuationStyle::Disabled)).unwrap().layout((input).clone()).unwrap();
        let mut any_repair = false;
        for i in 0..match u32::try_from((basic.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if basic.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.is_some() {
                any_repair = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_repair, None).unwrap();
    });
}

#[test]
fn kinsoku_level_strict_forbids_dash_at_line_start() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuLevelStrictForbidsDashAtLineStart", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuLevelStrictForbidsDashAtLineStart", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"kinsokuLevelStrictForbidsDashAtLineStart");
        let input = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中文中——文", 48 as f64, None).unwrap();
        let basic = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(Some(KinsokuLevel::Basic), Some(HangingPunctuationStyle::Disabled)).unwrap().layout((input).clone()).unwrap();
        let mut all_null = true;
        for i in 0..match u32::try_from((basic.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if basic.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.is_some() {
                all_null = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(all_null, None).unwrap();
        let strict = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(Some(KinsokuLevel::Strict), Some(HangingPunctuationStyle::Disabled)).unwrap().layout((input).clone()).unwrap();
        let mut any_repair = false;
        for i in 0..match u32::try_from((strict.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if strict.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.is_some() {
                any_repair = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_repair, None).unwrap();
    });
}

#[test]
fn kinsoku_pushes_line_start_punctuation_into_previous_line_when_trailing_glue_can_shrink() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrink", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.kinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrink",
|| {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"kinsokuPushesLineStartPunctuationIntoPreviousLineWhenTrailingGlueCanShrink");
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new("中文中。", Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(60 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let line = (result.lines[0usize]).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, (line.range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, (line.range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(64 as f64, line.natural_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(56 as f64, line.adjusted_width, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(56 as f64, line.visual_width, None).unwrap();
        let mut c_sum = 0.0f64;
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            c_sum += result.clusters[usize::try_from(i).unwrap_or(0)].advance;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(56 as f64, c_sum, None).unwrap();
        let mut g_sum = 0.0f64;
        for i in 0..match u32::try_from(result.glyph_runs.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            g_sum += result.glyph_runs[usize::try_from(i).unwrap_or(0)].advance;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(56 as f64, g_sum, None).unwrap();
        let mut stop_advance = 0.0f64;
        for i in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.clusters[usize::try_from(i).unwrap_or(0)].clone().text.to_string() == "。" {
                stop_advance = result.clusters[usize::try_from(i).unwrap_or(0)].advance;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, stop_advance, None).unwrap();
        let mut trailing_glue_consumed = 0.0f64;
        let mut resolved_advance = 0.0f64;
        for i in 0..match u32::try_from((result.debug).clone().geometry_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().geometry_decisions[usize::try_from(i).unwrap_or(0)].clone().source_text.to_string() == "。" {
                trailing_glue_consumed = (result.debug).clone().geometry_decisions[usize::try_from(i).unwrap_or(0)].trailing_glue_consumed;
                resolved_advance = (result.debug).clone().geometry_decisions[usize::try_from(i).unwrap_or(0)].resolved_advance;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, trailing_glue_consumed, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, resolved_advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((result.debug).clone().line_edge_trim_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", (((result.debug).clone().line_decisions[0usize]).clone().repair).as_deref().unwrap_or(""), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, (result.debug).clone().line_decisions[0usize].repair_penalty, None).unwrap();
        let repair_decision = ((result.debug).clone().line_decisions[0usize]).clone().repair_decision;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", (repair_decision.as_ref().unwrap().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"ForbiddenAtLineStart", (repair_decision.as_ref().unwrap().reason_code).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, (repair_decision.as_ref().unwrap().offender_range).clone().start, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, (repair_decision.as_ref().unwrap().offender_range).clone().end, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, (repair_decision.as_ref().unwrap().target_cluster_index).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, repair_decision.as_ref().unwrap().shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, repair_decision.as_ref().unwrap().available_capacity, None).unwrap();
        let repair_candidates = (((result.debug).clone().line_decisions[0usize]).clone().repair_candidates).clone();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(&"PushIn", ((repair_candidates[0usize]).clone().kind).to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bool(true, repair_candidates[0usize].accepted, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(4 as f64, repair_candidates[0usize].required_shrink, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8 as f64, repair_candidates[0usize].available_capacity, None).unwrap();
        let mut notes_match = false;
        let notes = (((result.debug).clone().line_decisions[0usize]).clone().notes).clone();
        for i in 0..match u32::try_from(notes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes((u_string::find_from(&(notes[usize::try_from(i).unwrap_or(0)]).clone(), "ForbiddenAtLineStart:。", 0)).to_ne_bytes()) <= 2147483647 && (u32::from_ne_bytes((u_string::find_from(&(notes[usize::try_from(i).unwrap_or(0)]).clone(), "pushed-in=4.0",
0)).to_ne_bytes())) <= 2147483647 {
                notes_match = true;
                break;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(notes_match, None).unwrap();
    });
}

#[test]
fn line_end_kinsoku_moves_dangling_opener_to_next_line() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.lineEndKinsokuMovesDanglingOpenerToNextLine", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.lineEndKinsokuMovesDanglingOpenerToNextLine", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"lineEndKinsokuMovesDanglingOpenerToNextLine");
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_fixed(None, None).unwrap().layout(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_input(&"中中中（中中）中", 64 as f64, None).unwrap()).unwrap();
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (result.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let mut last_cluster: Option<Cluster> = None;
            for j in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if i32::from_ne_bytes((((result.clusters[usize::try_from(j).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes()) <= i32::from_ne_bytes(((line.range).clone().end).to_ne_bytes()) {
                    last_cluster = Some((result.clusters[usize::try_from(j).unwrap_or(0)]).clone());
                }
            }
            let _ = TracedAssertions::traced_assertions_assert_true((last_cluster.as_ref().unwrap().text).to_string() != "（", Some((format!("{}{}",
            "line must not end on 开括号: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&result.clusters)
        )).to_string())).unwrap();
        }
        let mut any_carry_next = false;
        for i in 0..match u32::try_from((result.debug).clone().line_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if result.debug.clone().line_decisions[usize::try_from(i).unwrap_or(0)].clone().repair.as_ref().map_or(false, |v| v == &("CarryNext".to_string())) {
                any_carry_next = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any_carry_next, None).unwrap();
    });
}

#[test]
fn long_latin_sentence_wraps_at_word_boundaries() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.longLatinSentenceWrapsAtWordBoundaries", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.longLatinSentenceWrapsAtWordBoundaries", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"longLatinSentenceWrapsAtWordBoundaries");
        let result = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_layout(&"The quick brown fox", 160 as f64, None, None, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((i32::from_ne_bytes((u32::try_from((result.lines.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) > (1), Some("long Latin must wrap at word boundaries".to_string())).unwrap();
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (result.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let mut line_clusters: Vec<Cluster> = vec![];
            for j in 0..match u32::try_from(result.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if i32::from_ne_bytes((((result.clusters[usize::try_from(j).unwrap_or(0)]).clone().range).clone().start).to_ne_bytes()) >= i32::from_ne_bytes(((line.range).clone().start).to_ne_bytes()) &&
(i32::from_ne_bytes((((result.clusters[usize::try_from(j).unwrap_or(0)]).clone().range).clone().end).to_ne_bytes())) <= i32::from_ne_bytes(((line.range).clone().end).to_ne_bytes()) {
                    line_clusters.push((result.clusters[usize::try_from(j).unwrap_or(0)]).clone());
                }
            }
            let first = (line_clusters[0usize]).clone();
            let last = (line_clusters[usize::try_from(u32::wrapping_sub(u32::try_from((line_clusters.len()) & 0xFFFF_FFFF).unwrap_or(0), 1)).unwrap_or(0)]).clone();
            let mut first_all_space = i32::from_ne_bytes((u_string::unit_count(&((first.text).to_string()))).to_ne_bytes()) > (0);
            for j in 0..match u32::try_from(u_string::unit_count(&((first.text).to_string()))) { Ok(value) => value, Err(_) => u32::MAX } {
                if u_string::substring(&(first.text).to_string(), { let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }, i32::wrapping_add({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }, 1)) != " " {
                    first_all_space = false;
                }
            }
            if first_all_space {
                let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, first.advance, None).unwrap();
            }
            let mut last_all_space = i32::from_ne_bytes((u_string::unit_count(&((last.text).to_string()))).to_ne_bytes()) > (0);
            for j in 0..match u32::try_from(u_string::unit_count(&((last.text).to_string()))) { Ok(value) => value, Err(_) => u32::MAX } {
                if u_string::substring(&(last.text).to_string(), { let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }, i32::wrapping_add({ let v: u32 = j; i32::from_ne_bytes(v.to_ne_bytes()) }, 1)) != " " {
                    last_all_space = false;
                }
            }
            if last_all_space {
                let _ = TracedAssertions::traced_assertions_assert_equals_float(0 as f64, last.advance, None).unwrap();
            }
        }
    });
}

#[test]
fn number_with_suffix_symbol_never_splits_across_lines() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.numberWithSuffixSymbolNeverSplitsAcrossLines", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.numberWithSuffixSymbolNeverSplitsAcrossLines", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"numberWithSuffixSymbolNeverSplitsAcrossLines");
        let text = "销量增长了50%呢".to_string();
        let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(text.as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400),
Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(120 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
        let mut line_texts: Vec<String> = vec![];
        for i in 0..match u32::try_from(result.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            line_texts.push(LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_line_text((result).clone(), i));
        }
        let mut any50 = false;
        for i in 0..match u32::try_from(line_texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if u32::from_ne_bytes((u_string::find_from(&(line_texts[usize::try_from(i).unwrap_or(0)]).clone(), "50%", 0)).to_ne_bytes()) <= 2147483647 {
                any50 = true;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(any50, Some((format!("{}{}",
            "50% must stay together: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)
        )).to_string())).unwrap();
        let mut none_end50 = true;
        for i in 0..match u32::try_from(line_texts.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if line_texts[usize::try_from(i).unwrap_or(0)].clone().ends_with(&"50") {
                none_end50 = false;
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_true(none_end50, Some((format!("{}{}",
            "no line may end mid-number: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_strings(&line_texts)
        )).to_string())).unwrap();
    });
}

#[test]
fn ordinary_numeric_forms_do_not_become_bibliographic_locators() {
    testlib::run("org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.ordinaryNumericFormsDoNotBecomeBibliographicLocators", "org.tiqian.layout.KinsokuAndCohesionRepairEngineTest.ordinaryNumericFormsDoNotBecomeBibliographicLocators", || {
        let _ = LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_kinsoku_start(&"ordinaryNumericFormsDoNotBecomeBibliographicLocators");
        let tokens = vec!["3.14".to_string(), "1,000".to_string(), "12:34".to_string(), "2023-08-11".to_string()];
        for token in &tokens {
            let result = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string())).unwrap())),
Some(Box::new(BuiltInClreqProfileResolver::new())), Some(Box::new(StubFontMetricsResolver::new())), Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some(PunctuationAtomBuilder::new(None, None).unwrap()), Some(PunctuationSpacingCompressor::new().unwrap()),
Some(QuotePairAnalyzer::new()), Some(Box::new(GreedyLineBreaker::new(None, None, None, None))), Some(Justifier::new(Some(0.5), Some(0.25))), Some(Box::new(ExplainableStubTextShaper::new())), Some(DefaultHyphenator::default_hyphenator_default_hyphenator().unwrap()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).unwrap().layout(LayoutInput::new(TiqianTextContent::new(format!("{}{}",
            "中文",
            token
        ).as_str(), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start),
Some(WritingMode::HorizontalTb), None, Some((Ic::zero()).clone()), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some(LineLengthGrid::new(Some(false), None)), Some(RubyLineHeightMode::PerLine),
Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some(vec![]), Some(vec![]), Some(vec![]), Some(vec![]))).unwrap();
            let _ = TracedAssertions::traced_assertions_assert_true(u32::try_from(((result.debug).clone().break_opportunity_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0, Some((format!("{}{}{}",
            token,
            " must keep its existing numeric/token policy: ",
            LineBreakRepairEngineTestSupport::line_break_repair_engine_test_support_render_list(&(result.debug).clone().break_opportunity_decisions)
        )).to_string())).unwrap();
        }
    });
}
