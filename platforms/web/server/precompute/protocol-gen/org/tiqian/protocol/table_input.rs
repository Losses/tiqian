use crate::org::tiqian::protocol::table_metric_row::TableMetricRow;
use crate::org::tiqian::protocol::table_probe::TableProbe;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, PartialEq)]
pub struct TableInput {
    pub replay_strings: Vec<UString>,
    pub metrics: Vec<TableMetricRow>,
    pub probes: Vec<TableProbe>,
    pub faces: Vec<UString>,
    pub typographies: Vec<UString>,
    pub value_styles: Vec<UString>,
    pub font_preloads: Vec<UString>,
    pub revisions_text: UString,
}

impl TableInput {
    pub fn new(replay_strings: Vec<UString>, metrics: Vec<TableMetricRow>, probes: Vec<TableProbe>, faces: Vec<UString>, typographies: Vec<UString>, value_styles: Vec<UString>, font_preloads: Vec<UString>, revisions_text: &UStr) -> Self {
        Self {
            replay_strings,
            metrics,
            probes,
            faces,
            typographies,
            value_styles,
            font_preloads,
            revisions_text: revisions_text.to_ustring(),
        }
    }
}
