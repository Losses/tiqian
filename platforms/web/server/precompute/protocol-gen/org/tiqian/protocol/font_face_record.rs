use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct FontFaceRecord {
    pub family: UString,
    pub local_names: Vec<UString>,
    pub unicode_range: UString,
    pub public_url: UString,
}
