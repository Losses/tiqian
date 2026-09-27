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

    fn to_string_value(&self) -> String {
        return format!("{}{}{}",
            "Ic(count=",
            self.0,
            ")"
        );
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
        write!(formatter, "{}", self.to_string_value())
    }
}
