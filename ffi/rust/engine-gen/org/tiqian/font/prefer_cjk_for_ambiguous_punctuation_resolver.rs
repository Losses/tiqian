use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_policy::FallbackResolver;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, PartialEq)]
pub struct PreferCjkForAmbiguousPunctuationResolver {
    pub(crate) cjk_font_key: UString,
    pub(crate) latin_font_key: UString,
    pub(crate) symbol_font_key: UString,
}

impl PreferCjkForAmbiguousPunctuationResolver {
    pub fn new(cjk_font_key: Option<UString>, latin_font_key: Option<UString>, symbol_font_key: Option<UString>) -> Self {
        let cjk_font_key = cjk_font_key.unwrap_or_else(|| UString::from("cjk-primary"));
        let latin_font_key = latin_font_key.unwrap_or_else(|| UString::from("latin-primary"));
        let symbol_font_key = symbol_font_key.unwrap_or_else(|| UString::from("symbol-fallback"));
        Self {
            cjk_font_key: cjk_font_key,
            latin_font_key: latin_font_key,
            symbol_font_key: symbol_font_key,
        }
    }

    pub fn resolve(&self, _text: &UStr, range: TextRange, request: FontRequest) -> FontDecision {
        let c = PreferCjkForAmbiguousPunctuationResolver::prefer_cjk_for_ambiguous_punctuation_resolver_candidate_for((request).clone(), (self.cjk_font_key).to_ustring().as_ustr(), (self.latin_font_key).to_ustring().as_ustr(), (self.symbol_font_key).to_ustring().as_ustr());
        return FontDecision::new((range).clone(), (c).clone(), request.role, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PreferCjkForAmbiguousPunctuationResolver:")); __s += UString::from(request.role.name()).as_ustr(); __s }).as_str()).as_ustr());
    }

    pub(crate) fn prefer_cjk_for_ambiguous_punctuation_resolver_candidate_for(request: FontRequest, cjk_font_key: &UStr, latin_font_key: &UStr, symbol_font_key: &UStr) -> FontCandidate {
        let role = request.role;
        return match role {
            FontRole::CjkText | FontRole::CjkPunctuation => FontCandidate::new(cjk_font_key, if u32::try_from((request.preferred_families.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { cjk_font_key.to_ustring() } else { (request.preferred_families[0usize]).clone() }.as_ustr(), request.role),
            FontRole::LatinText => FontCandidate::new(latin_font_key, latin_font_key, request.role),
            FontRole::Symbol | FontRole::Emoji | FontRole::Unknown => FontCandidate::new(symbol_font_key, symbol_font_key, request.role),
        };
    }
}

impl FallbackResolver for PreferCjkForAmbiguousPunctuationResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.font.PreferCjkForAmbiguousPunctuationResolver.PreferCjkForAmbiguousPunctuationResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FallbackResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _text: &UStr, range: TextRange, request: FontRequest) -> FontDecision {
        let c = PreferCjkForAmbiguousPunctuationResolver::prefer_cjk_for_ambiguous_punctuation_resolver_candidate_for((request).clone(), (self.cjk_font_key).to_ustring().as_ustr(), (self.latin_font_key).to_ustring().as_ustr(), (self.symbol_font_key).to_ustring().as_ustr());
        return FontDecision::new((range).clone(), (c).clone(), request.role, UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PreferCjkForAmbiguousPunctuationResolver:")); __s += UString::from(request.role.name()).as_ustr(); __s }).as_str()).as_ustr());
    }
}
