use crate::runtime::u_string::UString;


#[derive(Clone, PartialEq)]
pub struct IntRange {
    pub start: u32,
    pub end: u32,
}

impl IntRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn get_is_empty(&self) -> bool {
        return (i32::from_ne_bytes(((self.start) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((self.end) as i32).to_ne_bytes()));
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.start)).as_str())); __s += &(UString::from("..")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.end)).as_str())); __s }).as_str());
    }
}
