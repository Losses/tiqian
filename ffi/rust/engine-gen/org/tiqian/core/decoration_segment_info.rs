use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationSegmentInfo {
    pub source_range: TextRange,
    pub kind: UString,
    pub line_index: u32,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub open_start: bool,
    pub open_end: bool,
    pub reason: UString,
}

impl DecorationSegmentInfo {
    pub fn new(source_range: TextRange, kind: &UStr, line_index: u32, left: f64, top: f64, right: f64, bottom: f64, open_start: bool, open_end: bool, reason: &UStr) -> Self {
        Self {
            source_range,
            kind: kind.to_ustring(),
            line_index,
            left,
            top,
            right,
            bottom,
            open_start,
            open_end,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("DecorationSegmentInfo(")); __s += &(UString::from("sourceRange=")); __s += UString::from(format!("{}", (self.source_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kind=")); __s += (self.kind).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.line_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("left=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.left)); __s += &(UString::from(", ")); __s += &(UString::from("top=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top)); __s += &(UString::from(", ")); __s += &(UString::from("right=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.right)); __s += &(UString::from(", ")); __s += &(UString::from("bottom=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.bottom)); __s += &(UString::from(", ")); __s += &(UString::from("openStart=")); __s += UString::from(format!("{}", (self.open_start).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("openEnd=")); __s += UString::from(format!("{}", (self.open_end).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
