use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_data::UnicodeScriptEvidenceData;


#[derive(Clone, Copy)]
pub struct UnicodeScriptEvidenceClassifier;

impl UnicodeScriptEvidenceClassifier {
    pub const UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_REVISION: &str = "17.0.0";
    pub const UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SOURCE: &str = "https://www.unicode.org/Public/17.0.0/ucd/Scripts.txt";
    pub const UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SHA256: &str = "9f5e50d3abaee7d6ce09480f325c706f485ae3240912527e651954d2d6b035bf";

    pub fn unicode_script_evidence_classifier_classify(code_point: u32) -> Result<UnicodeScriptEvidence, TextRangeError> {
        if code_point > 2147483647 || (i32::from_ne_bytes((code_point).to_ne_bytes())) > (1114111) {
            return Err(TextRangeError::Message { text: format!("{}{}",
            "Not a Unicode scalar value: ",
            crate::runtime::int_text::IntText::int_text(code_point)
        ).to_string() });
        }
        if i32::from_ne_bytes((code_point).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes((code_point).to_ne_bytes())) <= 57343 {
            return Err(TextRangeError::Message { text: format!("{}{}",
            "Surrogate is not a Unicode scalar value: ",
            crate::runtime::int_text::IntText::int_text(code_point)
        ).to_string() });
        }
        return Ok(UnicodeScriptEvidenceData::unicode_script_evidence_data_classify(u32::from_ne_bytes((code_point).to_ne_bytes())));
    }
}
