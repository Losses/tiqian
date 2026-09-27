#[derive(Debug, Clone, PartialEq)]
pub struct LineSpacingDecisionInfo {
    pub natural_height: f64,
    pub requested_line_height: Option<f64>,
    pub resolved_height: f64,
    pub spacing_floor: f64,
    pub floor_applied: bool,
    pub reason: String,
}

impl LineSpacingDecisionInfo {
    pub fn new(natural_height: f64, requested_line_height: Option<f64>, resolved_height: f64, spacing_floor: f64, floor_applied: bool, reason: &str) -> Self {
        Self {
            natural_height,
            requested_line_height,
            resolved_height,
            spacing_floor,
            floor_applied,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineSpacingDecisionInfo(",
            "naturalHeight=",
            self.natural_height,
            ", ",
            "requestedLineHeight=",
            match self.requested_line_height { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "resolvedHeight=",
            self.resolved_height,
            ", ",
            "spacingFloor=",
            self.spacing_floor,
            ", ",
            "floorApplied=",
            self.floor_applied,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
