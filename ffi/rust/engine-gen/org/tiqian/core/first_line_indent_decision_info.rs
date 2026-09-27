#[derive(Debug, Clone, PartialEq)]
pub struct FirstLineIndentDecisionInfo {
    pub source: String,
    pub measure_em: f64,
    pub threshold_em: f64,
    pub resolved_em: f64,
}

impl FirstLineIndentDecisionInfo {
    pub fn new(source: &str, measure_em: f64, threshold_em: f64, resolved_em: f64) -> Self {
        Self {
            source: source.to_string(),
            measure_em,
            threshold_em,
            resolved_em,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "FirstLineIndentDecisionInfo(",
            "source=",
            (self.source).to_string(),
            ", ",
            "measureEm=",
            self.measure_em,
            ", ",
            "thresholdEm=",
            self.threshold_em,
            ", ",
            "resolvedEm=",
            self.resolved_em,
            ")"
        );
    }
}
