#![cfg(test)]

use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault) -> Self {
        match value {
            EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault) -> Self {
        match value {
            EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault) -> Self {
        match value {
            EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault) -> Self {
        match value {
            EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault) -> Self {
        match value {
            EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault) -> Self {
        match value {
            EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingTestResolvesTheActualSourceUnitAtEachShapingClusterEdgeFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault) -> Self {
        match value {
            EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault) -> Self {
        match value {
            EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault) -> Self {
        match value {
            EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingTestResolvesConditionalValuesFromChineseLanguageContextFault::TracedAssertionsFailFaultFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
}
impl std::fmt::Display for EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::TracedAssertionsFailFaultFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault) -> Self {
        match value {
            EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault) -> Self {
        match value {
            EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault> for crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault {
    fn from(value: EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault) -> Self {
        match value {
            EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::TracedAssertionsFailFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::TextRangeErrorFault(value)
    }
}

impl From<crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault> for EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault {
    fn from(value: crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault) -> Self {
        EastAsianSpacingTestEnclosingMarkMakesTheWholeGraphemeClusterOtherFault::TracedAssertionsFailFaultFault(value)
    }
}

#[test]
fn chinese_language_context_uses_pinned_macrolanguage_registry() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.chineseLanguageContextUsesPinnedMacrolanguageRegistry", "org.tiqian.core.EastAsianSpacingTest.chineseLanguageContextUsesPinnedMacrolanguageRegistry", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,84,101,115,116]))).section(UStr::new(&[99,104,105,110,101,115,101,76,97,110,103,117,97,103,101,67,111,110,116,101,120,116,85,115,101,115,80,105,110,110,101,100,77,97,99,114,111,108,97,110,103,117,97,103,101,82,101,103,105,115,116,114,121]));
        let _ = TracedAssertions::traced_assertions_assert_true(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(UStr::new(&[122,104,45,72,97,110,115])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(UStr::new(&[121,117,101,45,72,97,110,116,45,72,75])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(UStr::new(&[101,110])), None).unwrap();
    });
}

#[test]
fn uses_pinned_unicode_draft_data_across_scripts() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.usesPinnedUnicodeDraftDataAcrossScripts", "org.tiqian.core.EastAsianSpacingTest.usesPinnedUnicodeDraftDataAcrossScripts", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,84,101,115,116]))).section(UStr::new(&[117,115,101,115,80,105,110,110,101,100,85,110,105,99,111,100,101,68,114,97,102,116,68,97,116,97,65,99,114,111,115,115,83,99,114,105,112,116,115]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(25552).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Wide.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(94208).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(65).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(945).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(1103).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(57).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Conditional.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(37).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(65295).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(128512).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn resolves_conditional_values_from_chinese_language_context() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.resolvesConditionalValuesFromChineseLanguageContext", "org.tiqian.core.EastAsianSpacingTest.resolvesConditionalValuesFromChineseLanguageContext", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,84,101,115,116]))).section(UStr::new(&[114,101,115,111,108,118,101,115,67,111,110,100,105,116,105,111,110,97,108,86,97,108,117,101,115,70,114,111,109,67,104,105,110,101,115,101,76,97,110,103,117,97,103,101,67,111,110,116,101,120,116]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[37]), UStr::new(&[122,104,45,72,97,110,115])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Narrow.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[37]), UStr::new(&[121,117,101,45,72,97,110,116,45,72,75])).unwrap().name()).as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[37]), UStr::new(&[101,110])).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn enclosing_mark_makes_the_whole_grapheme_cluster_other() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.enclosingMarkMakesTheWholeGraphemeClusterOther", "org.tiqian.core.EastAsianSpacingTest.enclosingMarkMakesTheWholeGraphemeClusterOther", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,84,101,115,116]))).section(UStr::new(&[101,110,99,108,111,115,105,110,103,77,97,114,107,77,97,107,101,115,84,104,101,87,104,111,108,101,71,114,97,112,104,101,109,101,67,108,117,115,116,101,114,79,116,104,101,114]));
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(UString::from(EastAsianSpacingValue::Other.name()).as_ustr(), UString::from(UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(UStr::new(&[65,8413]), UStr::new(&[122,104,45,72,97,110,115])).unwrap().name()).as_ustr(), None).unwrap();
    });
}

#[test]
fn resolves_the_actual_source_unit_at_each_shaping_cluster_edge() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.resolvesTheActualSourceUnitAtEachShapingClusterEdge", "org.tiqian.core.EastAsianSpacingTest.resolvesTheActualSourceUnitAtEachShapingClusterEdge", || {
        TestTraceRecorder::new(&(UStr::new(&[69,97,115,116,65,115,105,97,110,83,112,97,99,105,110,103,84,101,115,116]))).section(UStr::new(&[114,101,115,111,108,118,101,115,84,104,101,65,99,116,117,97,108,83,111,117,114,99,101,85,110,105,116,65,116,69,97,99,104,83,104,97,112,105,110,103,67,108,117,115,116,101,114,69,100,103,101]));
        let _ = TracedAssertions::traced_assertions_assert_equals_east_asian_spacing_edges(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Narrow, false), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(UStr::new(&[47,72,105]), UStr::new(&[122,104,45,72,97,110,115])).unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_east_asian_spacing_edges(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(UStr::new(&[65,8413]), UStr::new(&[122,104,45,72,97,110,115])).unwrap(), None).unwrap();
    });
}
