use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub font_families: Vec<String>,
    pub font_size: f64,
    pub locale: String,
    pub font_weight: u32,
    pub italic: bool,
    pub baseline_shift: f64,
    pub inline_attachment: InlineAttachment,
}

impl TextStyle {
    pub fn new(font_families: Option<Vec<String>>, font_size: Option<f64>, locale: Option<String>, font_weight: Option<u32>, italic: Option<bool>, baseline_shift: Option<f64>, inline_attachment: Option<InlineAttachment>) -> Self {
        let font_families = font_families.unwrap_or_else(|| vec![]);
        let font_size = font_size.unwrap_or_else(|| 16.0);
        let locale = locale.unwrap_or_else(|| "zh-Hans".to_string());
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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "TextStyle(",
            "fontFamilies=",
            {
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
    },
            ", ",
            "fontSize=",
            self.font_size,
            ", ",
            "locale=",
            (self.locale).to_string(),
            ", ",
            "fontWeight=",
            crate::runtime::int_text::IntText::int_text(self.font_weight),
            ", ",
            "italic=",
            self.italic,
            ", ",
            "baselineShift=",
            self.baseline_shift,
            ", ",
            "inlineAttachment=",
            self.inline_attachment.name(),
            ")"
        );
    }
}
