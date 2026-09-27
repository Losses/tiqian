#![cfg(test)]

use crate::org::tiqian::core::bopomofo_glyph_role::BopomofoGlyphRole;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_span::TextSpan;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::layout::bopomofo_layout_test_support::BopomofoLayoutTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum BopomofoLayoutTestSymbolsAndToneRightOfBaseFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for BopomofoLayoutTestSymbolsAndToneRightOfBaseFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<BopomofoLayoutTestSymbolsAndToneRightOfBaseFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BopomofoLayoutTestSymbolsAndToneRightOfBaseFault) -> Self {
        match value {
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestSymbolsAndToneRightOfBaseFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BopomofoLayoutTestSymbolsAndToneRightOfBaseFault) -> Self {
        match value {
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestSymbolsAndToneRightOfBaseFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BopomofoLayoutTestSymbolsAndToneRightOfBaseFault) -> Self {
        match value {
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestSymbolsAndToneRightOfBaseFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BopomofoLayoutTestSymbolsAndToneRightOfBaseFault) -> Self {
        match value {
            BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BopomofoLayoutTestSymbolsAndToneRightOfBaseFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BopomofoLayoutTestSymbolsAndToneRightOfBaseFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BopomofoLayoutTestSymbolsAndToneRightOfBaseFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BopomofoLayoutTestSymbolsAndToneRightOfBaseFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BopomofoLayoutTestSymbolsAndToneRightOfBaseFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault) -> Self {
        match value {
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault) -> Self {
        match value {
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault) -> Self {
        match value {
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault) -> Self {
        match value {
            BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BopomofoLayoutTestFontWeightFollowsAnnotatedBasePlusThreeStepsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault) -> Self {
        match value {
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault) -> Self {
        match value {
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault) -> Self {
        match value {
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault) -> Self {
        match value {
            BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BopomofoLayoutTestDecisionKeepsSourceReadingForCopyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BopomofoLayoutTestAnnotationLocaleDoesNotReplaceSimplifiedBaseLocaleFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault) -> Self {
        match value {
            BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        BopomofoLayoutTestAnnotatedBaseReservesHalfEmOnlyFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn symbols_and_tone_right_of_base() {
    testlib::run("org.tiqian.layout.BopomofoLayoutTest.symbolsAndToneRightOfBase", "org.tiqian.layout.BopomofoLayoutTest.symbolsAndToneRightOfBase", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[115,121,109,98,111,108,115,65,110,100,84,111,110,101,82,105,103,104,116,79,102,66,97,115,101]));
        let r = BopomofoLayoutTestSupport::bopomofo_layout_test_support_layout(&vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12563,12584,12581])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[12564,12580,714])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
], None).unwrap();
        let z = ((r.debug).clone().bopomofo_decisions).clone();
        if i32::from_ne_bytes(((u32::try_from((z.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (2) {
            let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((z.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
            return;
        }
        let a = (z[0usize]).clone();
        let b = (z[1usize]).clone();
        let mut symbols_a = 0u32;
        let mut tone_a = 0u32;
        let mut right = true;
        let mut lefts: Vec<UString> = vec![];
        for _g_index in 0..match u32::try_from(a.placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let p = (a.placements[usize::try_from(_g_index).unwrap_or(0)]).clone();
            if p.role == BopomofoGlyphRole::Symbol {
                symbols_a = u32::wrapping_add(symbols_a, 1);
            }
            if p.role == BopomofoGlyphRole::Tone {
                tone_a = u32::wrapping_add(tone_a, 1);
            }
            if p.left < (15.9f64) {
                right = false;
            }
            lefts.push(TestTraceRender::test_trace_render_render_float(p.left).unwrap());
        }
        let mut symbols_b = 0u32;
        let mut tone_b = 0u32;
        for _g_index1 in 0..match u32::try_from(b.placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let p = (b.placements[usize::try_from(_g_index1).unwrap_or(0)]).clone();
            if p.role == BopomofoGlyphRole::Symbol {
                symbols_b = u32::wrapping_add(symbols_b, 1);
            }
            if p.role == BopomofoGlyphRole::Tone {
                tone_b = u32::wrapping_add(tone_b, 1);
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((z.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(700, a.font_weight, Some(UString::from("bopomofo defaults three weight steps heavier than base"))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(3, symbols_a, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(tone_a == 0, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(right, Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("symbols right of base: [")); __s += UString::from(format!("{}", { let joined1 = lefts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, symbols_b, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, tone_b, None).unwrap();
    });
}

#[test]
fn annotated_base_reserves_half_em_only() {
    testlib::run("org.tiqian.layout.BopomofoLayoutTest.annotatedBaseReservesHalfEmOnly", "org.tiqian.layout.BopomofoLayoutTest.annotatedBaseReservesHalfEmOnly", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[97,110,110,111,116,97,116,101,100,66,97,115,101,82,101,115,101,114,118,101,115,72,97,108,102,69,109,79,110,108,121]));
        let plain = BopomofoLayoutTestSupport::bopomofo_layout_test_support_plain().unwrap();
        let r = BopomofoLayoutTestSupport::bopomofo_layout_test_support_layout(&vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12563,12584,12581])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((r.clusters[0usize].advance) > (plain.clusters[0usize].advance), Some(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("bopomofo reserves advance on annotated base (")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(r.clusters[0usize].advance)); __s += &(UString::from(" vs ")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(plain.clusters[0usize].advance)); __s += &(UString::from(")")); __s }).as_str()))).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(plain.clusters[1usize].advance, r.clusters[1usize].advance, Some(UString::from("current v1 does not reserve the unannotated adjacent char"))).unwrap();
    });
}

#[test]
fn font_weight_follows_annotated_base_plus_three_steps() {
    testlib::run("org.tiqian.layout.BopomofoLayoutTest.fontWeightFollowsAnnotatedBasePlusThreeSteps", "org.tiqian.layout.BopomofoLayoutTest.fontWeightFollowsAnnotatedBasePlusThreeSteps", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[102,111,110,116,87,101,105,103,104,116,70,111,108,108,111,119,115,65,110,110,111,116,97,116,101,100,66,97,115,101,80,108,117,115,84,104,114,101,101,83,116,101,112,115]));
        let r = BopomofoLayoutTestSupport::bopomofo_layout_test_support_layout(&vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12563,12584,12581])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
    (RubySpan::new(TextRange::new(1u32, 2u32).unwrap(), &(UStr::new(&[12584,12579,714])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
], Some(vec![
    (TextSpan::new(TextRange::new(0u32, 1u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(500), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
    (TextSpan::new(TextRange::new(1u32, 2u32).unwrap(), TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(700), Some(false), Some(0.0), Some(InlineAttachment::None)))).clone(),
])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(800, (r.debug).clone().bopomofo_decisions[0usize].font_weight, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(900, (r.debug).clone().bopomofo_decisions[1usize].font_weight, Some(UString::from("bopomofo weight clamps at 900"))).unwrap();
    });
}

#[test]
fn decision_keeps_source_reading_for_copy() {
    testlib::run("org.tiqian.layout.BopomofoLayoutTest.decisionKeepsSourceReadingForCopy", "org.tiqian.layout.BopomofoLayoutTest.decisionKeepsSourceReadingForCopy", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[100,101,99,105,115,105,111,110,75,101,101,112,115,83,111,117,114,99,101,82,101,97,100,105,110,103,70,111,114,67,111,112,121]));
        let r = BopomofoLayoutTestSupport::bopomofo_layout_test_support_layout(&vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[729,12553,12572])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
], None).unwrap();
        let d = ((r.debug).clone().bopomofo_decisions).clone();
        if u32::try_from((d.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[729,12553,12572]), (((r.input).clone().ruby_spans[0usize]).clone().text).to_ustring().as_ustr(), None).unwrap();
            return;
        }
        let decision = (d[0usize]).clone();
        let mut texts: Vec<UString> = vec![];
        for _g_index in 0..match u32::try_from(decision.placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let p = (decision.placements[usize::try_from(_g_index).unwrap_or(0)]).clone();
            texts.push((p.text).to_ustring());
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[729,12553,12572]), (decision.text).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![
    UString::from("˙").to_ustring(),
    UString::from("ㄉ").to_ustring(),
    UString::from("ㄜ").to_ustring(),
], &texts, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_enum(&(BopomofoGlyphRole::Neutral), &(decision.placements[0usize].role), None).unwrap();
    });
}

#[test]
fn annotation_locale_does_not_replace_simplified_base_locale() {
    testlib::run("org.tiqian.layout.BopomofoLayoutTest.annotationLocaleDoesNotReplaceSimplifiedBaseLocale", "org.tiqian.layout.BopomofoLayoutTest.annotationLocaleDoesNotReplaceSimplifiedBaseLocale", || {
        let mut t = TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,76,97,121,111,117,116,84,101,115,116])));
        t.section(UStr::new(&[97,110,110,111,116,97,116,105,111,110,76,111,99,97,108,101,68,111,101,115,78,111,116,82,101,112,108,97,99,101,83,105,109,112,108,105,102,105,101,100,66,97,115,101,76,111,99,97,108,101]));
        let r = BopomofoLayoutTestSupport::bopomofo_layout_test_support_layout(&vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), &(UStr::new(&[12563,12584,12581])), Some(vec![]), RubyKind::Bopomofo, Some(UString::from("zh-TW")))).clone(),
], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,72,97,110,115]), (((r.input).clone().text_style).clone().locale).to_ustring().as_ustr(), None).unwrap();
        if u32::try_from(((r.debug).clone().bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((r.debug).clone().bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
            return;
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[122,104,45,84,87]), (((r.debug).clone().bopomofo_decisions[0usize]).clone().locale).to_ustring().as_ustr(), None).unwrap();
    });
}
