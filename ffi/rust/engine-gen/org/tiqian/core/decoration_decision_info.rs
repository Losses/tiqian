use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct DecorationDecisionInfo {
    pub cluster_range: TextRange,
    pub source_text: String,
    pub kind: String,
    pub applied: bool,
    pub reason: String,
    pub anchor_x: f64,
    pub anchor_y: f64,
    pub dot_diameter: f64,
}

impl DecorationDecisionInfo {
    pub fn new(cluster_range: TextRange, source_text: &str, kind: &str, applied: bool, reason: &str, anchor_x: Option<f64>, anchor_y: Option<f64>, dot_diameter: Option<f64>) -> Self {
        let anchor_x = anchor_x.unwrap_or_else(|| 0.0);
        let anchor_y = anchor_y.unwrap_or_else(|| 0.0);
        let dot_diameter = dot_diameter.unwrap_or_else(|| 0.0);
        Self {
            cluster_range,
            source_text: source_text.to_string(),
            kind: kind.to_string(),
            applied,
            reason: reason.to_string(),
            anchor_x: anchor_x,
            anchor_y: anchor_y,
            dot_diameter: dot_diameter,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "DecorationDecisionInfo(",
            "clusterRange=",
            (self.cluster_range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "kind=",
            (self.kind).to_string(),
            ", ",
            "applied=",
            self.applied,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ", ",
            "anchorX=",
            self.anchor_x,
            ", ",
            "anchorY=",
            self.anchor_y,
            ", ",
            "dotDiameter=",
            self.dot_diameter,
            ")"
        );
    }
}
