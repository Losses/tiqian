use crate::org::tiqian::clreq::interior_punctuation_style::InteriorPunctuationStyle;


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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "PunctuationWidthPolicy(",
            "interior=",
            self.interior.name(),
            ", ",
            "gbFixedSeparators=",
            self.gb_fixed_separators,
            ")"
        );
    }

    pub fn punctuation_width_policy_same_policy(a: PunctuationWidthPolicy, b: PunctuationWidthPolicy) -> bool {
        return a.interior == b.interior && a.gb_fixed_separators == b.gb_fixed_separators;
    }
}
