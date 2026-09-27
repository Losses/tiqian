use crate::runtime::sorted_table::SortedTable;


#[derive(Debug, Clone, PartialEq)]
pub struct MaxLinesDecisionInfo {
    pub laid_out_lines: u32,
    pub visible_lines: u32,
    pub reason: String,
}

impl MaxLinesDecisionInfo {
    pub fn new(laid_out_lines: u32, visible_lines: u32, reason: Option<String>) -> Self {
        let reason = reason.unwrap_or_else(|| "MaxLinesLineTruncation".to_string());
        Self {
            laid_out_lines,
            visible_lines,
            reason: reason,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}",
            "MaxLinesDecisionInfo(",
            "laidOutLines=",
            crate::runtime::int_text::IntText::int_text(self.laid_out_lines),
            ", ",
            "visibleLines=",
            crate::runtime::int_text::IntText::int_text(self.visible_lines),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ")"
        );
    }
}

pub fn compare_max_lines_decision_info(a: &MaxLinesDecisionInfo, b: &MaxLinesDecisionInfo) -> i32 {
    let cmp_laid_out_lines = if a.laid_out_lines < b.laid_out_lines { -1 } else if a.laid_out_lines > b.laid_out_lines { 1 } else { 0 };
    if cmp_laid_out_lines != 0 { return cmp_laid_out_lines; }
    let cmp_visible_lines = if a.visible_lines < b.visible_lines { -1 } else if a.visible_lines > b.visible_lines { 1 } else { 0 };
    if cmp_visible_lines != 0 { return cmp_visible_lines; }
    let cmp_reason = SortedTable::sorted_table_compare_strings(a.reason.as_str(), b.reason.as_str());
    if cmp_reason != 0 { return cmp_reason; }
    0
}
