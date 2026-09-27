use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct ExplainableStubTextShaperTestSupport;

impl ExplainableStubTextShaperTestSupport {
    pub fn explainable_stub_text_shaper_test_support_input(text: &str, role: FontRole, display_text: Option<String>) -> Result<ShapingInput, TextRangeError> {
        let range = TextRange::new(0u32, u_string::unit_count(&(text)))?;
        return Ok(ShapingInput::new(text, (range).clone(), TextStyle::new(Some(vec![]), Some(16.0f64), Some("zh-Hans".to_string()), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)), FontDecision::new((range).clone(), FontCandidate::new("test-font", "test-font",
role), role, "test"), Some(match &(display_text) { None => text.to_string(), Some(__option) => __option.to_string() }.to_string()), Some(vec![])));
    }
}
