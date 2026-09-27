#![cfg(test)]

use crate::org::tiqian::core::east_asian_spacing_edges::EastAsianSpacingEdges;
use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::org::tiqian::core::unicode_east_asian_spacing::UnicodeEastAsianSpacing;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[derive(Debug, Clone, PartialEq)]
pub enum EastAsianSpacingTestUsesPinnedUnicodeDraftDataAcrossScriptsFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    TracedAssertionsFailFaultFault(crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault),
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
        TestTraceRecorder::new("EastAsianSpacingTest").section(&"chineseLanguageContextUsesPinnedMacrolanguageRegistry");
        let _ = TracedAssertions::traced_assertions_assert_true(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(&"zh-Hans"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_true(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(&"yue-Hant-HK"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_false(UnicodeEastAsianSpacing::unicode_east_asian_spacing_is_chinese_language_context(&"en"), None).unwrap();
    });
}

#[test]
fn uses_pinned_unicode_draft_data_across_scripts() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.usesPinnedUnicodeDraftDataAcrossScripts", "org.tiqian.core.EastAsianSpacingTest.usesPinnedUnicodeDraftDataAcrossScripts", || {
        TestTraceRecorder::new("EastAsianSpacingTest").section(&"usesPinnedUnicodeDraftDataAcrossScripts");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(25552).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Wide.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(94208).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(65).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(945).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(1103).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(57).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Conditional.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(37).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(65295).unwrap().name().to_string().as_str(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_property_of(128512).unwrap().name().to_string().as_str(), None).unwrap();
    });
}

#[test]
fn resolves_conditional_values_from_chinese_language_context() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.resolvesConditionalValuesFromChineseLanguageContext", "org.tiqian.core.EastAsianSpacingTest.resolvesConditionalValuesFromChineseLanguageContext", || {
        TestTraceRecorder::new("EastAsianSpacingTest").section(&"resolvesConditionalValuesFromChineseLanguageContext");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"%", &"zh-Hans").unwrap().name().to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Narrow.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"%", &"yue-Hant-HK").unwrap().name().to_string().as_str(),
None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"%", &"en").unwrap().name().to_string().as_str(),
None).unwrap();
    });
}

#[test]
fn enclosing_mark_makes_the_whole_grapheme_cluster_other() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.enclosingMarkMakesTheWholeGraphemeClusterOther", "org.tiqian.core.EastAsianSpacingTest.enclosingMarkMakesTheWholeGraphemeClusterOther", || {
        TestTraceRecorder::new("EastAsianSpacingTest").section(&"enclosingMarkMakesTheWholeGraphemeClusterOther");
        let _ = TracedAssertions::traced_assertions_assert_equals_rendered(EastAsianSpacingValue::Other.name().to_string().as_str(), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_for_grapheme_cluster(&"A⃝", &"zh-Hans").unwrap().name().to_string().as_str(),
None).unwrap();
    });
}

#[test]
fn resolves_the_actual_source_unit_at_each_shaping_cluster_edge() {
    testlib::run("org.tiqian.core.EastAsianSpacingTest.resolvesTheActualSourceUnitAtEachShapingClusterEdge", "org.tiqian.core.EastAsianSpacingTest.resolvesTheActualSourceUnitAtEachShapingClusterEdge", || {
        TestTraceRecorder::new("EastAsianSpacingTest").section(&"resolvesTheActualSourceUnitAtEachShapingClusterEdge");
        let _ = TracedAssertions::traced_assertions_assert_equals_east_asian_spacing_edges(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Narrow, false), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(&"/Hi",
&"zh-Hans").unwrap(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_east_asian_spacing_edges(EastAsianSpacingEdges::new(EastAsianSpacingValue::Other, EastAsianSpacingValue::Other, false), UnicodeEastAsianSpacing::unicode_east_asian_spacing_resolved_edges(&"A⃝", &"zh-Hans").unwrap(),
None).unwrap();
    });
}
