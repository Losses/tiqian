use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct JustificationDecisionInfo {
    pub line_range: TextRange,
    pub deficit_before: f64,
    pub deficit_after: f64,
    pub allocations: Vec<JustificationAllocationInfo>,
}

impl JustificationDecisionInfo {
    pub fn new(line_range: TextRange, deficit_before: f64, deficit_after: f64, allocations: Vec<JustificationAllocationInfo>) -> Self {
        Self {
            line_range,
            deficit_before,
            deficit_after,
            allocations,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "JustificationDecisionInfo(",
            "lineRange=",
            (self.line_range).clone().to_string(),
            ", ",
            "deficitBefore=",
            self.deficit_before,
            ", ",
            "deficitAfter=",
            self.deficit_after,
            ", ",
            "allocations=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.allocations).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ")"
        );
    }
}
