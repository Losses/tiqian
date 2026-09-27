use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::sorted_table::SortedTable;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct FontRequest {
    pub preferred_families: Vec<String>,
    pub locale: String,
    pub role: FontRole,
}

impl FontRequest {
    pub fn new(preferred_families: Vec<String>, locale: &str, role: FontRole) -> Self {
        Self {
            preferred_families,
            locale: locale.to_string(),
            role,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "FontRequest(",
            "preferredFamilies=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.preferred_families).clone();
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
            "locale=",
            (self.locale).to_string(),
            ", ",
            "role=",
            self.role.name(),
            ")"
        );
    }
}

fn font_request_role_order(v: &FontRole) -> i32 {
    match v {
        FontRole::Unknown => 5,
        FontRole::Symbol => 3,
        FontRole::LatinText => 2,
        FontRole::Emoji => 4,
        FontRole::CjkText => 0,
        FontRole::CjkPunctuation => 1,
    }
}
pub fn compare_font_request(a: &FontRequest, b: &FontRequest) -> i32 {
    let mut cmp_preferred_families = 0; for (av, bv) in a.preferred_families.iter().zip(b.preferred_families.iter()) { cmp_preferred_families = match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; if
cmp_preferred_families != 0 { break; } }
    if cmp_preferred_families == 0 { cmp_preferred_families = match a.preferred_families.len().cmp(&b.preferred_families.len()) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; }
    if cmp_preferred_families != 0 { return cmp_preferred_families; }
    let cmp_locale = SortedTable::sorted_table_compare_strings(a.locale.as_str(), b.locale.as_str());
    if cmp_locale != 0 { return cmp_locale; }
    let cmp_role = match font_request_role_order(&a.role).cmp(&font_request_role_order(&b.role)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_role != 0 { return cmp_role; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontCandidate {
    pub key: String,
    pub family: String,
    pub role: FontRole,
}

impl FontCandidate {
    pub fn new(key: &str, family: &str, role: FontRole) -> Self {
        Self {
            key: key.to_string(),
            family: family.to_string(),
            role,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "FontCandidate(",
            "key=",
            (self.key).to_string(),
            ", ",
            "family=",
            (self.family).to_string(),
            ", ",
            "role=",
            self.role.name(),
            ")"
        );
    }
}

fn font_candidate_role_order(v: &FontRole) -> i32 {
    match v {
        FontRole::Unknown => 5,
        FontRole::Symbol => 3,
        FontRole::LatinText => 2,
        FontRole::Emoji => 4,
        FontRole::CjkText => 0,
        FontRole::CjkPunctuation => 1,
    }
}
pub fn compare_font_candidate(a: &FontCandidate, b: &FontCandidate) -> i32 {
    let cmp_key = SortedTable::sorted_table_compare_strings(a.key.as_str(), b.key.as_str());
    if cmp_key != 0 { return cmp_key; }
    let cmp_family = SortedTable::sorted_table_compare_strings(a.family.as_str(), b.family.as_str());
    if cmp_family != 0 { return cmp_family; }
    let cmp_role = match font_candidate_role_order(&a.role).cmp(&font_candidate_role_order(&b.role)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_role != 0 { return cmp_role; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontDecision {
    pub range: TextRange,
    pub candidate: FontCandidate,
    pub role: FontRole,
    pub reason: String,
}

impl FontDecision {
    pub fn new(range: TextRange, candidate: FontCandidate, role: FontRole, reason: &str) -> Self {
        Self {
            range,
            candidate,
            role,
            reason: reason.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "FontDecision(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "candidate=",
            (self.candidate).clone().to_string(),
            ", ",
            "role=",
            self.role.name(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

fn font_decision_role_order(v: &FontRole) -> i32 {
    match v {
        FontRole::Unknown => 5,
        FontRole::Symbol => 3,
        FontRole::LatinText => 2,
        FontRole::Emoji => 4,
        FontRole::CjkText => 0,
        FontRole::CjkPunctuation => 1,
    }
}
pub fn compare_font_decision(a: &FontDecision, b: &FontDecision) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_candidate = compare_font_candidate(&a.candidate, &b.candidate);
    if cmp_candidate != 0 { return cmp_candidate; }
    let cmp_role = match font_decision_role_order(&a.role).cmp(&font_decision_role_order(&b.role)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_role != 0 { return cmp_role; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}

pub trait FallbackResolver: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn FallbackResolver>;
    fn resolve(&self, text: &str, range: TextRange, request: FontRequest) -> FontDecision;
}

impl Clone for Box<dyn FallbackResolver> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn FallbackResolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

impl FontRole {
    pub fn uses_latin_face(&self) -> bool {
        return *self == FontRole::LatinText;
    }
}

pub fn font_role_name_uses_latin_face(role_name: Option<String>) -> bool {
    if role_name.is_none() {
        return false;
    }
    let values = FontRole::ALL;
    let mut pipeline_result = false;
    for pipeline_index in 0..6 {
        let value = values[usize::try_from(pipeline_index).unwrap_or(0)];
        if role_name.as_ref().map_or(false, |v| v == &(value.name().to_string())) && value.uses_latin_face() {
            pipeline_result = true;
            break;
        }
    }
    return pipeline_result;
}
