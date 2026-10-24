#![cfg(test)]

use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct RecordedShapingEvidenceData;

impl RecordedShapingEvidenceData {
    pub fn recorded_shaping_evidence_data_evidence_json() -> UString {
        return UString::from(include_str!("RecordedShapingEvidenceData__asset_0.txt")).to_ustring();
    }
}
