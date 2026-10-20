use crate::org::tiqian::protocol::font_weight_spec::FontWeightSpec;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone)]
pub struct FontFaceSpec {
    pub family: UString,
    pub public_url: UString,
    pub source: Vec<u8>,
    pub face_index: Option<f64>,
    pub weight: FontWeightSpec,
    pub style: UString,
    pub unicode_range: Option<UString>,
    pub source_order: u32,
}
