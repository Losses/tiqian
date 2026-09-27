use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct MeasureAdaptiveFirstLineIndent {
    pub short_below_em: f64,
    pub short_em: f64,
    pub long_em: f64,
}

impl MeasureAdaptiveFirstLineIndent {
    pub fn new(short_below_em: Option<f64>, short_em: Option<f64>, long_em: Option<f64>) -> Self {
        let short_below_em = short_below_em.unwrap_or_else(|| 14.0);
        let short_em = short_em.unwrap_or_else(|| 1.0);
        let long_em = long_em.unwrap_or_else(|| 2.0);
        Self {
            short_below_em: short_below_em,
            short_em: short_em,
            long_em: long_em,
        }
    }

    pub fn resolve_em(&self, measure_em: f64) -> f64 {
        return if measure_em < (self.short_below_em) { self.short_em } else { self.long_em };
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("MeasureAdaptiveFirstLineIndent(")); __s += &(UString::from("shortBelowEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.short_below_em)); __s += &(UString::from(", ")); __s += &(UString::from("shortEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.short_em)); __s += &(UString::from(", ")); __s += &(UString::from("longEm=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.long_em)); __s += &(UString::from(")")); __s }).as_str());
    }
}
