use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationPolicy {
    pub punctuation_class: PunctuationClass,
    pub allow_at_line_start: bool,
    pub allow_at_line_end: bool,
    pub default_body_em: f64,
    pub default_advance_em: f64,
}

impl PunctuationPolicy {
    pub fn new(punctuation_class: PunctuationClass, allow_at_line_start: bool, allow_at_line_end: bool, default_body_em: f64, default_advance_em: Option<f64>) -> Self {
        let default_advance_em = default_advance_em.unwrap_or_else(|| 1.0);
        Self {
            punctuation_class,
            allow_at_line_start,
            allow_at_line_end,
            default_body_em,
            default_advance_em: default_advance_em,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PunctuationPolicy(")); __s += &(UString::from("punctuationClass=")); __s += UString::from(self.punctuation_class.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("allowAtLineStart=")); __s += UString::from(format!("{}", (self.allow_at_line_start).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("allowAtLineEnd=")); __s += UString::from(format!("{}", (self.allow_at_line_end).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("defaultBodyEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.default_body_em)); __s += &(UString::from(", ")); __s += &(UString::from("defaultAdvanceEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.default_advance_em)); __s += &(UString::from(")")); __s }).as_str());
    }
}
