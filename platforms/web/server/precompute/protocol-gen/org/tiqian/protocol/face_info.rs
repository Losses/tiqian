use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct FaceInfo {
    pub family: UString,
    pub style: UString,
    pub weight: Vec<f64>,
    pub unicode_range: UString,
    pub public_url: UString,
    pub source_sha256: UString,
    pub sfnt_sha256: UString,
    pub face_index: f64,
    pub source_order: u32,
    pub axis_tags: Vec<UString>,
    pub local_names: Vec<UString>,
}
