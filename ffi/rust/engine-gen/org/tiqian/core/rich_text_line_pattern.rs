use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use std::sync::LazyLock;


pub trait RichTextLinePattern: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn RichTextLinePattern>;
    fn to_string(&self) -> String;
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

    pub fn to_string(&self) -> String {
        return "Solid".to_string();
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

    fn to_string(&self) -> String {
        return "Solid".to_string();
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
            return Err(TextRangeError::Message { text: "Failed requirement.".to_string() });
        }
        Ok(Self {
            stroke_width,
            dash_length,
            gap_length,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "Dashed(",
            "strokeWidth=",
            self.stroke_width,
            ", ",
            "dashLength=",
            self.dash_length,
            ", ",
            "gapLength=",
            self.gap_length,
            ")"
        );
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

    fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "Dashed(",
            "strokeWidth=",
            self.stroke_width,
            ", ",
            "dashLength=",
            self.dash_length,
            ", ",
            "gapLength=",
            self.gap_length,
            ")"
        );
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
            return Err(TextRangeError::Message { text: "Failed requirement.".to_string() });
        }
        Ok(Self {
            dot_diameter,
            gap_length,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "Dotted(",
            "dotDiameter=",
            self.dot_diameter,
            ", ",
            "gapLength=",
            self.gap_length,
            ")"
        );
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

    fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "Dotted(",
            "dotDiameter=",
            self.dot_diameter,
            ", ",
            "gapLength=",
            self.gap_length,
            ")"
        );
    }
}
