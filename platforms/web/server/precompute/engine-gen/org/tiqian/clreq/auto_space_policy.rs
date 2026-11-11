use crate::org::tiqian::clreq::auto_space_mode::AutoSpaceMode;
use crate::runtime::u_string::UString;
use std::sync::LazyLock;


pub static AUTO_SPACE_POLICY_DEFAULT: LazyLock<AutoSpacePolicy> = LazyLock::new(|| AutoSpacePolicy::new(Some(AutoSpaceMode::Insert), Some(AutoSpaceMode::Insert), Some(0.125), Some(1.0 / 3.0)));

pub static AUTO_SPACE_POLICY_CLREQ: LazyLock<AutoSpacePolicy> = LazyLock::new(|| AutoSpacePolicy::new(Some(AutoSpaceMode::Insert), Some(AutoSpaceMode::Insert), Some(0.25f64), Some(0.5f64)));

pub static AUTO_SPACE_POLICY_DISABLED: LazyLock<AutoSpacePolicy> = LazyLock::new(|| AutoSpacePolicy::new(Some(AutoSpaceMode::Disabled), Some(AutoSpaceMode::Disabled), Some(0.125), Some(1.0 / 3.0)));

#[derive(Debug, Clone, PartialEq)]
pub struct AutoSpacePolicy {
    pub cjk_latin: AutoSpaceMode,
    pub cjk_digit: AutoSpaceMode,
    pub gap_em: f64,
    pub stretch_max_em: f64,
}

impl AutoSpacePolicy {
    pub fn new(cjk_latin: Option<AutoSpaceMode>, cjk_digit: Option<AutoSpaceMode>, gap_em: Option<f64>, stretch_max_em: Option<f64>) -> Self {
        let cjk_latin = cjk_latin.unwrap_or_else(|| AutoSpaceMode::Insert);
        let cjk_digit = cjk_digit.unwrap_or_else(|| AutoSpaceMode::Insert);
        let gap_em = gap_em.unwrap_or_else(|| 0.125);
        let stretch_max_em = stretch_max_em.unwrap_or_else(|| 1.0 / 3.0);
        Self {
            cjk_latin: cjk_latin,
            cjk_digit: cjk_digit,
            gap_em: gap_em,
            stretch_max_em: stretch_max_em,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("AutoSpacePolicy(")); __s += &(UString::from("cjkLatin=")); __s += UString::from(self.cjk_latin.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("cjkDigit=")); __s += UString::from(self.cjk_digit.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("gapEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.gap_em)); __s += &(UString::from(", ")); __s += &(UString::from("stretchMaxEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.stretch_max_em)); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn auto_space_policy_same_policy(a: AutoSpacePolicy, b: AutoSpacePolicy) -> bool {
        return a.cjk_latin == b.cjk_latin && a.cjk_digit == b.cjk_digit && a.gap_em == b.gap_em && a.stretch_max_em == b.stretch_max_em;
    }
}
