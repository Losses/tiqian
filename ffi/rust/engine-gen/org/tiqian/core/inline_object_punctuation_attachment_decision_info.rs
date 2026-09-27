use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectPunctuationAttachmentDecisionInfo {
    pub object_range: TextRange,
    pub separator_range: TextRange,
    pub punctuation_range: TextRange,
    pub punctuation_text: UString,
    pub protected_range: TextRange,
    pub collapsed_advance: f64,
    pub reason: UString,
}

impl InlineObjectPunctuationAttachmentDecisionInfo {
    pub fn new(object_range: TextRange, separator_range: TextRange, punctuation_range: TextRange, punctuation_text: &UStr, protected_range: TextRange, collapsed_advance: f64, reason: Option<UString>) -> Self {
        let reason = reason.unwrap_or_else(|| UString::from("InlineObjectPunctuationSeparatorSpaceCollapse"));
        Self {
            object_range,
            separator_range,
            punctuation_range,
            punctuation_text: punctuation_text.to_ustring(),
            protected_range,
            collapsed_advance,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineObjectPunctuationAttachmentDecisionInfo(")); __s += &(UString::from("objectRange=")); __s += UString::from(format!("{}", (self.object_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("separatorRange=")); __s += UString::from(format!("{}", (self.separator_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationRange=")); __s += UString::from(format!("{}", (self.punctuation_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("punctuationText=")); __s += (self.punctuation_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("protectedRange=")); __s += UString::from(format!("{}", (self.protected_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("collapsedAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.collapsed_advance)); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
