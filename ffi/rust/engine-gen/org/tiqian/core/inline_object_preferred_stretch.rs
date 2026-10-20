use crate::org::tiqian::core::inline_object_preferred_stretch_kind::InlineObjectPreferredStretchKind;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct InlineObjectPreferredStretch {
    pub kind: InlineObjectPreferredStretchKind,
    pub natural_width: f64,
    pub target_width: f64,
}

impl InlineObjectPreferredStretch {
    pub fn new(kind: InlineObjectPreferredStretchKind, natural_width: f64, target_width: f64) -> Result<Self, TextRangeError> {
        if !InlineObjectPreferredStretch::inline_object_preferred_stretch_is_finite(natural_width) || (natural_width) < (0.0f64) {
            return Err(TextRangeError::Message { text: UString::from("Inline-object preferred stretch natural width must be finite and non-negative") });
        }
        if !InlineObjectPreferredStretch::inline_object_preferred_stretch_is_finite(target_width) || (target_width) <= natural_width {
            return Err(TextRangeError::Message { text: UString::from("Inline-object preferred stretch target must be finite and exceed its natural width") });
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("InlineObjectPreferredStretch(")); __s += &(UString::from("kind=")); __s += UString::from(self.kind.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("naturalWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.natural_width)); __s += &(UString::from(", ")); __s += &(UString::from("targetWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.target_width)); __s += &(UString::from(")")); __s }).as_str());
    }

    pub(crate) fn inline_object_preferred_stretch_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}
