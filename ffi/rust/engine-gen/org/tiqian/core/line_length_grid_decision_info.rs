#[derive(Debug, Clone, PartialEq)]
pub struct LineLengthGridDecisionInfo {
    pub enabled: bool,
    pub container_width: f64,
    pub font_size: f64,
    pub cells: u32,
    pub measure: f64,
    pub slack: f64,
    pub body_alignment: String,
    pub body_offset: f64,
    pub reason: String,
}

impl LineLengthGridDecisionInfo {
    pub fn new(enabled: bool, container_width: f64, font_size: f64, cells: u32, measure: f64, slack: f64, body_alignment: &str, body_offset: f64, reason: &str) -> Self {
        Self {
            enabled,
            container_width,
            font_size,
            cells,
            measure,
            slack,
            body_alignment: body_alignment.to_string(),
            body_offset,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineLengthGridDecisionInfo(",
            "enabled=",
            self.enabled,
            ", ",
            "containerWidth=",
            self.container_width,
            ", ",
            "fontSize=",
            self.font_size,
            ", ",
            "cells=",
            crate::runtime::int_text::IntText::int_text(self.cells),
            ", ",
            "measure=",
            self.measure,
            ", ",
            "slack=",
            self.slack,
            ", ",
            "bodyAlignment=",
            (self.body_alignment).to_string(),
            ", ",
            "bodyOffset=",
            self.body_offset,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
