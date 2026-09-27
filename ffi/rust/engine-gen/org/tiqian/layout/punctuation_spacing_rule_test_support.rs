use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::org::tiqian::clreq::punctuation_glue_placement::PunctuationGluePlacement;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtom;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::LazyLock;


pub static PUNCTUATION_SPACING_RULE_TEST_SUPPORT_BUILDER: LazyLock<PunctuationAtomBuilder> = LazyLock::new(|| PunctuationAtomBuilder::new(Some(PunctuationGluePlacement::MainlandSimplified), None).unwrap());
pub static PUNCTUATION_SPACING_RULE_TEST_SUPPORT_COMPRESSOR: LazyLock<PunctuationSpacingCompressor> = LazyLock::new(|| PunctuationSpacingCompressor::new().unwrap());

#[derive(Clone, Copy)]
pub struct PunctuationSpacingRuleTestSupport;

impl PunctuationSpacingRuleTestSupport {
    pub const PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM: f64 = 16.0f64;

    pub fn punctuation_spacing_rule_test_support_atom(char: &UStr, index: u32) -> Result<PunctuationAtom, TextRangeError> {
        let a = (*PUNCTUATION_SPACING_RULE_TEST_SUPPORT_BUILDER).clone().build(char, TextRange::new(index, u32::wrapping_add(index, 1))?, PunctuationSpacingRuleTestSupport::PUNCTUATION_SPACING_RULE_TEST_SUPPORT_EM, None, None, None)?;
        if a.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("atom build returned null for ")); __s += char; __s }).as_str()) });
        }
        let pol_class = ClreqPunctuationPolicies::clreq_punctuation_policies_classify(char);
        if pol_class == PunctuationClass::Other {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("unexpected punctuation class for ")); __s += char; __s }).as_str()) });
        }
        return Ok((a).unwrap());
    }
}
