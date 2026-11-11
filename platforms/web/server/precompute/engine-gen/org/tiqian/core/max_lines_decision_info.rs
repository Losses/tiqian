use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct MaxLinesDecisionInfo {
    pub laid_out_lines: u32,
    pub visible_lines: u32,
    pub reason: UString,
}

impl MaxLinesDecisionInfo {
    pub fn new(laid_out_lines: u32, visible_lines: u32, reason: Option<UString>) -> Self {
        let reason = reason.unwrap_or_else(|| UString::from("MaxLinesLineTruncation"));
        Self {
            laid_out_lines,
            visible_lines,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("MaxLinesDecisionInfo(")); __s += &(UString::from("laidOutLines=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.laid_out_lines)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("visibleLines=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.visible_lines)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

pub fn compare_max_lines_decision_info(a: &MaxLinesDecisionInfo, b: &MaxLinesDecisionInfo) -> i32 {
    let cmp_laid_out_lines = match i32::from_ne_bytes((*(&a.laid_out_lines)).to_ne_bytes()).cmp(&i32::from_ne_bytes((*(&b.laid_out_lines)).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_laid_out_lines != 0 { return cmp_laid_out_lines; }
    let cmp_visible_lines = match i32::from_ne_bytes((*(&a.visible_lines)).to_ne_bytes()).cmp(&i32::from_ne_bytes((*(&b.visible_lines)).to_ne_bytes())) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_visible_lines != 0 { return cmp_visible_lines; }
    let cmp_reason = SortedTable::sorted_table_compare_strings((&a.reason).as_ustr(), (&b.reason).as_ustr());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
