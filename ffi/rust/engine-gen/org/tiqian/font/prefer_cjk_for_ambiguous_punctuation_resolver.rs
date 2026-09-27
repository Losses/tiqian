use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_policy::FallbackResolver;
use crate::org::tiqian::font::font_policy::FontCandidate;
use crate::org::tiqian::font::font_policy::FontDecision;
use crate::org::tiqian::font::font_policy::FontRequest;
use crate::org::tiqian::font::font_role::FontRole;


#[derive(Clone, PartialEq)]
pub struct PreferCjkForAmbiguousPunctuationResolver {
    pub(crate) cjk_font_key: String,
    pub(crate) latin_font_key: String,
    pub(crate) symbol_font_key: String,
}

impl PreferCjkForAmbiguousPunctuationResolver {
    pub fn new(cjk_font_key: Option<String>, latin_font_key: Option<String>, symbol_font_key: Option<String>) -> Self {
        let cjk_font_key = cjk_font_key.unwrap_or_else(|| "cjk-primary".to_string());
        let latin_font_key = latin_font_key.unwrap_or_else(|| "latin-primary".to_string());
        let symbol_font_key = symbol_font_key.unwrap_or_else(|| "symbol-fallback".to_string());
        Self {
            cjk_font_key: cjk_font_key,
            latin_font_key: latin_font_key,
            symbol_font_key: symbol_font_key,
        }
    }

    pub fn resolve(&self, _text: &str, range: TextRange, request: FontRequest) -> FontDecision {
        let c = PreferCjkForAmbiguousPunctuationResolver::prefer_cjk_for_ambiguous_punctuation_resolver_candidate_for((request).clone(), (self.cjk_font_key).to_string().as_str(), (self.latin_font_key).to_string().as_str(), (self.symbol_font_key).to_string().as_str());
        return FontDecision::new((range).clone(), (c).clone(), request.role, format!("{}{}",
            "PreferCjkForAmbiguousPunctuationResolver:",
            request.role.name()
        ).as_str());
    }

    pub(crate) fn prefer_cjk_for_ambiguous_punctuation_resolver_candidate_for(request: FontRequest, cjk_font_key: &str, latin_font_key: &str, symbol_font_key: &str) -> FontCandidate {
        let role = request.role;
        return match role {
            FontRole::CjkText | FontRole::CjkPunctuation => FontCandidate::new(cjk_font_key, if u32::try_from((request.preferred_families.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { cjk_font_key.to_string() } else { (request.preferred_families[0usize]).clone() }.as_str(),
request.role),
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

    fn resolve(&self, _text: &str, range: TextRange, request: FontRequest) -> FontDecision {
        let c = PreferCjkForAmbiguousPunctuationResolver::prefer_cjk_for_ambiguous_punctuation_resolver_candidate_for((request).clone(), (self.cjk_font_key).to_string().as_str(), (self.latin_font_key).to_string().as_str(), (self.symbol_font_key).to_string().as_str());
        return FontDecision::new((range).clone(), (c).clone(), request.role, format!("{}{}",
            "PreferCjkForAmbiguousPunctuationResolver:",
            request.role.name()
        ).as_str());
    }
}
