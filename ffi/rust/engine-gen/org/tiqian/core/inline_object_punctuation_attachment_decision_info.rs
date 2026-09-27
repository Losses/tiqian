use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectPunctuationAttachmentDecisionInfo {
    pub object_range: TextRange,
    pub separator_range: TextRange,
    pub punctuation_range: TextRange,
    pub punctuation_text: String,
    pub protected_range: TextRange,
    pub collapsed_advance: f64,
    pub reason: String,
}

impl InlineObjectPunctuationAttachmentDecisionInfo {
    pub fn new(object_range: TextRange, separator_range: TextRange, punctuation_range: TextRange, punctuation_text: &str, protected_range: TextRange, collapsed_advance: f64, reason: Option<String>) -> Self {
        let reason = reason.unwrap_or_else(|| "InlineObjectPunctuationSeparatorSpaceCollapse".to_string());
        Self {
            object_range,
            separator_range,
            punctuation_range,
            punctuation_text: punctuation_text.to_string(),
            protected_range,
            collapsed_advance,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "InlineObjectPunctuationAttachmentDecisionInfo(",
            "objectRange=",
            (self.object_range).clone().to_string(),
            ", ",
            "separatorRange=",
            (self.separator_range).clone().to_string(),
            ", ",
            "punctuationRange=",
            (self.punctuation_range).clone().to_string(),
            ", ",
            "punctuationText=",
            (self.punctuation_text).to_string(),
            ", ",
            "protectedRange=",
            (self.protected_range).clone().to_string(),
            ", ",
            "collapsedAdvance=",
            self.collapsed_advance,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
