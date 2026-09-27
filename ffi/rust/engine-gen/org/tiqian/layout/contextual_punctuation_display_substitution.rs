use crate::org::tiqian::clreq::cjk_punctuation_glyph_substitution::CjkPunctuationGlyphSubstitution;
use crate::org::tiqian::clreq::clreq_punctuation_glyph_substitutor::ClreqPunctuationGlyphSubstitutor;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct ContextualPunctuationDisplaySubstitutionFns;

impl ContextualPunctuationDisplaySubstitutionFns {
    pub fn contextual_punctuation_display_substitution_fns_substitute_for_role(self_: ClreqPunctuationGlyphSubstitutor, source_text: &UStr, role: FontRole) -> Result<CjkPunctuationGlyphSubstitution, UStringFault> {
        let candidate = self_.substitute(source_text)?;
        return Ok(if role == FontRole::CjkPunctuation || (candidate.display_text).to_ustring() == source_text { candidate } else { CjkPunctuationGlyphSubstitution::new(source_text, source_text, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("CjkRoleGatedDisplaySubstitution:preserve-role-")); __s += UString::from(role.name()).as_ustr(); __s }).as_str()).as_ustr()) });
    }
}
