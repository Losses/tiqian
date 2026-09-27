use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub enum TextRangeError {
    StartGreaterThanEnd,
    NegativeStart,
    Message { text: UString },
    ParagraphLayoutEngineNewFault(Box<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault>),
    ParagraphShapingStageCoverageTestSupportLayoutFault(Box<crate::org::tiqian::layout::paragraph_shaping_stage_coverage_test_support::ParagraphShapingStageCoverageTestSupportLayoutFault>),
    WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault(Box<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault>),
    WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault(Box<crate::org::tiqian::layout::width_independent_annotation_cache::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault>),
    LineAdjustmentStageFinishParagraphLayoutFault(Box<crate::org::tiqian::layout::line_adjustment_stage::LineAdjustmentStageFinishParagraphLayoutFault>),
    ParagraphShapingStageShapeParagraphFault(Box<crate::org::tiqian::layout::paragraph_shaping_stage::ParagraphShapingStageShapeParagraphFault>),
}

impl std::fmt::Display for TextRangeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextRangeError::StartGreaterThanEnd => write!(formatter, "{}", "TextRange start must not be greater than end."),
            TextRangeError::NegativeStart => write!(formatter, "{}", "TextRange start must be non-negative."),
            TextRangeError::Message { text } => {
                write!(formatter, "{}", text)
            }
            TextRangeError::ParagraphLayoutEngineNewFault(inner) => write!(formatter, "{:?}", inner),
            TextRangeError::ParagraphShapingStageCoverageTestSupportLayoutFault(inner) => write!(formatter, "{:?}", inner),
            TextRangeError::WidthIndependentAnnotationCachePrepareWidthIndependentAnnotationFault(inner) => write!(formatter, "{:?}", inner),
            TextRangeError::WidthIndependentAnnotationCacheBuildParagraphLayoutPrepFault(inner) => write!(formatter, "{:?}", inner),
            TextRangeError::LineAdjustmentStageFinishParagraphLayoutFault(inner) => write!(formatter, "{:?}", inner),
            TextRangeError::ParagraphShapingStageShapeParagraphFault(inner) => write!(formatter, "{:?}", inner),
        }
    }
}

impl std::error::Error for TextRangeError {}
