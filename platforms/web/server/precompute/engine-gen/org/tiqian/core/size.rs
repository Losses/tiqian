use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Size(")); __s += &(UString::from("width=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.width)); __s += &(UString::from(", ")); __s += &(UString::from("height=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.height)); __s += &(UString::from(")")); __s }).as_str());
    }
}
