use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::sync::Mutex;


pub trait RichTextRole: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn RichTextRole>;
    fn to_string(&self) -> UString;
}

impl Clone for Box<dyn RichTextRole> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn RichTextRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

pub static BACKGROUND_INSTANCE: Mutex<Background> = Mutex::new(Background::new());

#[derive(Clone, PartialEq)]
pub struct Background {
}

impl Background {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from("Background").to_ustring();
    }
}

impl RichTextRole for Background {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextRole.Background"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextRole> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from("Background").to_ustring();
    }
}

pub static UNDERLINE_INSTANCE: Mutex<Underline> = Mutex::new(Underline::new());

#[derive(Clone, PartialEq)]
pub struct Underline {
}

impl Underline {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from("Underline").to_ustring();
    }
}

impl RichTextRole for Underline {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextRole.Underline"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextRole> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from("Underline").to_ustring();
    }
}

pub static LINE_THROUGH_INSTANCE: Mutex<LineThrough> = Mutex::new(LineThrough::new());

#[derive(Clone, PartialEq)]
pub struct LineThrough {
}

impl LineThrough {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from("LineThrough").to_ustring();
    }
}

impl RichTextRole for LineThrough {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextRole.LineThrough"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextRole> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from("LineThrough").to_ustring();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub target: UString,
}

impl Link {
    pub fn new(target: &UStr) -> Self {
        Self {
            target: target.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Link(")); __s += &(UString::from("target=")); __s += (self.target).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

impl RichTextRole for Link {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextRole.Link"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextRole> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Link(")); __s += &(UString::from("target=")); __s += (self.target).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_link(a: &Link, b: &Link) -> i32 {
    let cmp_target = SortedTable::sorted_table_compare_strings(a.target.as_ustr(), b.target.as_ustr());
    if cmp_target != 0 { return cmp_target; }
    0
}

pub static TECHNICAL_INLINE_INSTANCE: Mutex<TechnicalInline> = Mutex::new(TechnicalInline::new());

#[derive(Clone, PartialEq)]
pub struct TechnicalInline {
}

impl TechnicalInline {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from("TechnicalInline").to_ustring();
    }
}

impl RichTextRole for TechnicalInline {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextRole.TechnicalInline"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextRole> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from("TechnicalInline").to_ustring();
    }
}

pub static INLINE_CODE_INSTANCE: Mutex<InlineCode> = Mutex::new(InlineCode::new());

#[derive(Clone, PartialEq)]
pub struct InlineCode {
}

impl InlineCode {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from("InlineCode").to_ustring();
    }
}

impl RichTextRole for InlineCode {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.core.RichTextRole.InlineCode"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn RichTextRole> {
        Box::new(self.clone())
    }

    fn to_string(&self) -> UString {
        return UString::from("InlineCode").to_ustring();
    }
}
