use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct PunctuationWidthPolicy {
    pub interior: InteriorPunctuationStyle,
    pub gb_fixed_separators: bool,
}

impl PunctuationWidthPolicy {
    pub fn new(interior: Option<InteriorPunctuationStyle>, gb_fixed_separators: Option<bool>) -> Self {
        let interior = interior.unwrap_or_else(|| InteriorPunctuationStyle::FullWidth);
        let gb_fixed_separators = gb_fixed_separators.unwrap_or_else(|| false);
        Self {
            interior: interior,
            gb_fixed_separators: gb_fixed_separators,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("PunctuationWidthPolicy(")); __s += &(UString::from("interior=")); __s += UString::from(self.interior.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("gbFixedSeparators=")); __s += UString::from(format!("{}", (self.gb_fixed_separators).to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn punctuation_width_policy_same_policy(a: PunctuationWidthPolicy, b: PunctuationWidthPolicy) -> bool {
        return a.interior == b.interior && a.gb_fixed_separators == b.gb_fixed_separators;
    }
}
