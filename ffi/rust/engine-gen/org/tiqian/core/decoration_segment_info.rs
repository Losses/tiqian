use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationSegmentInfo {
    pub source_range: TextRange,
    pub kind: String,
    pub line_index: u32,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub open_start: bool,
    pub open_end: bool,
    pub reason: String,
}

impl DecorationSegmentInfo {
    pub fn new(source_range: TextRange, kind: &str, line_index: u32, left: f64, top: f64, right: f64, bottom: f64, open_start: bool, open_end: bool, reason: &str) -> Self {
        Self {
            source_range,
            kind: kind.to_string(),
            line_index,
            left,
            top,
            right,
            bottom,
            open_start,
            open_end,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "DecorationSegmentInfo(",
            "sourceRange=",
            (self.source_range).clone().to_string(),
            ", ",
            "kind=",
            (self.kind).to_string(),
            ", ",
            "lineIndex=",
            crate::runtime::int_text::IntText::int_text(self.line_index),
            ", ",
            "left=",
            self.left,
            ", ",
            "top=",
            self.top,
            ", ",
            "right=",
            self.right,
            ", ",
            "bottom=",
            self.bottom,
            ", ",
            "openStart=",
            self.open_start,
            ", ",
            "openEnd=",
            self.open_end,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
