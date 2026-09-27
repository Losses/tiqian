#[derive(Debug, Clone, PartialEq)]
pub enum EncodeResult {
    COk { bytes: Vec<u8> },
    CErr { issue: String },
}

impl EncodeResult {
    pub fn to_string(&self) -> String {
        match self {
            EncodeResult::COk { bytes } => format!("COk(bytes={})", format!("{:?}", bytes)),
            EncodeResult::CErr { issue } => format!("CErr(issue={})", (issue).clone()),
        }
    }
}
