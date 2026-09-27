use crate::org::tiqian::core::rich_text_paint::RichTextPaint;
use crate::org::tiqian::core::rich_text_role::Link;
use crate::org::tiqian::core::rich_text_role::RichTextRole;
use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone)]
pub struct RichTextSpan {
    pub range: TextRange,
    pub role: Box<dyn RichTextRole>,
    pub paint: RichTextPaint,
}

impl RichTextSpan {
    pub fn new(range: TextRange, role: Box<dyn RichTextRole>, paint: RichTextPaint) -> Self {
        Self {
            range,
            role,
            paint,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "RichTextSpan(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "role=",
            self.role.to_string(),
            ", ",
            "paint=",
            (self.paint).clone().to_string(),
            ")"
        );
    }

    pub(crate) fn rich_text_span_same_role(a: Box<dyn RichTextRole>, b: Box<dyn RichTextRole>) -> bool {
        if a.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" && b.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" {
            return (((a).as_any().downcast_ref::<Link>().unwrap()).target).to_string() == (((b).as_any().downcast_ref::<Link>().unwrap()).target).to_string();
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextRole.Background" {
            return b.__haxe_type_name() == "org.tiqian.core.RichTextRole.Background";
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextRole.Underline" {
            return b.__haxe_type_name() == "org.tiqian.core.RichTextRole.Underline";
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextRole.LineThrough" {
            return b.__haxe_type_name() == "org.tiqian.core.RichTextRole.LineThrough";
        }
        if a.__haxe_type_name() == "org.tiqian.core.RichTextRole.TechnicalInline" {
            return b.__haxe_type_name() == "org.tiqian.core.RichTextRole.TechnicalInline";
        }
        return a.__haxe_type_name() == "org.tiqian.core.RichTextRole.InlineCode" && b.__haxe_type_name() == "org.tiqian.core.RichTextRole.InlineCode";
    }
}
