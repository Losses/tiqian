use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_role::FontRole;


#[derive(Clone, Copy)]
pub struct CjkFontRoleClassifierTestSupport;

impl CjkFontRoleClassifierTestSupport {
    pub fn cjk_font_role_classifier_test_support_c(t: &str, s: u32, e: u32) -> Result<FontRole, TextRangeError> {
        return Ok(CjkFontRoleClassifier::new().classify(t, TextRange::new(s, e)?, None));
    }
}
