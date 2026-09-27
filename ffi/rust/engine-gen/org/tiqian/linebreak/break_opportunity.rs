use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::text_range::compare_text_range;
use crate::org::tiqian::linebreak::break_kind::BreakKind;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct BreakOpportunity {
    pub index: u32,
    pub kind: BreakKind,
    pub penalty: u32,
    pub reason: UString,
}

impl BreakOpportunity {
    pub fn new(index: u32, kind: BreakKind, reason: &UStr, penalty: Option<u32>) -> Self {
        let penalty = penalty.unwrap_or_else(|| 0);
        Self {
            index,
            kind,
            reason: reason.to_ustring(),
            penalty: penalty,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("BreakOpportunity(")); __s += &(UString::from("index=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.index)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("kind=")); __s += UString::from(self.kind.name()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("penalty=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.penalty)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

fn break_opportunity_kind_order(v: &BreakKind) -> i32 {
    match v {
        BreakKind::Required => 2,
        BreakKind::Problematic => 3,
        BreakKind::Forbidden => 1,
        BreakKind::Allowed => 0,
    }
}
pub fn compare_break_opportunity(a: &BreakOpportunity, b: &BreakOpportunity) -> i32 {
    let cmp_index = if a.index < b.index { -1 } else if a.index > b.index { 1 } else { 0 };
    if cmp_index != 0 { return cmp_index; }
    let cmp_kind = match break_opportunity_kind_order(&a.kind).cmp(&break_opportunity_kind_order(&b.kind)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_kind != 0 { return cmp_kind; }
    let cmp_penalty = if a.penalty < b.penalty { -1 } else if a.penalty > b.penalty { 1 } else { 0 };
    if cmp_penalty != 0 { return cmp_penalty; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForbiddenBreak {
    pub range: TextRange,
    pub reason: UString,
}

impl ForbiddenBreak {
    pub fn new(range: TextRange, reason: &UStr) -> Self {
        Self {
            range,
            reason: reason.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ForbiddenBreak(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_forbidden_break(a: &ForbiddenBreak, b: &ForbiddenBreak) -> i32 {
    let cmp_range = compare_text_range(&a.range, &b.range);
    if cmp_range != 0 { return cmp_range; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_ustr(), b.reason.as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
