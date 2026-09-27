use crate::org::tiqian::protocol::table_input::TableInput;
use crate::org::tiqian::protocol::table_metric_row::TableMetricRow;
use crate::org::tiqian::protocol::table_probe::TableProbe;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct SnapshotTableTestSupport;

impl SnapshotTableTestSupport {
    pub fn snapshot_table_test_support_hex_bytes(bytes: &[u8]) -> String {
        let digits = "0123456789abcdef".to_string();
        let mut out_b = String::new();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let byte = u32::from(bytes[usize::try_from(index_idx).unwrap_or(0)]);
            {
                let c = u_string::unit_at(&digits, byte >> 4 & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { String::from_utf16(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(c) as u16]) });
            }
            {
                let c = u_string::unit_at(&digits, byte & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { String::from_utf16(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(c) as u16]) });
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return out_b;
    }

    pub fn snapshot_table_test_support_assert_hex_bytes(recorder: TestTraceRecorder, label: &str, expected: &str, bytes: &[u8]) -> Result<(), TracedAssertionsFailFault> {
        let _ = recorder.record(label).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected, SnapshotTableTestSupport::snapshot_table_test_support_hex_bytes(&bytes).as_str(), Some((label).to_string()))?;
        Ok(())
    }

    pub fn snapshot_table_test_support_golden_input() -> TableInput {
        let mut metrics: Vec<TableMetricRow> = Vec::new();
        metrics.push(TableMetricRow::new("Arial", 400.0f64, false, "sans", "fs-1", SnapshotTableTestSupport::snapshot_table_test_support_five_values(1.5f64, None, 2.0f64).to_vec()));
        metrics.push(TableMetricRow::new("宋体", 400.0f64, true, "sans", "fs-1", SnapshotTableTestSupport::snapshot_table_test_support_five_values(1.5f64, None, 2.0f64).to_vec()));
        let mut probes: Vec<TableProbe> = Vec::new();
        probes.push(TableProbe::new("汉", 12.5f64, 16.0f64, 400.0f64, false, "Hans", "zh", SnapshotTableTestSupport::snapshot_table_test_support_one_feature().to_vec()));
        probes.push(TableProbe::new("字", 12.5f64, 16.0f64, 400.0f64, false, "Hans", "zh", SnapshotTableTestSupport::snapshot_table_test_support_one_feature().to_vec()));
        let mut faces: Vec<String> = Vec::new();
        faces.push("{\"family\":\"F\"}".to_string());
        let typographies: Vec<String> = Vec::new();
        let mut value_styles: Vec<String> = Vec::new();
        value_styles.push("vs-0".to_string());
        let preloads: Vec<String> = Vec::new();
        return TableInput::new(SnapshotTableTestSupport::snapshot_table_test_support_single(&"replay-a").to_vec(), metrics.to_vec(), probes.to_vec(), faces.to_vec(), typographies.to_vec(), value_styles.to_vec(), preloads.to_vec(), "{\"backendRevision\":\"r1\"}");
    }

    pub fn snapshot_table_test_support_single(text: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        out.push(text.to_string());
        return out;
    }

    pub fn snapshot_table_test_support_one_feature() -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        out.push("liga".to_string());
        return out;
    }

    pub fn snapshot_table_test_support_five_values(a: f64, b: Option<f64>, c: f64) -> Vec<Option<f64>> {
        let mut out: Vec<Option<f64>> = Vec::new();
        out.push(Some(a));
        out.push(b);
        out.push(Some(c));
        out.push(None);
        out.push(None);
        return out;
    }

    pub fn snapshot_table_test_support_two_pool_input() -> TableInput {
        let mut metrics: Vec<TableMetricRow> = Vec::new();
        metrics.push(TableMetricRow::new("Arial", 400.0f64, false, "sans", "fs-1", SnapshotTableTestSupport::snapshot_table_test_support_five_values(1.5f64, None, 2.0f64).to_vec()));
        metrics.push(TableMetricRow::new("宋体", 500.0f64, true, "sans", "fs-1", SnapshotTableTestSupport::snapshot_table_test_support_five_values(3.5f64, Some(4.0f64), 5.0f64).to_vec()));
        let probes: Vec<TableProbe> = Vec::new();
        let empty: Vec<String> = Vec::new();
        let mut texts: Vec<String> = Vec::new();
        texts.push("{}".to_string());
        return TableInput::new(texts.to_vec(), metrics.to_vec(), probes.to_vec(), empty.to_vec(), empty.to_vec(), empty.to_vec(), empty.to_vec(), "{}");
    }
}
