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
        return (i32::from_ne_bytes((self.start).to_ne_bytes())) > (i32::from_ne_bytes((self.end).to_ne_bytes()));
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}",
            crate::runtime::int_text::IntText::int_text(self.start),
            "..",
            crate::runtime::int_text::IntText::int_text(self.end)
        );
    }
}
