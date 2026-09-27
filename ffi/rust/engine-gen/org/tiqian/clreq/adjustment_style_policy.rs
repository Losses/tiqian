use crate::org::tiqian::clreq::line_adjustment_strategy::LineAdjustmentStrategy;
use crate::org::tiqian::clreq::line_end_punctuation_style::LineEndPunctuationStyle;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct AdjustmentStylePolicy {
    pub line_end_punctuation: LineEndPunctuationStyle,
    pub allow_inline_stop_compression: bool,
    pub allow_sino_western_gap_adjustment: bool,
    pub line_adjustment: LineAdjustmentStrategy,
}

impl AdjustmentStylePolicy {
    pub fn new(line_end_punctuation: Option<LineEndPunctuationStyle>, allow_inline_stop_compression: Option<bool>, allow_sino_western_gap_adjustment: Option<bool>, line_adjustment: Option<LineAdjustmentStrategy>) -> Self {
        let line_end_punctuation = line_end_punctuation.unwrap_or_else(|| LineEndPunctuationStyle::ForceHalfWidth);
        let allow_inline_stop_compression = allow_inline_stop_compression.unwrap_or_else(|| true);
        let allow_sino_western_gap_adjustment = allow_sino_western_gap_adjustment.unwrap_or_else(|| true);
        let line_adjustment = line_adjustment.unwrap_or_else(|| LineAdjustmentStrategy::PushInFirst);
        Self {
            line_end_punctuation: line_end_punctuation,
            allow_inline_stop_compression: allow_inline_stop_compression,
            allow_sino_western_gap_adjustment: allow_sino_western_gap_adjustment,
            line_adjustment: line_adjustment,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("AdjustmentStylePolicy(")); __s += &(UString::from("lineEndPunctuation=")); __s += UString::from(self.line_end_punctuation.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("allowInlineStopCompression=")); __s += UString::from(format!("{}", (self.allow_inline_stop_compression).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("allowSinoWesternGapAdjustment=")); __s += UString::from(format!("{}", (self.allow_sino_western_gap_adjustment).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineAdjustment=")); __s += UString::from(self.line_adjustment.name()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn adjustment_style_policy_same_policy(a: AdjustmentStylePolicy, b: AdjustmentStylePolicy) -> bool {
        return a.line_end_punctuation == b.line_end_punctuation && a.allow_inline_stop_compression == b.allow_inline_stop_compression && a.allow_sino_western_gap_adjustment == b.allow_sino_western_gap_adjustment && a.line_adjustment == b.line_adjustment;
    }
}
