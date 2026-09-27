#[derive(Debug, Clone, PartialEq)]
pub struct KinsokuDecisionInfo {
    pub measure_em: f64,
    pub level: String,
    pub hanging: String,
    pub reason: String,
}

impl KinsokuDecisionInfo {
    pub fn new(measure_em: f64, level: &str, hanging: &str, reason: &str) -> Self {
        Self {
            measure_em,
            level: level.to_string(),
            hanging: hanging.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "KinsokuDecisionInfo(",
            "measureEm=",
            self.measure_em,
            ", ",
            "level=",
            (self.level).to_string(),
            ", ",
            "hanging=",
            (self.hanging).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
