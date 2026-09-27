use crate::runtime::sorted_table::SortedTable;
use std::sync::Mutex;


pub trait RichTextRole: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn RichTextRole>;
    fn to_string(&self) -> String;
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

    pub fn to_string(&self) -> String {
        return "Background".to_string();
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

    fn to_string(&self) -> String {
        return "Background".to_string();
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

    pub fn to_string(&self) -> String {
        return "Underline".to_string();
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

    fn to_string(&self) -> String {
        return "Underline".to_string();
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

    pub fn to_string(&self) -> String {
        return "LineThrough".to_string();
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

    fn to_string(&self) -> String {
        return "LineThrough".to_string();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub target: String,
}

impl Link {
    pub fn new(target: &str) -> Self {
        Self {
            target: target.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}",
            "Link(",
            "target=",
            (self.target).to_string(),
            ")"
        );
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

    fn to_string(&self) -> String {
        return format!("{}{}{}{}",
            "Link(",
            "target=",
            (self.target).to_string(),
            ")"
        );
    }
}

pub fn compare_link(a: &Link, b: &Link) -> i32 {
    let cmp_target = SortedTable::sorted_table_compare_strings(a.target.as_str(), b.target.as_str());
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

    pub fn to_string(&self) -> String {
        return "TechnicalInline".to_string();
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

    fn to_string(&self) -> String {
        return "TechnicalInline".to_string();
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

    pub fn to_string(&self) -> String {
        return "InlineCode".to_string();
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

    fn to_string(&self) -> String {
        return "InlineCode".to_string();
    }
}
