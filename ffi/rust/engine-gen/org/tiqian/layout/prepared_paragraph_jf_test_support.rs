use crate::org::tiqian::core::int_range::IntRange;
use crate::org::tiqian::core::line_box::LineBox;
use crate::org::tiqian::core::line_debug_info::LineDebugInfo;
use crate::org::tiqian::core::line_end_reason::LineEndReason;
use crate::org::tiqian::core::text_range::TextRange;


#[derive(Clone, Copy)]
pub struct PreparedParagraphJfTestSupport;

impl PreparedParagraphJfTestSupport {
    pub fn prepared_paragraph_jf_test_support_line(range: TextRange, cluster_range: IntRange, natural_width: Option<f64>) -> LineBox {
        let width = match &(natural_width) { None => 26.0f64, Some(__option) => *__option };
        return LineBox::new((range).clone(), (cluster_range).clone(), 20 as f64 as f64, 0 as f64 as f64, 24 as f64 as f64, width, width, width, Some(0.0), Some(0.0), Some(LineEndReason::ParagraphEnd), Some(0.0), Some(vec![]), LineDebugInfo::new(None.clone(), Some(vec![])));
    }
}
