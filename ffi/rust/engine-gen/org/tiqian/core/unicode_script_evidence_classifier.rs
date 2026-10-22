use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::core::unicode_script_evidence::UnicodeScriptEvidence;
use crate::org::tiqian::core::unicode_script_evidence_data::UnicodeScriptEvidenceData;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct UnicodeScriptEvidenceClassifier;

impl UnicodeScriptEvidenceClassifier {
    pub const UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_REVISION: &UStr = unsafe { &*(&[0x0031u16, 0x0037u16, 0x002Eu16, 0x0030u16, 0x002Eu16, 0x0030u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    pub const UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SOURCE: &UStr = unsafe { &*(&[0x0068u16, 0x0074u16, 0x0074u16, 0x0070u16, 0x0073u16, 0x003Au16, 0x002Fu16, 0x002Fu16, 0x0077u16, 0x0077u16, 0x0077u16, 0x002Eu16, 0x0075u16, 0x006Eu16, 0x0069u16, 0x0063u16, 0x006Fu16, 0x0064u16, 0x0065u16, 0x002Eu16, 0x006Fu16, 0x0072u16, 0x0067u16, 0x002Fu16, 0x0050u16, 0x0075u16, 0x0062u16, 0x006Cu16, 0x0069u16, 0x0063u16, 0x002Fu16, 0x0031u16, 0x0037u16, 0x002Eu16, 0x0030u16, 0x002Eu16, 0x0030u16, 0x002Fu16, 0x0075u16, 0x0063u16, 0x0064u16, 0x002Fu16, 0x0053u16, 0x0063u16, 0x0072u16, 0x0069u16, 0x0070u16, 0x0074u16, 0x0073u16, 0x002Eu16, 0x0074u16, 0x0078u16, 0x0074u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
    pub const UNICODE_SCRIPT_EVIDENCE_CLASSIFIER_DATA_SHA256: &UStr = unsafe { &*(&[0x0039u16, 0x0066u16, 0x0035u16, 0x0065u16, 0x0035u16, 0x0030u16, 0x0064u16, 0x0033u16, 0x0061u16, 0x0062u16, 0x0061u16, 0x0065u16, 0x0065u16, 0x0037u16, 0x0064u16, 0x0036u16, 0x0063u16, 0x0065u16, 0x0030u16, 0x0039u16, 0x0034u16, 0x0038u16, 0x0030u16, 0x0066u16, 0x0033u16, 0x0032u16, 0x0035u16, 0x0063u16, 0x0037u16, 0x0030u16, 0x0036u16, 0x0066u16, 0x0034u16, 0x0038u16, 0x0035u16, 0x0061u16, 0x0065u16, 0x0033u16, 0x0032u16, 0x0034u16, 0x0030u16, 0x0039u16, 0x0031u16, 0x0032u16, 0x0035u16, 0x0032u16, 0x0037u16, 0x0065u16, 0x0036u16, 0x0035u16, 0x0031u16, 0x0039u16, 0x0035u16, 0x0034u16, 0x0064u16, 0x0032u16, 0x0064u16, 0x0036u16, 0x0062u16, 0x0030u16, 0x0033u16, 0x0035u16, 0x0062u16, 0x0066u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    pub fn unicode_script_evidence_classifier_classify(code_point: u32) -> Result<UnicodeScriptEvidence, TextRangeError> {
        if code_point > 2147483647 || (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) > (1114111) {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        if i32::from_ne_bytes(((code_point) as i32).to_ne_bytes()) >= 55296 && (i32::from_ne_bytes(((code_point) as i32).to_ne_bytes())) <= 57343 {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Surrogate is not a Unicode scalar value: ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(code_point)).as_str())); __s }).as_str()) });
        }
        return Ok(UnicodeScriptEvidenceData::unicode_script_evidence_data_classify(u32::from_ne_bytes(((code_point) as u32).to_ne_bytes())));
    }
}
