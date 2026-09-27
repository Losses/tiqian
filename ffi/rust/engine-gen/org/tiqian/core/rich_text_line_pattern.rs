use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;
use std::sync::LazyLock;


pub trait RichTextLinePattern: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn RichTextLinePattern>;
    fn to_string(&self) -> UString;
}

impl Clone for Box<dyn RichTextLinePattern> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn RichTextLinePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

pub static SOLID_INSTANCE: LazyLock<Solid> = LazyLock::new(|| Solid::new().unwrap());

#[derive(Clone, PartialEq)]
pub struct Solid {
}

impl Solid {
    pub fn new() -> Result<Self, TextRangeError> {
        Ok(Self {
        })
    }

    pub fn to_string(&self) -> UString {
        return UString::from("Solid").to_ustring();
    }
}

impl RichTextLinePattern for Solid {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextLinePattern.Solid"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextLinePattern> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from("Solid").to_ustring();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dashed {
    pub stroke_width: f64,
    pub dash_length: f64,
    pub gap_length: f64,
}

impl Dashed {
    pub fn new(stroke_width: f64, dash_length: f64, gap_length: f64) -> Result<Self, TextRangeError> {
        if !Dashed::dashed_is_finite(stroke_width) || (stroke_width) <= 0.0f64 || !Dashed::dashed_is_finite(dash_length) || (dash_length) <= 0.0f64 || !Dashed::dashed_is_finite(gap_length) || (gap_length) <= 0.0f64 {
            return Err(TextRangeError::Message { text: UString::from("Failed requirement.") });
        }
        Ok(Self {
            stroke_width,
            dash_length,
            gap_length,
        })
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Dashed(")); __s += &(UString::from("strokeWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.stroke_width)); __s += &(UString::from(", ")); __s += &(UString::from("dashLength=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.dash_length)); __s += &(UString::from(", ")); __s += &(UString::from("gapLength=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.gap_length)); __s += &(UString::from(")")); __s }).as_str());
    }

    pub(crate) fn dashed_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}

impl RichTextLinePattern for Dashed {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextLinePattern.Dashed"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextLinePattern> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Dashed(")); __s += &(UString::from("strokeWidth=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.stroke_width)); __s += &(UString::from(", ")); __s += &(UString::from("dashLength=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.dash_length)); __s += &(UString::from(", ")); __s += &(UString::from("gapLength=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.gap_length)); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dotted {
    pub dot_diameter: f64,
    pub gap_length: f64,
}

impl Dotted {
    pub fn new(dot_diameter: f64, gap_length: f64) -> Result<Self, TextRangeError> {
        if !Dotted::dotted_is_finite(dot_diameter) || (dot_diameter) <= 0.0f64 || !Dotted::dotted_is_finite(gap_length) || (gap_length) <= 0.0f64 {
            return Err(TextRangeError::Message { text: UString::from("Failed requirement.") });
        }
        Ok(Self {
            dot_diameter,
            gap_length,
        })
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Dotted(")); __s += &(UString::from("dotDiameter=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.dot_diameter)); __s += &(UString::from(", ")); __s += &(UString::from("gapLength=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.gap_length)); __s += &(UString::from(")")); __s }).as_str());
    }

    pub(crate) fn dotted_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}

impl RichTextLinePattern for Dotted {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextLinePattern.Dotted"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextLinePattern> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Dotted(")); __s += &(UString::from("dotDiameter=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.dot_diameter)); __s += &(UString::from(", ")); __s += &(UString::from("gapLength=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.gap_length)); __s += &(UString::from(")")); __s }).as_str());
    }
}
