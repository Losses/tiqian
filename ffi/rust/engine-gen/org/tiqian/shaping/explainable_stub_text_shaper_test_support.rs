use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_role::FontRole;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct ExplainableStubTextShaperTestSupport;

impl ExplainableStubTextShaperTestSupport {
    pub fn explainable_stub_text_shaper_test_support_input(text: &UStr, role: FontRole, display_text: Option<UString>) -> Result<ShapingInput, TextRangeError> {
        let range = TextRange::new(0u32, u_string::unit_count(&(text)))?;
        return Ok(ShapingInput::new(text, (range).clone(), TextStyle::new(Some(vec![]), Some(16.0f64), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None)), FontDecision::new((range).clone(), FontCandidate::new(&(UStr::new(&[116,101,115,116,45,102,111,110,116])), &(UStr::new(&[116,101,115,116,45,102,111,110,116])), role), role, &(UStr::new(&[116,101,115,116]))), Some((match &(display_text) { None => text.to_ustring(), Some(__option) => __option.to_ustring() }).to_ustring()), Some(vec![])));
    }
}
