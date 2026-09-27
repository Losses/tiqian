use crate::org::tiqian::protocol::metric_entry::MetricEntry;
use crate::org::tiqian::protocol::value_row::ValueRow;


#[derive(Clone, PartialEq)]
pub struct TableData {
    pub replay_string_count: u32,
    pub strings: Vec<String>,
    pub metric_rows: Vec<MetricEntry>,
    pub value_pool: Vec<ValueRow>,
    pub probe_text_refs: Vec<u32>,
    pub probe_advance_refs: Vec<u32>,
    pub probe_style_refs: Vec<u32>,
    pub probe_feature_refs: Vec<u32>,
    pub advance_pool: Vec<f64>,
    pub style_font_size: Vec<f64>,
    pub style_font_weight: Vec<f64>,
    pub style_italic: Vec<u32>,
    pub style_script_refs: Vec<u32>,
    pub style_language_refs: Vec<u32>,
    pub features_pool: Vec<Vec<u32>>,
    pub face_texts: Vec<String>,
    pub typography_texts: Vec<String>,
    pub value_style_texts: Vec<String>,
    pub font_preload_texts: Vec<String>,
    pub revision_text: String,
}

impl TableData {
    pub fn new() -> Self {
        Self {
            replay_string_count: 0,
            strings: Vec::new(),
            metric_rows: Vec::new(),
            value_pool: Vec::new(),
            probe_text_refs: Vec::new(),
            probe_advance_refs: Vec::new(),
            probe_style_refs: Vec::new(),
            probe_feature_refs: Vec::new(),
            advance_pool: Vec::new(),
            style_font_size: Vec::new(),
            style_font_weight: Vec::new(),
            style_italic: Vec::new(),
            style_script_refs: Vec::new(),
            style_language_refs: Vec::new(),
            features_pool: Vec::new(),
            face_texts: Vec::new(),
            typography_texts: Vec::new(),
            value_style_texts: Vec::new(),
            font_preload_texts: Vec::new(),
            revision_text: "".to_string(),
        }
    }
}
