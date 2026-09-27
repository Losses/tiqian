use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use std::sync::LazyLock;


pub trait RichTextBackgroundDrawStyle: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn RichTextBackgroundDrawStyle>;
    fn to_string(&self) -> String;
}

impl Clone for Box<dyn RichTextBackgroundDrawStyle> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn RichTextBackgroundDrawStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

pub static FILL_INSTANCE: LazyLock<Fill> = LazyLock::new(|| Fill::new().unwrap());

#[derive(Clone, PartialEq)]
pub struct Fill {
}

impl Fill {
    pub fn new() -> Result<Self, TextRangeError> {
        Ok(Self {
        })
    }

    pub fn to_string(&self) -> String {
        return "Fill".to_string();
    }
}

impl RichTextBackgroundDrawStyle for Fill {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextBackgroundDrawStyle.Fill"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextBackgroundDrawStyle> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> String {
        return "Fill".to_string();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub stroke_width: f64,
}

impl Border {
    pub fn new(stroke_width: f64) -> Result<Self, TextRangeError> {
        if !Border::border_is_finite(stroke_width) || (stroke_width) <= 0.0f64 {
            return Err(TextRangeError::Message { text: "Failed requirement.".to_string() });
        }
        Ok(Self {
            stroke_width,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}",
            "Border(",
            "strokeWidth=",
            self.stroke_width,
            ")"
        );
    }

    pub(crate) fn border_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}

impl RichTextBackgroundDrawStyle for Border {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextBackgroundDrawStyle.Border"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextBackgroundDrawStyle> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> String {
        return format!("{}{}{}{}",
            "Border(",
            "strokeWidth=",
            self.stroke_width,
            ")"
        );
    }
}
