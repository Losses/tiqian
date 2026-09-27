use crate::org::tiqian::protocol::table_metric_row::TableMetricRow;
use crate::org::tiqian::protocol::table_probe::TableProbe;


#[derive(Clone, PartialEq)]
pub struct TableInput {
    pub replay_strings: Vec<String>,
    pub metrics: Vec<TableMetricRow>,
    pub probes: Vec<TableProbe>,
    pub faces: Vec<String>,
    pub typographies: Vec<String>,
    pub value_styles: Vec<String>,
    pub font_preloads: Vec<String>,
    pub revisions_text: String,
}

impl TableInput {
    pub fn new(replay_strings: Vec<String>, metrics: Vec<TableMetricRow>, probes: Vec<TableProbe>, faces: Vec<String>, typographies: Vec<String>, value_styles: Vec<String>, font_preloads: Vec<String>, revisions_text: &str) -> Self {
        Self {
            replay_strings,
            metrics,
            probes,
            faces,
            typographies,
            value_styles,
            font_preloads,
            revisions_text: revisions_text.to_string(),
        }
    }
}
