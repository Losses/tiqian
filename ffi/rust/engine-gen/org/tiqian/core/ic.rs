use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct Ic(pub f64);

impl Ic {
    pub fn new(value: f64) -> Self {
        return Self(value);
    }

    pub fn count(&self) -> f64 {
        return self.0;
    }

    pub fn to_px(&self, em_px: f64) -> f64 {
        return self.0 * em_px;
    }

    fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Ic(count=")); __s += UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(self.0)).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn zero() -> Ic { Ic(0.0f64) }
}

impl std::ops::Add for Ic {
    type Output = Ic;
    fn add(self, rhs: Ic) -> Ic {
        return Ic(self.0 + rhs.0);
    }
}

impl std::ops::Neg for Ic {
    type Output = Ic;
    fn neg(self) -> Ic {
        return Ic(-self.0);
    }
}

impl std::fmt::Display for Ic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}
