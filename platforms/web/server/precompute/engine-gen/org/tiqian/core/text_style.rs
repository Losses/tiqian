use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub font_families: Vec<UString>,
    pub font_size: f64,
    pub locale: UString,
    pub font_weight: u32,
    pub italic: bool,
    pub baseline_shift: f64,
    pub inline_attachment: InlineAttachment,
}

impl TextStyle {
    pub fn new(font_families: Option<Vec<UString>>, font_size: Option<f64>, locale: Option<UString>, font_weight: Option<u32>, italic: Option<bool>, baseline_shift: Option<f64>, inline_attachment: Option<InlineAttachment>) -> Self {
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_size = font_size.unwrap_or_else(|| 16.0);
        let locale = locale.unwrap_or_else(|| UString::from("zh-Hans"));
        let font_weight = font_weight.unwrap_or_else(|| 400);
        let italic = italic.unwrap_or_else(|| false);
        let baseline_shift = baseline_shift.unwrap_or_else(|| 0.0);
        let inline_attachment = inline_attachment.unwrap_or_else(|| InlineAttachment::None);
        Self {
            font_families: font_families,
            font_size: font_size,
            locale: locale,
            font_weight: font_weight,
            italic: italic,
            baseline_shift: baseline_shift,
            inline_attachment: inline_attachment,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("TextStyle(")); __s += &(UString::from("fontFamilies=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.font_families).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontSize=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.font_size)); __s += &(UString::from(", ")); __s += &(UString::from("locale=")); __s += (self.locale).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontWeight=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.font_weight)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("italic=")); __s += UString::from(format!("{}", (self.italic).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("baselineShift=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.baseline_shift)); __s += &(UString::from(", ")); __s += &(UString::from("inlineAttachment=")); __s += UString::from(self.inline_attachment.name()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
