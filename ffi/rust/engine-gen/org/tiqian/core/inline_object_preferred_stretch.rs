use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectPreferredStretch {
    pub kind: InlineObjectPreferredStretchKind,
    pub natural_width: f64,
    pub target_width: f64,
}

impl InlineObjectPreferredStretch {
    pub fn new(kind: InlineObjectPreferredStretchKind, natural_width: f64, target_width: f64) -> Result<Self, TextRangeError> {
        if !InlineObjectPreferredStretch::inline_object_preferred_stretch_is_finite(natural_width) || (natural_width) < (0.0f64) {
            return Err(TextRangeError::Message { text: "Inline-object preferred stretch natural width must be finite and non-negative".to_string() });
        }
        if !InlineObjectPreferredStretch::inline_object_preferred_stretch_is_finite(target_width) || (target_width) <= natural_width {
            return Err(TextRangeError::Message { text: "Inline-object preferred stretch target must be finite and exceed its natural width".to_string() });
        }
        Ok(Self {
            kind,
            natural_width,
            target_width,
        })
    }

    pub fn get_capacity(&self) -> f64 {
        return self.target_width - self.natural_width;
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "InlineObjectPreferredStretch(",
            "kind=",
            self.kind.name(),
            ", ",
            "naturalWidth=",
            self.natural_width,
            ", ",
            "targetWidth=",
            self.target_width,
            ")"
        );
    }

    pub(crate) fn inline_object_preferred_stretch_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}
