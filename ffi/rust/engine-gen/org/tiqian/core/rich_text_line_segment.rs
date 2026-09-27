use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::rich_text_span::RichTextSpan;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone)]
pub struct RichTextLineSegment {
    pub span: RichTextSpan,
    pub line_index: u32,
    pub range: TextRange,
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub baseline: f64,
}

impl RichTextLineSegment {
    pub fn new(span: RichTextSpan, line_index: u32, range: TextRange, left: f64, top: f64, right: f64, bottom: f64, baseline: f64) -> Self {
        Self {
            span,
            line_index,
            range,
            left,
            top,
            right,
            bottom,
            baseline,
        }
    }

    pub fn get_width(&self) -> f64 {
        return self.right - self.left;
    }

    pub fn get_height(&self) -> f64 {
        return self.bottom - self.top;
    }

    pub fn get_rect(&self) -> Rect {
        return Rect::new(self.left, self.top, self.right, self.bottom);
    }

    pub fn get_continues_from_previous_line(&self) -> bool {
        return (i32::from_ne_bytes((((self.range).clone().start) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((((self.span).clone().range).clone().start) as i32).to_ne_bytes()));
    }

    pub fn get_continues_on_next_line(&self) -> bool {
        return (i32::from_ne_bytes((((self.range).clone().end) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((((self.span).clone().range).clone().end) as i32).to_ne_bytes()));
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RichTextLineSegment(")); __s += &(UString::from("span=")); __s += UString::from(format!("{}", (self.span).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("lineIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.line_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("left=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.left)); __s += &(UString::from(", ")); __s += &(UString::from("top=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top)); __s += &(UString::from(", ")); __s += &(UString::from("right=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.right)); __s += &(UString::from(", ")); __s += &(UString::from("bottom=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.bottom)); __s += &(UString::from(", ")); __s += &(UString::from("baseline=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline)); __s += &(UString::from(")")); __s }).as_str());
    }
}
