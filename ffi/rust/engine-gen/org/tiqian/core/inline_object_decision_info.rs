use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectDecisionInfo {
    pub range: TextRange,
    pub advance: f64,
    pub ascent: f64,
    pub descent: f64,
    pub cluster_index: u32,
    pub line_index: u32,
    pub leading_uniform_stretch: bool,
    pub leading_preferred_stretch_kind: Option<String>,
    pub leading_preferred_stretch_natural_width: f64,
    pub leading_preferred_stretch_target_width: f64,
    pub leading_preferred_stretch_capacity: f64,
    pub leading_prevents_line_break: bool,
    pub leading_shrink_capacity: f64,
    pub leading_line_end_discardable_advance: f64,
    pub trailing_uniform_stretch: bool,
    pub trailing_preferred_stretch_kind: Option<String>,
    pub trailing_preferred_stretch_natural_width: f64,
    pub trailing_preferred_stretch_target_width: f64,
    pub trailing_preferred_stretch_capacity: f64,
    pub trailing_prevents_line_break: bool,
    pub trailing_shrink_capacity: f64,
    pub trailing_line_end_discardable_advance: f64,
    pub reason: String,
}

impl InlineObjectDecisionInfo {
    pub fn new(range: TextRange, advance: f64, ascent: f64, descent: f64, cluster_index: u32, line_index: u32, leading_uniform_stretch: Option<bool>, leading_preferred_stretch_kind: Option<String>, leading_preferred_stretch_natural_width: Option<f64>,
leading_preferred_stretch_target_width: Option<f64>, leading_preferred_stretch_capacity: Option<f64>, leading_prevents_line_break: Option<bool>, leading_shrink_capacity: Option<f64>, leading_line_end_discardable_advance: Option<f64>, trailing_uniform_stretch: Option<bool>,
trailing_preferred_stretch_kind: Option<String>, trailing_preferred_stretch_natural_width: Option<f64>, trailing_preferred_stretch_target_width: Option<f64>, trailing_preferred_stretch_capacity: Option<f64>, trailing_prevents_line_break: Option<bool>, trailing_shrink_capacity:
Option<f64>, trailing_line_end_discardable_advance: Option<f64>, reason: Option<String>) -> Self {
        let leading_uniform_stretch = leading_uniform_stretch.unwrap_or_else(|| false);
        let leading_preferred_stretch_kind = leading_preferred_stretch_kind.or_else(|| None);
        let leading_preferred_stretch_natural_width = leading_preferred_stretch_natural_width.unwrap_or_else(|| 0.0);
        let leading_preferred_stretch_target_width = leading_preferred_stretch_target_width.unwrap_or_else(|| 0.0);
        let leading_preferred_stretch_capacity = leading_preferred_stretch_capacity.unwrap_or_else(|| 0.0);
        let leading_prevents_line_break = leading_prevents_line_break.unwrap_or_else(|| false);
        let leading_shrink_capacity = leading_shrink_capacity.unwrap_or_else(|| 0.0);
        let leading_line_end_discardable_advance = leading_line_end_discardable_advance.unwrap_or_else(|| 0.0);
        let trailing_uniform_stretch = trailing_uniform_stretch.unwrap_or_else(|| false);
        let trailing_preferred_stretch_kind = trailing_preferred_stretch_kind.or_else(|| None);
        let trailing_preferred_stretch_natural_width = trailing_preferred_stretch_natural_width.unwrap_or_else(|| 0.0);
        let trailing_preferred_stretch_target_width = trailing_preferred_stretch_target_width.unwrap_or_else(|| 0.0);
        let trailing_preferred_stretch_capacity = trailing_preferred_stretch_capacity.unwrap_or_else(|| 0.0);
        let trailing_prevents_line_break = trailing_prevents_line_break.unwrap_or_else(|| false);
        let trailing_shrink_capacity = trailing_shrink_capacity.unwrap_or_else(|| 0.0);
        let trailing_line_end_discardable_advance = trailing_line_end_discardable_advance.unwrap_or_else(|| 0.0);
        let reason = reason.unwrap_or_else(|| "MeasurableOpaqueInlineObject".to_string());
        Self {
            range,
            advance,
            ascent,
            descent,
            cluster_index,
            line_index,
            leading_uniform_stretch: leading_uniform_stretch,
            leading_preferred_stretch_kind: leading_preferred_stretch_kind,
            leading_preferred_stretch_natural_width: leading_preferred_stretch_natural_width,
            leading_preferred_stretch_target_width: leading_preferred_stretch_target_width,
            leading_preferred_stretch_capacity: leading_preferred_stretch_capacity,
            leading_prevents_line_break: leading_prevents_line_break,
            leading_shrink_capacity: leading_shrink_capacity,
            leading_line_end_discardable_advance: leading_line_end_discardable_advance,
            trailing_uniform_stretch: trailing_uniform_stretch,
            trailing_preferred_stretch_kind: trailing_preferred_stretch_kind,
            trailing_preferred_stretch_natural_width: trailing_preferred_stretch_natural_width,
            trailing_preferred_stretch_target_width: trailing_preferred_stretch_target_width,
            trailing_preferred_stretch_capacity: trailing_preferred_stretch_capacity,
            trailing_prevents_line_break: trailing_prevents_line_break,
            trailing_shrink_capacity: trailing_shrink_capacity,
            trailing_line_end_discardable_advance: trailing_line_end_discardable_advance,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "InlineObjectDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "ascent=",
            self.ascent,
            ", ",
            "descent=",
            self.descent,
            ", ",
            "clusterIndex=",
            crate::runtime::int_text::IntText::int_text(self.cluster_index),
            ", ",
            "lineIndex=",
            crate::runtime::int_text::IntText::int_text(self.line_index),
            ", ",
            "leadingUniformStretch=",
            self.leading_uniform_stretch,
            ", ",
            "leadingPreferredStretchKind=",
            match (self.leading_preferred_stretch_kind).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "leadingPreferredStretchNaturalWidth=",
            self.leading_preferred_stretch_natural_width,
            ", ",
            "leadingPreferredStretchTargetWidth=",
            self.leading_preferred_stretch_target_width,
            ", ",
            "leadingPreferredStretchCapacity=",
            self.leading_preferred_stretch_capacity,
            ", ",
            "leadingPreventsLineBreak=",
            self.leading_prevents_line_break,
            ", ",
            "leadingShrinkCapacity=",
            self.leading_shrink_capacity,
            ", ",
            "leadingLineEndDiscardableAdvance=",
            self.leading_line_end_discardable_advance,
            ", ",
            "trailingUniformStretch=",
            self.trailing_uniform_stretch,
            ", ",
            "trailingPreferredStretchKind=",
            match (self.trailing_preferred_stretch_kind).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "trailingPreferredStretchNaturalWidth=",
            self.trailing_preferred_stretch_natural_width,
            ", ",
            "trailingPreferredStretchTargetWidth=",
            self.trailing_preferred_stretch_target_width,
            ", ",
            "trailingPreferredStretchCapacity=",
            self.trailing_preferred_stretch_capacity,
            ", ",
            "trailingPreventsLineBreak=",
            self.trailing_prevents_line_break,
            ", ",
            "trailingShrinkCapacity=",
            self.trailing_shrink_capacity,
            ", ",
            "trailingLineEndDiscardableAdvance=",
            self.trailing_line_end_discardable_advance,
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}
