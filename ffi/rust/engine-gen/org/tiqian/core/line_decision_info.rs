use crate::org::tiqian::core::line_repair_candidate_info::LineRepairCandidateInfo;
use crate::org::tiqian::core::line_repair_decision_info::LineRepairDecisionInfo;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct LineDecisionInfo {
    pub range: TextRange,
    pub kind: String,
    pub repair: Option<String>,
    pub repair_penalty: u32,
    pub repair_decision: Option<LineRepairDecisionInfo>,
    pub repair_candidates: Vec<LineRepairCandidateInfo>,
    pub notes: Vec<String>,
}

impl LineDecisionInfo {
    pub fn new(range: TextRange, kind: &str, repair: Option<String>, repair_penalty: Option<u32>, repair_decision: Option<LineRepairDecisionInfo>, repair_candidates: Option<Vec<LineRepairCandidateInfo>>, notes: Option<Vec<String>>) -> Self {
        let repair = repair.or_else(|| None);
        let repair_penalty = repair_penalty.unwrap_or_else(|| 0);
        let repair_decision = repair_decision.or_else(|| None);
        let repair_candidates = repair_candidates.unwrap_or_else(|| vec![]);
        let notes = notes.unwrap_or_else(|| vec![]);
        Self {
            range,
            kind: kind.to_string(),
            repair: repair,
            repair_penalty: repair_penalty,
            repair_decision: repair_decision,
            repair_candidates: repair_candidates,
            notes: notes,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LineDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "kind=",
            (self.kind).to_string(),
            ", ",
            "repair=",
            match (self.repair).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "repairPenalty=",
            crate::runtime::int_text::IntText::int_text(self.repair_penalty),
            ", ",
            "repairDecision=",
            (match &(self.repair_decision) { None => "null".to_string(), Some(__option) => __option.to_string() }),
            ", ",
            "repairCandidates=",
            {
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
    },
            ", ",
            "notes=",
            {
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
    },
            ")"
        );
    }
}
