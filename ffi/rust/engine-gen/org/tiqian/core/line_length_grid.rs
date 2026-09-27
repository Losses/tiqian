use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct LineLengthGrid {
    pub enabled: bool,
    pub body_alignment: Option<LastLineAlignment>,
}

impl LineLengthGrid {
    pub fn new(enabled: Option<bool>, body_alignment: Option<LastLineAlignment>) -> Self {
        let enabled = enabled.unwrap_or_else(|| true);
        let body_alignment = body_alignment.or_else(|| None);
        Self {
            enabled: enabled,
            body_alignment: body_alignment,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineLengthGrid(")); __s += &(UString::from("enabled=")); __s += UString::from(format!("{}", (self.enabled).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("bodyAlignment=")); __s += (match &(self.body_alignment) { None => UString::from("null"), Some(__option) => UString::from((*__option).name()) }).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
