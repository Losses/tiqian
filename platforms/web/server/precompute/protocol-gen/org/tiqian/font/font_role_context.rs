use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct FontRoleContext {
    pub locale: String,
    pub region_hint: Option<String>,
}

impl FontRoleContext {
    pub fn new(locale: Option<String>, region_hint: Option<String>) -> Self {
        let locale = locale.unwrap_or_else(|| "zh-Hans".to_string());
        Self {
            locale: locale,
            region_hint: match region_hint { Some(v) => Some(v.to_string()), None => None },
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}",
            "FontRoleContext(",
            "locale=",
            (self.locale).to_string(),
            ", ",
            "regionHint=",
            match (self.region_hint).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

pub fn compare_font_role_context(a: &FontRoleContext, b: &FontRoleContext) -> i32 {
    let cmp_locale = SortedTable::sorted_table_compare_strings(a.locale.as_str(), b.locale.as_str());
    if cmp_locale != 0 { return cmp_locale; }
    let cmp_region_hint = match (&a.region_hint, &b.region_hint) { (None, None) => 0, (None, Some(_)) => -1, (Some(_), None) => 1, (Some(av), Some(bv)) => match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 } };
    if cmp_region_hint != 0 { return cmp_region_hint; }
    0
}

pub trait FontRoleClassifier: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn FontRoleClassifier>;
    fn classify(&self, text: &str, range: TextRange, context: Option<FontRoleContext>) -> FontRole;
}

impl Clone for Box<dyn FontRoleClassifier> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn FontRoleClassifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}
