use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectDecisionInfo {
    pub range: TextRange,
    pub advance: f64,
    pub ascent: f64,
    pub descent: f64,
    pub cluster_index: u32,
    pub line_index: u32,
    pub leading_uniform_stretch: bool,
    pub leading_preferred_stretch_kind: Option<UString>,
    pub leading_preferred_stretch_natural_width: f64,
    pub leading_preferred_stretch_target_width: f64,
    pub leading_preferred_stretch_capacity: f64,
    pub leading_prevents_line_break: bool,
    pub leading_shrink_capacity: f64,
    pub leading_line_end_discardable_advance: f64,
    pub trailing_uniform_stretch: bool,
    pub trailing_preferred_stretch_kind: Option<UString>,
    pub trailing_preferred_stretch_natural_width: f64,
    pub trailing_preferred_stretch_target_width: f64,
    pub trailing_preferred_stretch_capacity: f64,
    pub trailing_prevents_line_break: bool,
    pub trailing_shrink_capacity: f64,
    pub trailing_line_end_discardable_advance: f64,
    pub reason: UString,
}

impl InlineObjectDecisionInfo {
    pub fn new(range: TextRange, advance: f64, ascent: f64, descent: f64, cluster_index: u32, line_index: u32, leading_uniform_stretch: Option<bool>, leading_preferred_stretch_kind: Option<UString>, leading_preferred_stretch_natural_width: Option<f64>, leading_preferred_stretch_target_width: Option<f64>, leading_preferred_stretch_capacity: Option<f64>, leading_prevents_line_break: Option<bool>, leading_shrink_capacity: Option<f64>, leading_line_end_discardable_advance: Option<f64>, trailing_uniform_stretch: Option<bool>, trailing_preferred_stretch_kind: Option<UString>, trailing_preferred_stretch_natural_width: Option<f64>, trailing_preferred_stretch_target_width: Option<f64>, trailing_preferred_stretch_capacity: Option<f64>, trailing_prevents_line_break: Option<bool>, trailing_shrink_capacity: Option<f64>, trailing_line_end_discardable_advance: Option<f64>, reason: Option<UString>) -> Self {
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
        let reason = reason.unwrap_or_else(|| UString::from("MeasurableOpaqueInlineObject"));
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineObjectDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("clusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.cluster_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("lineIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.line_index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("leadingUniformStretch=")); __s += UString::from(format!("{}", (self.leading_uniform_stretch).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("leadingPreferredStretchKind=")); __s += match &((self.leading_preferred_stretch_kind).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("leadingPreferredStretchNaturalWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_preferred_stretch_natural_width)); __s += &(UString::from(", ")); __s += &(UString::from("leadingPreferredStretchTargetWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_preferred_stretch_target_width)); __s += &(UString::from(", ")); __s += &(UString::from("leadingPreferredStretchCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_preferred_stretch_capacity)); __s += &(UString::from(", ")); __s += &(UString::from("leadingPreventsLineBreak=")); __s += UString::from(format!("{}", (self.leading_prevents_line_break).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("leadingShrinkCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_shrink_capacity)); __s += &(UString::from(", ")); __s += &(UString::from("leadingLineEndDiscardableAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading_line_end_discardable_advance)); __s += &(UString::from(", ")); __s += &(UString::from("trailingUniformStretch=")); __s += UString::from(format!("{}", (self.trailing_uniform_stretch).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trailingPreferredStretchKind=")); __s += match &((self.trailing_preferred_stretch_kind).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("trailingPreferredStretchNaturalWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_preferred_stretch_natural_width)); __s += &(UString::from(", ")); __s += &(UString::from("trailingPreferredStretchTargetWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_preferred_stretch_target_width)); __s += &(UString::from(", ")); __s += &(UString::from("trailingPreferredStretchCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_preferred_stretch_capacity)); __s += &(UString::from(", ")); __s += &(UString::from("trailingPreventsLineBreak=")); __s += UString::from(format!("{}", (self.trailing_prevents_line_break).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("trailingShrinkCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_shrink_capacity)); __s += &(UString::from(", ")); __s += &(UString::from("trailingLineEndDiscardableAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.trailing_line_end_discardable_advance)); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
