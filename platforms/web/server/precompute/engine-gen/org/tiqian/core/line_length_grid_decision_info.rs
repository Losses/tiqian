use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineLengthGridDecisionInfo {
    pub enabled: bool,
    pub container_width: f64,
    pub font_size: f64,
    pub cells: u32,
    pub measure: f64,
    pub slack: f64,
    pub body_alignment: UString,
    pub body_offset: f64,
    pub reason: UString,
}

impl LineLengthGridDecisionInfo {
    pub fn new(enabled: bool, container_width: f64, font_size: f64, cells: u32, measure: f64, slack: f64, body_alignment: &UStr, body_offset: f64, reason: &UStr) -> Self {
        Self {
            enabled,
            container_width,
            font_size,
            cells,
            measure,
            slack,
            body_alignment: body_alignment.to_ustring(),
            body_offset,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineLengthGridDecisionInfo(")); __s += &(UString::from("enabled=")); __s += UString::from(format!("{}", (self.enabled).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("containerWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.container_width)); __s += &(UString::from(", ")); __s += &(UString::from("fontSize=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.font_size)); __s += &(UString::from(", ")); __s += &(UString::from("cells=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.cells)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("measure=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.measure)); __s += &(UString::from(", ")); __s += &(UString::from("slack=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.slack)); __s += &(UString::from(", ")); __s += &(UString::from("bodyAlignment=")); __s += (self.body_alignment).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("bodyOffset=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.body_offset)); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
