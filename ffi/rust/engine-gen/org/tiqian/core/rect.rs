use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Rect {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn get_width(&self) -> f64 {
        return self.right - self.left;
    }

    pub fn get_height(&self) -> f64 {
        return self.bottom - self.top;
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Rect(left=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.left)); __s += &(UString::from(", top=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top)); __s += &(UString::from(", right=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.right)); __s += &(UString::from(", bottom=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.bottom)); __s += &(UString::from(")")); __s }).as_str());
    }
}
