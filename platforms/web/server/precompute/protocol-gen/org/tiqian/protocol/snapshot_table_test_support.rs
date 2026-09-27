use crate::org::tiqian::protocol::table_input::TableInput;
use crate::org::tiqian::protocol::table_metric_row::TableMetricRow;
use crate::org::tiqian::protocol::table_probe::TableProbe;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct SnapshotTableTestSupport;

impl SnapshotTableTestSupport {
    pub fn snapshot_table_test_support_hex_bytes(bytes: &[u8]) -> UString {
        let digits = UString::from("0123456789abcdef").to_ustring();
        let mut out_b = UString::new();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes(((index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let byte = u32::from(bytes[usize::try_from(index_idx).unwrap_or(0)]);
            {
                let c = u_string::unit_at(&digits, byte >> 4 & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
            }
            {
                let c = u_string::unit_at(&digits, byte & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return out_b;
    }

    pub fn snapshot_table_test_support_assert_hex_bytes(recorder: TestTraceRecorder, label: &UStr, expected: &UStr, bytes: &[u8]) -> Result<(), TracedAssertionsFailFault> {
        let _ = recorder.record(label).map_err(|e| TracedAssertionsFailFault::UStringFaultFault(e))?;
        let _ = TracedAssertions::traced_assertions_assert_equals_string(expected, SnapshotTableTestSupport::snapshot_table_test_support_hex_bytes(&bytes).as_ustr(), Some((label).to_ustring()))?;
        Ok(())
    }

    pub fn snapshot_table_test_support_golden_input() -> TableInput {
        let mut metrics: Vec<TableMetricRow> = Vec::new();
        metrics.push(TableMetricRow::new(&(UStr::new(&[65,114,105,97,108])), 400.0f64, false, &(UStr::new(&[115,97,110,115])), &(UStr::new(&[102,115,45,49])), SnapshotTableTestSupport::snapshot_table_test_support_five_values(1.5f64, None, 2.0f64).to_vec()));
        metrics.push(TableMetricRow::new(&(UStr::new(&[23435,20307])), 400.0f64, true, &(UStr::new(&[115,97,110,115])), &(UStr::new(&[102,115,45,49])), SnapshotTableTestSupport::snapshot_table_test_support_five_values(1.5f64, None, 2.0f64).to_vec()));
        let mut probes: Vec<TableProbe> = Vec::new();
        probes.push(TableProbe::new(&(UStr::new(&[27721])), 12.5f64, 16.0f64, 400.0f64, false, &(UStr::new(&[72,97,110,115])), &(UStr::new(&[122,104])), SnapshotTableTestSupport::snapshot_table_test_support_one_feature().to_vec()));
        probes.push(TableProbe::new(&(UStr::new(&[23383])), 12.5f64, 16.0f64, 400.0f64, false, &(UStr::new(&[72,97,110,115])), &(UStr::new(&[122,104])), SnapshotTableTestSupport::snapshot_table_test_support_one_feature().to_vec()));
        let mut faces: Vec<UString> = Vec::new();
        faces.push(UString::from("{\"family\":\"F\"}").to_ustring());
        let typographies: Vec<UString> = Vec::new();
        let mut value_styles: Vec<UString> = Vec::new();
        value_styles.push(UString::from("vs-0").to_ustring());
        let preloads: Vec<UString> = Vec::new();
        return TableInput::new(SnapshotTableTestSupport::snapshot_table_test_support_single(UStr::new(&[114,101,112,108,97,121,45,97])).to_vec(), metrics.to_vec(), probes.to_vec(), faces.to_vec(), typographies.to_vec(), value_styles.to_vec(), preloads.to_vec(), &(UStr::new(&[123,34,98,97,99,107,101,110,100,82,101,118,105,115,105,111,110,34,58,34,114,49,34,125])));
    }

    pub fn snapshot_table_test_support_single(text: &UStr) -> Vec<UString> {
        let mut out: Vec<UString> = Vec::new();
        out.push(text.to_ustring());
        return out;
    }

    pub fn snapshot_table_test_support_one_feature() -> Vec<UString> {
        let mut out: Vec<UString> = Vec::new();
        out.push(UString::from("liga").to_ustring());
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
        metrics.push(TableMetricRow::new(&(UStr::new(&[65,114,105,97,108])), 400.0f64, false, &(UStr::new(&[115,97,110,115])), &(UStr::new(&[102,115,45,49])), SnapshotTableTestSupport::snapshot_table_test_support_five_values(1.5f64, None, 2.0f64).to_vec()));
        metrics.push(TableMetricRow::new(&(UStr::new(&[23435,20307])), 500.0f64, true, &(UStr::new(&[115,97,110,115])), &(UStr::new(&[102,115,45,49])), SnapshotTableTestSupport::snapshot_table_test_support_five_values(3.5f64, Some(4.0f64), 5.0f64).to_vec()));
        let probes: Vec<TableProbe> = Vec::new();
        let empty: Vec<UString> = Vec::new();
        let mut texts: Vec<UString> = Vec::new();
        texts.push(UString::from("{}").to_ustring());
        return TableInput::new(texts.to_vec(), metrics.to_vec(), probes.to_vec(), empty.to_vec(), empty.to_vec(), empty.to_vec(), empty.to_vec(), &(UStr::new(&[123,125])));
    }
}
