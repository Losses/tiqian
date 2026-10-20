use crate::org::tiqian::core::rich_text_paint::RichTextPaint;
use crate::org::tiqian::core::rich_text_role::Link;
use crate::org::tiqian::core::rich_text_role::RichTextRole;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UString;


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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RichTextSpan(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += UString::from(format!("{}", self.role.to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("paint=")); __s += UString::from(format!("{}", (self.paint).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub(crate) fn rich_text_span_same_role(a: Box<dyn RichTextRole>, b: Box<dyn RichTextRole>) -> bool {
        if a.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" && b.__haxe_type_name() == "org.tiqian.core.RichTextRole.Link" {
            return (((a).as_any().downcast_ref::<Link>().unwrap()).target).to_ustring() == (((b).as_any().downcast_ref::<Link>().unwrap()).target).to_ustring();
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
