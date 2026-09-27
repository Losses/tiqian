use crate::org::tiqian::clreq::punctuation_class::PunctuationClass;


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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "PunctuationPolicy(",
            "punctuationClass=",
            self.punctuation_class.name(),
            ", ",
            "allowAtLineStart=",
            self.allow_at_line_start,
            ", ",
            "allowAtLineEnd=",
            self.allow_at_line_end,
            ", ",
            "defaultBodyEm=",
            self.default_body_em,
            ", ",
            "defaultAdvanceEm=",
            self.default_advance_em,
            ")"
        );
    }
}
