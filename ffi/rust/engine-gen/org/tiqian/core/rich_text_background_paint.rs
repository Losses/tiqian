use crate::org::tiqian::core::rich_text_background_draw_style::Border;
use crate::org::tiqian::core::rich_text_background_draw_style::RichTextBackgroundDrawStyle;
use crate::org::tiqian::core::rich_text_background_metric_policy::RichTextBackgroundMetricPolicy;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;


#[derive(Debug, Clone)]
pub struct RichTextBackgroundPaint {
    pub horizontal_padding: f64,
    pub vertical_padding: f64,
    pub corner_radius: f64,
    pub continuation_corner_radius: f64,
    pub metric_policy: RichTextBackgroundMetricPolicy,
    pub draw_style: Box<dyn RichTextBackgroundDrawStyle>,
}

impl RichTextBackgroundPaint {
    pub fn new(horizontal_padding: Option<f64>, vertical_padding: Option<f64>, corner_radius: Option<f64>, continuation_corner_radius: Option<f64>, metric_policy: Option<RichTextBackgroundMetricPolicy>, draw_style: Option<Box<dyn RichTextBackgroundDrawStyle>>) -> Result<Self,
TextRangeError> {
        let horizontal_padding = horizontal_padding.unwrap_or_else(|| 0.0);
        let vertical_padding = vertical_padding.unwrap_or_else(|| 0.0);
        let corner_radius = corner_radius.unwrap_or_else(|| 0.0);
        let continuation_corner_radius = continuation_corner_radius.unwrap_or_else(|| corner_radius);
        let metric_policy = metric_policy.unwrap_or_else(|| RichTextBackgroundMetricPolicy::MarkedFaces);
        let draw_style = draw_style.unwrap_or_else(|| Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone()));
        if !RichTextBackgroundPaint::rich_text_background_paint_is_finite(horizontal_padding) || (horizontal_padding) < (0.0f64) || !RichTextBackgroundPaint::rich_text_background_paint_is_finite(vertical_padding) || (vertical_padding) < (0.0f64) ||
!RichTextBackgroundPaint::rich_text_background_paint_is_finite(corner_radius) || (corner_radius) < (0.0f64) || !RichTextBackgroundPaint::rich_text_background_paint_is_finite(continuation_corner_radius) || (continuation_corner_radius) < (0.0f64) {
            return Err(TextRangeError::Message { text: "Failed requirement.".to_string() });
        }
        Ok(Self {
            horizontal_padding: horizontal_padding,
            vertical_padding: vertical_padding,
            corner_radius: corner_radius,
            continuation_corner_radius: continuation_corner_radius,
            metric_policy: metric_policy,
            draw_style: draw_style,
        })
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RichTextBackgroundPaint(",
            "horizontalPadding=",
            self.horizontal_padding,
            ", ",
            "verticalPadding=",
            self.vertical_padding,
            ", ",
            "cornerRadius=",
            self.corner_radius,
            ", ",
            "continuationCornerRadius=",
            self.continuation_corner_radius,
            ", ",
            "metricPolicy=",
            self.metric_policy.name(),
            ", ",
            "drawStyle=",
            self.draw_style.to_string(),
            ")"
        );
    }

    pub fn rich_text_background_paint_with_horizontal_padding(value: f64) -> Result<RichTextBackgroundPaint, TextRangeError> {
        return Ok(RichTextBackgroundPaint::new(Some(value), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?);
    }

    pub fn rich_text_background_paint_with_metric_policy(value: RichTextBackgroundMetricPolicy) -> Result<RichTextBackgroundPaint, TextRangeError> {
        return Ok(RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(0.0f64), Some(value), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?);
    }

    pub fn rich_text_background_paint_with_corner_radius(corner: f64, continuation: Option<f64>) -> Result<RichTextBackgroundPaint, TextRangeError> {
        return Ok(RichTextBackgroundPaint::new(Some(0.0f64), Some(0.0f64), Some(corner), continuation, Some(RichTextBackgroundMetricPolicy::MarkedFaces), Some(Box::new((*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone())))?);
    }

    pub(crate) fn rich_text_background_paint_same_values(a: RichTextBackgroundPaint, b: RichTextBackgroundPaint) -> bool {
        return a.horizontal_padding == b.horizontal_padding && a.vertical_padding == b.vertical_padding && a.corner_radius == b.corner_radius && a.continuation_corner_radius == b.continuation_corner_radius && a.metric_policy == b.metric_policy &&
RichTextBackgroundPaint::rich_text_background_paint_same_draw_style(a.draw_style.clone(), b.draw_style.clone());
    }

    pub(crate) fn rich_text_background_paint_same_draw_style(a: Box<dyn RichTextBackgroundDrawStyle>, b: Box<dyn RichTextBackgroundDrawStyle>) -> bool {
        if a.__haxe_type_name() == "org.tiqian.core.RichTextBackgroundDrawStyle.Fill" || b.__haxe_type_name() == "org.tiqian.core.RichTextBackgroundDrawStyle.Fill" {
            return a.__haxe_type_name() == "org.tiqian.core.RichTextBackgroundDrawStyle.Fill" && b.__haxe_type_name() == "org.tiqian.core.RichTextBackgroundDrawStyle.Fill" && (*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone().__haxe_type_name() ==
a.__haxe_type_name() && (*crate::org::tiqian::core::rich_text_background_draw_style::FILL_INSTANCE).clone().__haxe_type_name() == b.__haxe_type_name();
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextBackgroundDrawStyle.Border" && b.__haxe_type_name() == "org.tiqian.core.RichTextBackgroundDrawStyle.Border" {
            return ((a).as_any().downcast_ref::<Border>().unwrap()).stroke_width == ((b).as_any().downcast_ref::<Border>().unwrap()).stroke_width;
        }
        return false;
    }

    pub(crate) fn rich_text_background_paint_is_finite(value: f64) -> bool {
        return value == value && value != f64::INFINITY && value != f64::NEG_INFINITY;
    }
}
