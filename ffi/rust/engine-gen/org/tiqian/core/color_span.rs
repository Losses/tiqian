use crate::runtime::u_string::UString;


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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ColorSpan(")); __s += &(UString::from("start=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.start)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("end=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.end)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("argb=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.argb)).as_str())); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_color_span(a: &ColorSpan, b: &ColorSpan) -> i32 {
    let cmp_start = match i32::from_ne_bytes((*(&a.start)).to_ne_bytes()).cmp(&i32::from_ne_bytes((*(&b.start)).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_start != 0 { return cmp_start; }
    let cmp_end = match i32::from_ne_bytes((*(&a.end)).to_ne_bytes()).cmp(&i32::from_ne_bytes((*(&b.end)).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_end != 0 { return cmp_end; }
    let cmp_argb = match i32::from_ne_bytes((*(&a.argb)).to_ne_bytes()).cmp(&i32::from_ne_bytes((*(&b.argb)).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_argb != 0 { return cmp_argb; }
    0
}
