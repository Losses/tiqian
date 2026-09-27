use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::runtime::sorted_table::SortedTable;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct RubySpan {
    pub base_range: TextRange,
    pub text: String,
    pub font_families: Vec<String>,
    pub kind: RubyKind,
    pub locale: Option<String>,
}

impl RubySpan {
    pub fn new(base_range: TextRange, text: &str, font_families: Option<Vec<String>>, kind: RubyKind, locale: Option<String>) -> Self {
        let font_families = font_families.unwrap_or_else(|| vec![]);
        Self {
            base_range,
            text: text.to_string(),
            font_families: font_families,
            kind,
            locale: match locale { Some(v) => Some(v.to_string()), None => None },
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "RubySpan(",
            "baseRange=",
            (self.base_range).clone().to_string(),
            ", ",
            "text=",
            (self.text).to_string(),
            ", ",
            "fontFamilies=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.font_families).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "kind=",
            self.kind.name(),
            ", ",
            "locale=",
            match (self.locale).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}

fn ruby_span_kind_order(v: &RubyKind) -> i32 {
    match v {
        RubyKind::Pinyin => 0,
        RubyKind::Bopomofo => 1,
    }
}
pub fn compare_ruby_span(a: &RubySpan, b: &RubySpan) -> i32 {
    let cmp_base_range = compare_text_range(&a.base_range, &b.base_range);
    if cmp_base_range != 0 { return cmp_base_range; }
    let cmp_text = SortedTable::sorted_table_compare_strings(a.text.as_str(), b.text.as_str());
    if cmp_text != 0 { return cmp_text; }
    let mut cmp_font_families = 0; for (av, bv) in a.font_families.iter().zip(b.font_families.iter()) { cmp_font_families = match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; if cmp_font_families != 0 { break;
} }
    if cmp_font_families == 0 { cmp_font_families = match a.font_families.len().cmp(&b.font_families.len()) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; }
    if cmp_font_families != 0 { return cmp_font_families; }
    let cmp_kind = match ruby_span_kind_order(&a.kind).cmp(&ruby_span_kind_order(&b.kind)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_kind != 0 { return cmp_kind; }
    let cmp_locale = match (&a.locale, &b.locale) { (None, None) => 0, (None, Some(_)) => -1, (Some(_), None) => 1, (Some(av), Some(bv)) => match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 } };
    if cmp_locale != 0 { return cmp_locale; }
    0
}
