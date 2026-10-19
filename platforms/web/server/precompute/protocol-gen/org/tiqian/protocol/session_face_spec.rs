use crate::org::tiqian::protocol::font_face_spec::FontFaceSpec;


#[derive(Debug, Clone)]
pub struct SessionFaceSpec {
    pub spec: FontFaceSpec,
    pub source_order: Option<f64>,
}
