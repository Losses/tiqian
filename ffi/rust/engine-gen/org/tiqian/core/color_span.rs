#[derive(Debug, Clone, PartialEq)]
pub struct ColorSpan {
    pub start: u32,
    pub end: u32,
    pub argb: u32,
}

impl ColorSpan {
    pub fn new(start: u32, end: u32, argb: u32) -> Self {
        Self {
            start,
            end,
            argb,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "ColorSpan(",
            "start=",
            crate::runtime::int_text::IntText::int_text(self.start),
            ", ",
            "end=",
            crate::runtime::int_text::IntText::int_text(self.end),
            ", ",
            "argb=",
            crate::runtime::int_text::IntText::int_text(self.argb),
            ")"
        );
    }
}

pub fn compare_color_span(a: &ColorSpan, b: &ColorSpan) -> i32 {
    let cmp_start = if a.start < b.start { -1 } else if a.start > b.start { 1 } else { 0 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = if a.end < b.end { -1 } else if a.end > b.end { 1 } else { 0 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_argb = if a.argb < b.argb { -1 } else if a.argb > b.argb { 1 } else { 0 };
    if cmp_argb != 0 { return cmp_argb; }
    0
}
