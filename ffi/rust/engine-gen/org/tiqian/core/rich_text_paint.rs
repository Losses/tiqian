use crate::org::tiqian::core::rich_text_background_metric_policy::RichTextBackgroundMetricPolicy;
use crate::org::tiqian::core::rich_text_background_paint::RichTextBackgroundPaint;
use crate::org::tiqian::core::rich_text_line_pattern::Dashed;
use crate::org::tiqian::core::rich_text_line_pattern::Dotted;
use crate::org::tiqian::core::rich_text_line_pattern::RichTextLinePattern;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone)]
pub struct RichTextPaint {
    pub argb: Option<u32>,
    pub line_pattern: Box<dyn RichTextLinePattern>,
    pub background: RichTextBackgroundPaint,
    pub adjacent_same_style_clearance: f64,
}

impl RichTextPaint {
    pub fn new(argb: Option<u32>, line_pattern: Option<Box<dyn RichTextLinePattern>>, background: RichTextBackgroundPaint, adjacent_same_style_clearance: Option<f64>) -> Result<Self, TextRangeError> {
        let argb = argb.or_else(|| None);
        let line_pattern = line_pattern.unwrap_or_else(|| Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone()));
        let adjacent_same_style_clearance = adjacent_same_style_clearance.unwrap_or_else(|| 0.0);
        if !RichTextPaint::rich_text_paint_is_finite(adjacent_same_style_clearance) || (adjacent_same_style_clearance) < (0.0f64) {
            return Err(TextRangeError::Message { text: UString::from("Failed requirement.") });
        }
        Ok(Self {
            argb: argb,
            line_pattern: line_pattern,
            background,
            adjacent_same_style_clearance: adjacent_same_style_clearance,
        })
    }

    pub fn same_visible_style(&self, other: RichTextPaint) -> bool {
        let a = self.argb;
        let b = other.argb;
        if a != b {
            return false;
        }
        if !RichTextPaint::rich_text_paint_same_line_pattern((self.line_pattern).clone(), other.line_pattern.clone()) {
            return false;
        }
        return RichTextBackgroundPaint::rich_text_background_paint_same_values((self.background).clone(), (other.background).clone());
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RichTextPaint(")); __s += &(UString::from("argb=")); __s += &(match self.argb { Some(v) => UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("linePattern=")); __s += UString::from(format!("{}", self.line_pattern.to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("background=")); __s += UString::from(format!("{}", (self.background).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("adjacentSameStyleClearance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.adjacent_same_style_clearance)); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn rich_text_paint_with_background(background: RichTextBackgroundPaint) -> Result<RichTextPaint, TextRangeError> {
        return Ok(RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), (background).clone(), Some(0.0f64))?);
    }

    pub fn rich_text_paint_with_clearance(clearance: f64) -> Result<RichTextPaint, TextRangeError> {
        return Ok(RichTextPaint::new(None, Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?, Some(clearance))?);
    }

    pub fn rich_text_paint_with_argb(value: u32) -> Result<RichTextPaint, TextRangeError> {
        return Ok(RichTextPaint::new(Some(value), Some(Box::new((*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone())), RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?, Some(0.0f64))?);
    }

    pub(crate) fn rich_text_paint_same_line_pattern(a: Box<dyn RichTextLinePattern>, b: Box<dyn RichTextLinePattern>) -> bool {
        if a.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Solid" || b.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Solid" {
            return a.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Solid" && b.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Solid" && (*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone().__haxe_type_name() == a.__haxe_type_name() && (*crate::org::tiqian::core::rich_text_line_pattern::SOLID_INSTANCE).clone().__haxe_type_name() == b.__haxe_type_name();
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Dashed" && b.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Dashed" {
            let aa = ((a).as_any().downcast_ref::<Dashed>().unwrap()).clone();
            let bb = ((b).as_any().downcast_ref::<Dashed>().unwrap()).clone();
            return aa.stroke_width == bb.stroke_width && aa.dash_length == bb.dash_length && aa.gap_length == bb.gap_length;
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Dotted" && b.__haxe_type_name() == "org.tiqian.core.RichTextLinePattern.Dotted" {
            let aa = ((a).as_any().downcast_ref::<Dotted>().unwrap()).clone();
            let bb = ((b).as_any().downcast_ref::<Dotted>().unwrap()).clone();
            return aa.dot_diameter == bb.dot_diameter && aa.gap_length == bb.gap_length;
        }
        return false;
    }

    pub(crate) fn rich_text_paint_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}
