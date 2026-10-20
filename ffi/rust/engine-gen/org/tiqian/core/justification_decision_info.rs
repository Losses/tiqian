use crate::org::tiqian::core::justification_allocation_info::JustificationAllocationInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UString;
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

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("JustificationDecisionInfo(")); __s += &(UString::from("lineRange=")); __s += UString::from(format!("{}", (self.line_range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("deficitBefore=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.deficit_before)); __s += &(UString::from(", ")); __s += &(UString::from("deficitAfter=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.deficit_after)); __s += &(UString::from(", ")); __s += &(UString::from("allocations=")); __s += UString::from(format!("{}", {
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
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
