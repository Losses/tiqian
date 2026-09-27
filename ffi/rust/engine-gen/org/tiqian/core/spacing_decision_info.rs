use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct SpacingDecisionInfo {
    pub range: TextRange,
    pub left_char: String,
    pub right_char: String,
    pub natural_inner_glue: f64,
    pub adjusted_inner_glue: f64,
    pub reduction: f64,
    pub reduction_target_range: TextRange,
    pub reason: String,
}

impl SpacingDecisionInfo {
    pub fn new(range: TextRange, left_char: &str, right_char: &str, natural_inner_glue: f64, adjusted_inner_glue: f64, reduction: f64, reduction_target_range: TextRange, reason: &str) -> Self {
        Self {
            range,
            left_char: left_char.to_string(),
            right_char: right_char.to_string(),
            natural_inner_glue,
            adjusted_inner_glue,
            reduction,
            reduction_target_range,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "SpacingDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "leftChar=",
            (self.left_char).to_string(),
            ", ",
            "rightChar=",
            (self.right_char).to_string(),
            ", ",
            "naturalInnerGlue=",
            self.natural_inner_glue,
            ", ",
            "adjustedInnerGlue=",
            self.adjusted_inner_glue,
            ", ",
            "reduction=",
            self.reduction,
            ", ",
            "reductionTargetRange=",
            (self.reduction_target_range).clone().to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
