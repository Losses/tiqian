use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct EastAsianSpacingEdges {
    pub leading: EastAsianSpacingValue,
    pub trailing: EastAsianSpacingValue,
    pub contains_wide: bool,
}

impl EastAsianSpacingEdges {
    pub fn new(leading: EastAsianSpacingValue, trailing: EastAsianSpacingValue, contains_wide: bool) -> Self {
        Self {
            leading,
            trailing,
            contains_wide,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("EastAsianSpacingEdges(")); __s += &(UString::from("leading=")); __s += UString::from(self.leading.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trailing=")); __s += UString::from(self.trailing.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("containsWide=")); __s += UString::from(format!("{}", (self.contains_wide).to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
