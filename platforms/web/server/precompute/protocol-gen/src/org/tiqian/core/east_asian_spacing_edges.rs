use crate::org::tiqian::core::east_asian_spacing_value::EastAsianSpacingValue;


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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "EastAsianSpacingEdges(",
            "leading=",
            self.leading.name(),
            ", ",
            "trailing=",
            self.trailing.name(),
            ", ",
            "containsWide=",
            self.contains_wide,
            ")"
        );
    }
}
