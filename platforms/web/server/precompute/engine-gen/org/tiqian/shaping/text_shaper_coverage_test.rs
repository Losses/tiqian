#![cfg(test)]

use crate::org::tiqian::core::illegal_state_exception::IllegalStateException;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingSource;
use crate::org::tiqian::shaping::text_shaper::TextShaper;
use crate::org::tiqian::shaping::text_shaper::UnimplementedTextShaper;
use crate::org::tiqian::shaping::text_shaper_coverage_test_support::TextShaperCoverageTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    TraceAssertionErrorFault(crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError),
    IllegalStateExceptionFault(crate::org::tiqian::core::illegal_state_exception::IllegalStateException),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TracedAssertionsAssertFailsWithFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TraceAssertionErrorFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::IllegalStateExceptionFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TracedAssertionsAssertFailsWithFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TraceAssertionErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::org::tiqian::core::illegal_state_exception::IllegalStateException {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::IllegalStateExceptionFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TracedAssertionsAssertFailsWithFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault) -> Self {
        match value {
            TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::org::tiqian::test::trace::trace_assertion_exception::TraceAssertionError) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TraceAssertionErrorFault(value)
    }
}

impl From<crate::org::tiqian::core::illegal_state_exception::IllegalStateException> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::org::tiqian::core::illegal_state_exception::IllegalStateException) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::IllegalStateExceptionFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsAssertFailsWithFault) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TracedAssertionsAssertFailsWithFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextShaperCoverageTestUnimplementedTextShaperThrowsOnShapeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault) -> Self {
        match value {
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault) -> Self {
        match value {
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault) -> Self {
        match value {
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault) -> Self {
        match value {
            TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextShaperCoverageTestSurrogatePairHandlingInCodePointCountFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}
impl std::fmt::Display for TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault) -> Self {
        match value {
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault) -> Self {
        match value {
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault) -> Self {
        match value {
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault) -> Self {
        match value {
            TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TracedAssertionsFailFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextShaperCoverageTestShapingInputWithFeaturesAndConstantsFault::TextRangeErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault {
    TextShaperShapeFaultFault(crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TextShaperShapeFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault> for crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault {
    fn from(value: TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault) -> Self {
        match value {
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TextShaperShapeFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault) -> Self {
        match value {
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault) -> Self {
        match value {
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault) -> Self {
        match value {
            TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault> for TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault {
    fn from(value: crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault) -> Self {
        TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TextShaperShapeFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        TextShaperCoverageTestExplainableStubNominalAdvanceBranchesFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn covers_all_shaping_source_enum_entries() {
    testlib::run("org.tiqian.shaping.TextShaperCoverageTest.coversAllShapingSourceEnumEntries", "org.tiqian.shaping.TextShaperCoverageTest.coversAllShapingSourceEnumEntries", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,83,104,97,112,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[99,111,118,101,114,115,65,108,108,83,104,97,112,105,110,103,83,111,117,114,99,101,69,110,117,109,69,110,116,114,105,101,115]));
        let sources = ShapingSource::ALL;
        let mut has_stub = false;
        let mut has_jvm_awt = false;
        let mut has_android_paint = false;
        let mut has_skia = false;
        let mut has_harf_buzz = false;
        let mut has_core_text = false;
        let mut e = 0u32;
        while (i32::from_ne_bytes(((e) as i32).to_ne_bytes())) < (6) {
            let entry = sources[usize::try_from(e).unwrap_or(0)];
            if entry == ShapingSource::Stub {
                has_stub = true;
            }
            if entry == ShapingSource::JvmAwt {
                has_jvm_awt = true;
            }
            if entry == ShapingSource::AndroidPaint {
                has_android_paint = true;
            }
            if entry == ShapingSource::Skia {
                has_skia = true;
            }
            if entry == ShapingSource::HarfBuzz {
                has_harf_buzz = true;
            }
            if entry == ShapingSource::CoreText {
                has_core_text = true;
            }
            e = u32::wrapping_add(e, 1);
        }
        let _ = TracedAssertions::traced_assertions_assert_true(has_stub, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has_jvm_awt, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has_android_paint, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has_skia, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has_harf_buzz, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(has_core_text, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(6, 6, None).unwrap();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (6) {
            let source = sources[usize::try_from(i).unwrap_or(0)];
            let name = UString::from(source.name());
            let _ = TracedAssertions::traced_assertions_assert_equals_rendered(name.as_ustr(), UString::from(ShapingSource::from_name(name.as_ustr()).unwrap().name()).as_ustr(), None).unwrap();
            i = u32::wrapping_add(i, 1);
        }
    });
}

#[test]
fn unimplemented_text_shaper_throws_on_shape() {
    testlib::run("org.tiqian.shaping.TextShaperCoverageTest.unimplementedTextShaperThrowsOnShape", "org.tiqian.shaping.TextShaperCoverageTest.unimplementedTextShaperThrowsOnShape", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,83,104,97,112,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[117,110,105,109,112,108,101,109,101,110,116,101,100,84,101,120,116,83,104,97,112,101,114,84,104,114,111,119,115,79,110,83,104,97,112,101]));
        let error = TracedAssertions::traced_assertions_assert_fails_with(None.clone(), {  Arc::new(move || {
        Arc::new(Mutex::new(UnimplementedTextShaper::new())).lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[116,101,115,116]), None, None, None).map_err(|e| IllegalStateException::new(&format!("{}", e)))?).map_err(|e| IllegalStateException::new(&format!("{}", e)))?;
        Ok(())
}) }).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true((u32::from_ne_bytes(((u_string::find_from(&UString::from(format!("{}", format!("{}", error)).as_str()), UString::from("platform-specific").as_ustr(), 0)) as u32).to_ne_bytes())) <= 2147483647, None).unwrap();
    });
}

#[test]
fn explainable_stub_nominal_advance_branches() {
    testlib::run("org.tiqian.shaping.TextShaperCoverageTest.explainableStubNominalAdvanceBranches", "org.tiqian.shaping.TextShaperCoverageTest.explainableStubNominalAdvanceBranches", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,83,104,97,112,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[101,120,112,108,97,105,110,97,98,108,101,83,116,117,98,78,111,109,105,110,97,108,65,100,118,97,110,99,101,66,114,97,110,99,104,101,115]));
        let shaper = Arc::new(Mutex::new(ExplainableStubTextShaper::new()));
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[11834]), Some(FontRole::CjkPunctuation), None, None).unwrap()).unwrap().clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[8212,8212]), Some(FontRole::CjkPunctuation), Some(UString::from("⸺")), None).unwrap()).unwrap().clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(8.0f64, shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[32]), None, None, None).unwrap()).unwrap().clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(24.0f64, shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[32,32,32]), None, None, None).unwrap()).unwrap().clusters[0usize].advance, None).unwrap();
        let empty = shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[]), None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(0.0f64, empty.clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((empty.glyph_runs[0usize]).clone().glyphs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[32,97]), None, None, None).unwrap()).unwrap().clusters[0usize].advance, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[97,32]), None, None, None).unwrap()).unwrap().clusters[0usize].advance, None).unwrap();
    });
}

#[test]
fn surrogate_pair_handling_in_code_point_count() {
    testlib::run("org.tiqian.shaping.TextShaperCoverageTest.surrogatePairHandlingInCodePointCount", "org.tiqian.shaping.TextShaperCoverageTest.surrogatePairHandlingInCodePointCount", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,83,104,97,112,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[115,117,114,114,111,103,97,116,101,80,97,105,114,72,97,110,100,108,105,110,103,73,110,67,111,100,101,80,111,105,110,116,67,111,117,110,116]));
        let shaper = Arc::new(Mutex::new(ExplainableStubTextShaper::new()));
        let one = shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_surrogate_text(&vec![55357, 56832]).as_ustr(), None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, one.decisions[0usize].glyph_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, one.clusters[0usize].advance, None).unwrap();
        let two = shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_surrogate_text(&vec![55357, 56832, 55360, 56331]).as_ustr(), None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, two.decisions[0usize].glyph_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, two.clusters[0usize].advance, None).unwrap();
        let high = shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_surrogate_text(&vec![55357]).as_ustr(), None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, high.decisions[0usize].glyph_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(16.0f64, high.clusters[0usize].advance, None).unwrap();
        let invalid = shaper.lock().unwrap().shape(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(TextShaperCoverageTestSupport::text_shaper_coverage_test_support_surrogate_text(&vec![55357, 65]).as_ustr(), None, None, None).unwrap()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, invalid.decisions[0usize].glyph_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_float(32.0f64, invalid.clusters[0usize].advance, None).unwrap();
    });
}

#[test]
fn shaping_input_with_features_and_constants() {
    testlib::run("org.tiqian.shaping.TextShaperCoverageTest.shapingInputWithFeaturesAndConstants", "org.tiqian.shaping.TextShaperCoverageTest.shapingInputWithFeaturesAndConstants", || {
        TestTraceRecorder::new(&(UStr::new(&[84,101,120,116,83,104,97,112,101,114,67,111,118,101,114,97,103,101,84,101,115,116]))).section(UStr::new(&[115,104,97,112,105,110,103,73,110,112,117,116,87,105,116,104,70,101,97,116,117,114,101,115,65,110,100,67,111,110,115,116,97,110,116,115]));
        let input_value = TextShaperCoverageTestSupport::text_shaper_coverage_test_support_input(UStr::new(&[84,101,115,116]), None, None.clone(), Some(vec![UString::from("fwid=1").to_ustring(), UString::from("vert=1").to_ustring()])).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("fwid=1").to_ustring(), UString::from("vert=1").to_ustring()], &input_value.open_type_features, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[84,101,115,116]), (input_value.display_text).to_ustring().as_ustr(), None).unwrap();
        let result = Arc::new(Mutex::new(ExplainableStubTextShaper::new())).lock().unwrap().shape((input_value).clone()).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, result.decisions[0usize].glyph_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(4, result.decisions[0usize].glyphs_without_ink_bounds, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[69,120,112,108,97,105,110,97,98,108,101,83,116,117,98,84,101,120,116,83,104,97,112,101,114,58,110,111,109,105,110,97,108,45,101,109,45,97,100,118,97,110,99,101]), ((result.decisions[0usize]).clone().reason).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UString::from(ShapingSource::Stub.name()).as_ustr(), ((result.decisions[0usize]).clone().source).to_ustring().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("'")); __s += TextShaper::TEXT_SHAPER_UNVERIFIED_DISPLAY_SUBSTITUTION_COVERAGE_ISSUE.to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_record_rendered_not_null(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("'")); __s += TextShaper::TEXT_SHAPER_PLATFORM_MULTI_FACE_STRING_DRAW_ISSUE.to_ustring().as_ustr(); __s += &(UString::from("'")); __s }).as_str()).as_ustr(), None).unwrap();
    });
}
