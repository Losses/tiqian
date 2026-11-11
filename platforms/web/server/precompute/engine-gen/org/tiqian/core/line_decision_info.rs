use crate::org::tiqian::core::line_repair_candidate_info::LineRepairCandidateInfo;
use crate::org::tiqian::core::line_repair_decision_info::LineRepairDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LineDecisionInfo {
    pub range: TextRange,
    pub kind: UString,
    pub repair: Option<UString>,
    pub repair_penalty: u32,
    pub repair_decision: Option<LineRepairDecisionInfo>,
    pub repair_candidates: Vec<LineRepairCandidateInfo>,
    pub notes: Vec<UString>,
}

impl LineDecisionInfo {
    pub fn new(range: TextRange, kind: &UStr, repair: Option<UString>, repair_penalty: Option<u32>, repair_decision: Option<LineRepairDecisionInfo>, repair_candidates: Option<Vec<LineRepairCandidateInfo>>, notes: Option<Vec<UString>>) -> Self {
        let repair = repair.or_else(|| None);
        let repair_penalty = repair_penalty.unwrap_or_else(|| 0);
        let repair_decision = repair_decision.or_else(|| None);
        let repair_candidates = repair_candidates.unwrap_or_else(|| vec![]);
        let notes = notes.unwrap_or_else(|| vec![]);
        Self {
            range,
            kind: kind.to_ustring(),
            repair: repair,
            repair_penalty: repair_penalty,
            repair_decision: repair_decision,
            repair_candidates: repair_candidates,
            notes: notes,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("LineDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("kind=")); __s += (self.kind).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("repair=")); __s += match &((self.repair).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("repairPenalty=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.repair_penalty)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("repairDecision=")); __s += (match &(self.repair_decision) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("repairCandidates=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.repair_candidates).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("notes=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.notes).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}
