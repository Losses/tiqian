use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct RichTextCornerRadii {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

impl RichTextCornerRadii {
    pub fn new(top_left: f64, top_right: f64, bottom_right: f64, bottom_left: f64) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    pub fn get_is_square(&self) -> bool {
        return self.top_left == 0.0f64 && self.top_right == 0.0f64 && self.bottom_right == 0.0f64 && self.bottom_left == 0.0f64;
    }

    pub fn get_is_uniform(&self) -> bool {
        return self.top_left == self.top_right && self.top_right == self.bottom_right && self.bottom_right == self.bottom_left;
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RichTextCornerRadii(")); __s += &(UString::from("topLeft=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top_left)); __s += &(UString::from(", ")); __s += &(UString::from("topRight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.top_right)); __s += &(UString::from(", ")); __s += &(UString::from("bottomRight=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.bottom_right)); __s += &(UString::from(", ")); __s += &(UString::from("bottomLeft=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.bottom_left)); __s += &(UString::from(")")); __s }).as_str());
    }
}
