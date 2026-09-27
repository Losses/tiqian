use crate::org::tiqian::protocol::metric_entry::MetricEntry;
use crate::org::tiqian::protocol::style_row::StyleRow;
use crate::org::tiqian::protocol::table_data::TableData;
use crate::org::tiqian::protocol::table_input::TableInput;
use crate::org::tiqian::protocol::table_probe::TableProbe;
use crate::org::tiqian::protocol::value_row::ValueRow;
use crate::runtime::bytes_buffer::BytesBuffer;
use crate::runtime::fp_helper::FPHelper;


#[derive(Clone, Copy)]
pub struct SnapshotTableBinary;

impl SnapshotTableBinary {
    pub fn snapshot_table_binary_encode(table: TableInput) -> Vec<u8> {
        return SnapshotTableBinary::snapshot_table_binary_encode_data(SnapshotTableBinary::snapshot_table_binary_lower((table).clone()));
    }

    pub fn snapshot_table_binary_encode_data(data: TableData) -> Vec<u8> {
        let mut writer = TableWriter::new();
        writer.raw(&"TIQTBL03".as_bytes().to_vec());
        writer.u32(data.replay_string_count);
        writer.u32(u32::try_from((data.strings.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.probe_text_refs.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.advance_pool.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.style_font_size.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.face_texts.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.typography_texts.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.value_style_texts.len()) & 0xFFFF_FFFF).unwrap_or(0));
        writer.u32(u32::try_from((data.font_preload_texts.len()) & 0xFFFF_FFFF).unwrap_or(0));
        SnapshotTableBinary::snapshot_table_binary_write_text_region(&mut writer, &data.strings);
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(data.metric_rows[usize::try_from(index_idx).unwrap_or(0)].families_ref);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.f64(data.metric_rows[usize::try_from(index_idx).unwrap_or(0)].weight);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u8(data.metric_rows[usize::try_from(index_idx).unwrap_or(0)].italic);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(data.metric_rows[usize::try_from(index_idx).unwrap_or(0)].role_ref);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(data.metric_rows[usize::try_from(index_idx).unwrap_or(0)].face_selection_ref);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(data.metric_rows[usize::try_from(index_idx).unwrap_or(0)].value_pool_ref);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            SnapshotTableBinary::snapshot_table_binary_write_value_row(&mut writer, (data.value_pool[usize::try_from(index_idx).unwrap_or(0)]).clone());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.probe_text_refs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(data.probe_text_refs[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.probe_advance_refs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u16(data.probe_advance_refs[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.probe_style_refs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u16(data.probe_style_refs[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.probe_feature_refs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u16(data.probe_feature_refs[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.advance_pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.f64(data.advance_pool[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.style_font_size.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.f64(data.style_font_size[usize::try_from(index_idx).unwrap_or(0)]);
            writer.f64(data.style_font_weight[usize::try_from(index_idx).unwrap_or(0)]);
            writer.u8(data.style_italic[usize::try_from(index_idx).unwrap_or(0)]);
            writer.u32(data.style_script_refs[usize::try_from(index_idx).unwrap_or(0)]);
            writer.u32(data.style_language_refs[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(u32::try_from((SnapshotTableBinary::snapshot_table_binary_feature_row_bytes(&(data.features_pool[usize::try_from(index_idx).unwrap_or(0)]).clone()).len()) & 0xFFFF_FFFF).unwrap_or(0));
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((data.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.raw(&SnapshotTableBinary::snapshot_table_binary_feature_row_bytes(&(data.features_pool[usize::try_from(index_idx).unwrap_or(0)]).clone()));
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        SnapshotTableBinary::snapshot_table_binary_write_text_region(&mut writer, &data.face_texts);
        SnapshotTableBinary::snapshot_table_binary_write_text_region(&mut writer, &data.typography_texts);
        SnapshotTableBinary::snapshot_table_binary_write_text_region(&mut writer, &data.value_style_texts);
        SnapshotTableBinary::snapshot_table_binary_write_text_region(&mut writer, &data.font_preload_texts);
        writer.raw(&(data.revision_text).to_string().as_bytes().to_vec());
        return writer.finish();
    }

    pub fn snapshot_table_binary_decode_into(bytes: &[u8], data: &mut TableData) -> String {
        let mut reader = TableReader::new(bytes);
        if !reader.match_magic() {
            return ((reader.issue).to_string()).clone();
        }
        data.replay_string_count = reader.u32();
        let string_count = reader.u32();
        let metric_count = reader.u32();
        let value_pool_count = reader.u32();
        let probe_count = reader.u32();
        let advance_pool_count = reader.u32();
        let style_pool_count = reader.u32();
        let features_pool_count = reader.u32();
        let face_count = reader.u32();
        let typography_count = reader.u32();
        let value_style_count = reader.u32();
        let font_preload_count = reader.u32();
        data.strings = reader.text_region(string_count);
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((metric_count).to_ne_bytes())) {
            let mut row = MetricEntry::new(reader.u32(), 0 as f64 as f64, 0u32, 0u32, 0u32, 0u32);
            row.stored = true;
            data.metric_rows.push(row.clone());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((metric_count).to_ne_bytes())) {
            (data.metric_rows[usize::try_from(index_idx).unwrap_or(0)]).clone().weight = reader.f64();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((metric_count).to_ne_bytes())) {
            (data.metric_rows[usize::try_from(index_idx).unwrap_or(0)]).clone().italic = reader.u8();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((metric_count).to_ne_bytes())) {
            (data.metric_rows[usize::try_from(index_idx).unwrap_or(0)]).clone().role_ref = reader.u32();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((metric_count).to_ne_bytes())) {
            (data.metric_rows[usize::try_from(index_idx).unwrap_or(0)]).clone().face_selection_ref = reader.u32();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((metric_count).to_ne_bytes())) {
            (data.metric_rows[usize::try_from(index_idx).unwrap_or(0)]).clone().value_pool_ref = reader.u32();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((value_pool_count).to_ne_bytes())) {
            data.value_pool.push(SnapshotTableBinary::snapshot_table_binary_read_value_row(&mut reader));
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((probe_count).to_ne_bytes())) {
            data.probe_text_refs.push(reader.u32());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((probe_count).to_ne_bytes())) {
            data.probe_advance_refs.push(reader.u16());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((probe_count).to_ne_bytes())) {
            data.probe_style_refs.push(reader.u16());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((probe_count).to_ne_bytes())) {
            data.probe_feature_refs.push(reader.u16());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((advance_pool_count).to_ne_bytes())) {
            data.advance_pool.push(reader.f64());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((style_pool_count).to_ne_bytes())) {
            data.style_font_size.push(reader.f64());
            data.style_font_weight.push(reader.f64());
            data.style_italic.push(reader.u8());
            data.style_script_refs.push(reader.u32());
            data.style_language_refs.push(reader.u32());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((features_pool_count).to_ne_bytes())) {
            reader.u32();
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        for _ in 0..features_pool_count {
            data.features_pool.push(reader.feature_row());
        }
        data.face_texts = reader.text_region(face_count);
        data.typography_texts = reader.text_region(typography_count);
        data.value_style_texts = reader.text_region(value_style_count);
        data.font_preload_texts = reader.text_region(font_preload_count);
        if reader.failed {
            return ((reader.issue).to_string()).clone();
        }
        data.revision_text = reader.rest_text();
        if reader.failed {
            return ((reader.issue).to_string()).clone();
        }
        return String::new();
    }

    pub fn snapshot_table_binary_sort_metric_rows(rows: &mut Vec<MetricEntry>) -> &Vec<MetricEntry> {
        let mut write_idx = 1u32;
        while (i32::from_ne_bytes((write_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((rows.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let record = (rows[usize::try_from(write_idx).unwrap_or(0)]).clone();
            let mut read_idx = write_idx;
            while (read_idx) > (0) && SnapshotTableBinary::snapshot_table_binary_metric_before((record).clone(), (rows[usize::try_from(u32::wrapping_sub(read_idx, 1)).unwrap_or(0)]).clone()) {
                rows[usize::try_from(read_idx).unwrap_or(0)] = (rows[usize::try_from(u32::wrapping_sub(read_idx, 1)).unwrap_or(0)]).clone();
                read_idx = u32::wrapping_sub(read_idx, 1);
            }
            rows[usize::try_from(read_idx).unwrap_or(0)] = record;
            write_idx = u32::wrapping_add(write_idx, 1);
        }
        return rows;
    }

    pub(crate) fn snapshot_table_binary_metric_before(a: MetricEntry, b: MetricEntry) -> bool {
        if a.families_ref != b.families_ref {
            return (i32::from_ne_bytes((a.families_ref).to_ne_bytes())) < (i32::from_ne_bytes((b.families_ref).to_ne_bytes()));
        }
        if a.weight < (b.weight) {
            return true;
        }
        if b.weight < (a.weight) {
            return false;
        }
        if a.italic != b.italic {
            return (i32::from_ne_bytes((a.italic).to_ne_bytes())) < (i32::from_ne_bytes((b.italic).to_ne_bytes()));
        }
        if a.role_ref != b.role_ref {
            return (i32::from_ne_bytes((a.role_ref).to_ne_bytes())) < (i32::from_ne_bytes((b.role_ref).to_ne_bytes()));
        }
        return (i32::from_ne_bytes((a.face_selection_ref).to_ne_bytes())) < (i32::from_ne_bytes((b.face_selection_ref).to_ne_bytes()));
    }

    pub(crate) fn snapshot_table_binary_lower(table: TableInput) -> TableData {
        let mut data = TableData::new();
        data.replay_string_count = u32::try_from((table.replay_strings.len()) & 0xFFFF_FFFF).unwrap_or(0);
        let mut interner = StringInterner::new(table.replay_strings.to_vec());
        let mut index_idx = 0u32;
        let mut metric_entries: Vec<MetricEntry> = Vec::new();
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((table.metrics.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let row = (table.metrics[usize::try_from(index_idx).unwrap_or(0)]).clone();
            let entry = MetricEntry::new(interner.intern((row.serialized_families).to_string().as_str()), row.font_weight, if row.italic { 1 } else { 0 }, interner.intern((row.role).to_string().as_str()), interner.intern((row.face_selection_text).to_string().as_str()), 0u32);
            metric_entries.push(entry.clone());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        SnapshotTableBinary::snapshot_table_binary_sort_metric_rows(&mut metric_entries);
        let mut value_pool: Vec<ValueRow> = Vec::new();
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((metric_entries.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let values = ((table.metrics[usize::try_from(index_idx).unwrap_or(0)]).clone().values_em).clone();
            (metric_entries[usize::try_from(index_idx).unwrap_or(0)]).clone().value_pool_ref = SnapshotTableBinary::snapshot_table_binary_pool_ref_of(&mut value_pool, &values);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        data.metric_rows = metric_entries;
        data.value_pool = value_pool;
        let mut advance_pool: Vec<f64> = Vec::new();
        let mut style_pool: Vec<StyleRow> = Vec::new();
        let mut features_pool: Vec<Vec<u32>> = Vec::new();
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((table.probes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let probe = (table.probes[usize::try_from(index_idx).unwrap_or(0)]).clone();
            let text_ref = interner.intern((probe.text).to_string().as_str());
            let script_ref = interner.intern((probe.script).to_string().as_str());
            let language_ref = interner.intern((probe.language).to_string().as_str());
            let mut feature_refs: Vec<u32> = Vec::new();
            let mut feature_idx = 0u32;
            while (i32::from_ne_bytes((feature_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((probe.features.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                feature_refs.push(interner.intern((probe.features[usize::try_from(feature_idx).unwrap_or(0)]).clone().as_str()));
                feature_idx = u32::wrapping_add(feature_idx, 1);
            }
            data.probe_text_refs.push(text_ref);
            data.probe_advance_refs.push(SnapshotTableBinary::snapshot_table_binary_pooled_float(&mut advance_pool, probe.advance_px));
            data.probe_style_refs.push(SnapshotTableBinary::snapshot_table_binary_pooled_style(&mut style_pool, (probe).clone(), script_ref, language_ref));
            data.probe_feature_refs.push(SnapshotTableBinary::snapshot_table_binary_pooled_features(&mut features_pool, &feature_refs));
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        data.advance_pool = advance_pool;
        data.features_pool = features_pool;
        data.style_font_size = Vec::new();
        data.style_font_weight = Vec::new();
        data.style_italic = Vec::new();
        data.style_script_refs = Vec::new();
        data.style_language_refs = Vec::new();
        let mut style_idx = 0u32;
        while (i32::from_ne_bytes((style_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((style_pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            data.style_font_size.push(style_pool[usize::try_from(style_idx).unwrap_or(0)].font_size_px);
            data.style_font_weight.push(style_pool[usize::try_from(style_idx).unwrap_or(0)].font_weight);
            data.style_italic.push(style_pool[usize::try_from(style_idx).unwrap_or(0)].italic);
            data.style_script_refs.push(style_pool[usize::try_from(style_idx).unwrap_or(0)].script_ref);
            data.style_language_refs.push(style_pool[usize::try_from(style_idx).unwrap_or(0)].language_ref);
            style_idx = u32::wrapping_add(style_idx, 1);
        }
        data.strings = interner.strings;
        data.face_texts = SnapshotTableBinary::snapshot_table_binary_copy_texts(&table.faces);
        data.typography_texts = SnapshotTableBinary::snapshot_table_binary_copy_texts(&table.typographies);
        data.value_style_texts = SnapshotTableBinary::snapshot_table_binary_copy_texts(&table.value_styles);
        data.font_preload_texts = SnapshotTableBinary::snapshot_table_binary_copy_texts(&table.font_preloads);
        data.revision_text = (table.revisions_text).to_string();
        return data;
    }

    pub(crate) fn snapshot_table_binary_copy_texts(texts: &Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((texts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            out.push((texts[usize::try_from(index_idx).unwrap_or(0)]).clone());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return out;
    }

    pub(crate) fn snapshot_table_binary_pool_ref_of(pool: &mut Vec<ValueRow>, values: &Vec<Option<f64>>) -> u32 {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if SnapshotTableBinary::snapshot_table_binary_same_values(&((pool[usize::try_from(index_idx).unwrap_or(0)]).clone().values).clone(), &values) {
                return index_idx;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        pool.push(ValueRow::new(values.to_vec()));
        return u32::wrapping_sub(u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
    }

    pub(crate) fn snapshot_table_binary_same_values(a: &Vec<Option<f64>>, b: &Vec<Option<f64>>) -> bool {
        if u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let left = a[usize::try_from(index_idx).unwrap_or(0)];
            let right = b[usize::try_from(index_idx).unwrap_or(0)];
            if left.is_none() && right.is_none() {
                index_idx = u32::wrapping_add(index_idx, 1);
                continue;
            }
            if match &(left) { None => true, Some(__option) => right.is_none() } {
                return false;
            }
            if left != right {
                return false;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return true;
    }

    pub(crate) fn snapshot_table_binary_f64_bits_equal(a: f64, b: f64) -> bool {
        let ab = FPHelper::double_to_i64(a);
        let bb = FPHelper::double_to_i64(b);
        return ab.high == bb.high && ab.low == bb.low;
    }

    pub(crate) fn snapshot_table_binary_pooled_float(pool: &mut Vec<f64>, value: f64) -> u32 {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if SnapshotTableBinary::snapshot_table_binary_f64_bits_equal(pool[usize::try_from(index_idx).unwrap_or(0)], value) {
                return index_idx;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        pool.push(value);
        return u32::wrapping_sub(u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
    }

    pub(crate) fn snapshot_table_binary_pooled_style(pool: &mut Vec<StyleRow>, probe: TableProbe, script_ref: u32, language_ref: u32) -> u32 {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let row = (pool[usize::try_from(index_idx).unwrap_or(0)]).clone();
            if SnapshotTableBinary::snapshot_table_binary_f64_bits_equal(row.font_size_px, probe.font_size_px) && SnapshotTableBinary::snapshot_table_binary_f64_bits_equal(row.font_weight, probe.font_weight) && row.italic == if probe.italic { 1 } else { 0 } && row.script_ref ==
script_ref && row.language_ref == language_ref {
                return index_idx;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        pool.push(StyleRow::new(probe.font_size_px, probe.font_weight, if probe.italic { 1 } else { 0 }, script_ref, language_ref));
        return u32::wrapping_sub(u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
    }

    pub(crate) fn snapshot_table_binary_pooled_features(pool: &mut Vec<Vec<u32>>, refs: &Vec<u32>) -> u32 {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if SnapshotTableBinary::snapshot_table_binary_same_refs(&(pool[usize::try_from(index_idx).unwrap_or(0)]).clone(), &refs) {
                return index_idx;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        pool.push((*refs).clone());
        return u32::wrapping_sub(u32::try_from((pool.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
    }

    pub(crate) fn snapshot_table_binary_same_refs(a: &Vec<u32>, b: &Vec<u32>) -> bool {
        if u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if a[usize::try_from(index_idx).unwrap_or(0)] != b[usize::try_from(index_idx).unwrap_or(0)] {
                return false;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return true;
    }

    pub(crate) fn snapshot_table_binary_write_text_region(writer: &mut TableWriter, texts: &Vec<String>) {
        let mut total_idx = 0u32;
        while (i32::from_ne_bytes((total_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((texts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.u32(u32::try_from(((texts[usize::try_from(total_idx).unwrap_or(0)]).clone().as_bytes().to_vec().len()) & 0xFFFF_FFFF).unwrap_or(0));
            total_idx = u32::wrapping_add(total_idx, 1);
        }
        total_idx = 0u32;
        while (i32::from_ne_bytes((total_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((texts.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            writer.raw(&(texts[usize::try_from(total_idx).unwrap_or(0)]).clone().as_bytes().to_vec());
            total_idx = u32::wrapping_add(total_idx, 1);
        }
    }

    pub(crate) fn snapshot_table_binary_write_value_row(writer: &mut TableWriter, row: ValueRow) {
        let mut slot_idx = 0u32;
        while (i32::from_ne_bytes((slot_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((row.values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let value = row.values[usize::try_from(slot_idx).unwrap_or(0)];
            match &(value) {
                None => {
                    writer.absent_f64();
                }
                Some(__option1) => {
                    writer.f64(*__option1);
                }
            }
            slot_idx = u32::wrapping_add(slot_idx, 1);
        }
    }

    pub(crate) fn snapshot_table_binary_feature_row_bytes(refs: &Vec<u32>) -> Vec<u8> {
        let mut row = TableWriter::new();
        row.u16(u32::try_from((refs.len()) & 0xFFFF_FFFF).unwrap_or(0));
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((refs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            row.u32(refs[usize::try_from(index_idx).unwrap_or(0)]);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return row.finish();
    }

    pub(crate) fn snapshot_table_binary_read_value_row(reader: &mut TableReader) -> ValueRow {
        let mut values: Vec<Option<f64>> = Vec::new();
        for _ in 0..5 {
            let low = reader.u32();
            let high = reader.u32();
            if high == 2146959360 && low == 0 {
                values.push(None);
            } else {
                values.push(Some(FPHelper::i64_to_double(low, high)));
            }
        }
        return ValueRow::new(values.to_vec());
    }
}

#[derive(Clone, PartialEq)]
pub struct StringInterner {
    pub strings: Vec<String>,
}

impl StringInterner {
    pub fn new(replay_strings: Vec<String>) -> Self {
    let mut strings = Vec::new();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((replay_strings.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            strings.push((replay_strings[usize::try_from(index_idx).unwrap_or(0)]).clone());
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        Self {
            strings: strings,
        }
    }

    pub fn intern(&mut self, text: &str) -> u32 {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((self.strings.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if self.strings[usize::try_from(index_idx).unwrap_or(0)].clone() == text {
                return index_idx;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        self.strings.push(text.to_string());
        return u32::wrapping_sub(u32::try_from((self.strings.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
    }
}

pub struct TableWriter {
    pub(crate) buf: BytesBuffer,
}

impl TableWriter {
    pub fn new() -> Self {
        Self {
            buf: BytesBuffer::new(),
        }
    }

    pub fn raw(&mut self, bytes: &[u8]) {
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            self.buf.add_byte(u8::try_from((u32::from(bytes[usize::try_from(index_idx).unwrap_or(0)])) & 0xFF).unwrap_or(0));
            index_idx = u32::wrapping_add(index_idx, 1);
        }
    }

    pub fn u8(&mut self, value: u32) {
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
    }

    pub fn u16(&mut self, value: u32) {
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[2])) & 0xFF).unwrap_or(0));
    }

    pub fn u32(&mut self, value: u32) {
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[2])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[1])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[0])) & 0xFF).unwrap_or(0));
    }

    pub fn f64(&mut self, value: f64) {
        let bits = FPHelper::double_to_i64(value);
        self.u32(bits.low);
        self.u32(bits.high);
    }

    pub fn absent_f64(&mut self) {
        self.u32(0);
        self.u32(2146959360);
    }

    pub fn finish(&mut self) -> Vec<u8> {
        return self.buf.get_bytes();
    }
}

pub struct TableReader<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) pos: u32,
    pub failed: bool,
    pub issue: String,
}

impl<'a> TableReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            pos: 0,
            failed: false,
            issue: "".to_string(),
        }
    }

    pub fn u8(&mut self) -> u32 {
        if !self.need(1) {
            return 0;
        }
        let value = u32::from(self.bytes[usize::try_from(self.pos).unwrap_or(0)]);
        self.pos += 1;
        return value;
    }

    pub fn u16(&mut self) -> u32 {
        if !self.need(2) {
            return 0;
        }
        let value = u32::from(self.bytes[usize::try_from(self.pos).unwrap_or(0)]) | (u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, 1)).unwrap_or(0)])) << (8);
        self.pos += 2;
        return value;
    }

    pub fn u32(&mut self) -> u32 {
        if !self.need(4) {
            return 0;
        }
        let value = u32::wrapping_add(u32::from(self.bytes[usize::try_from(self.pos).unwrap_or(0)]) | (u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, 1)).unwrap_or(0)])) << (8) | (u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos,
2)).unwrap_or(0)])) << (16), u32::wrapping_mul(u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, 3)).unwrap_or(0)]), 16777216));
        self.pos += 4;
        return value;
    }

    pub fn f64(&mut self) -> f64 {
        let low = self.u32();
        let high = self.u32();
        if self.failed {
            return 0.0f64;
        }
        return FPHelper::i64_to_double(low, high);
    }

    pub fn match_magic(&mut self) -> bool {
        if !self.need(8) {
            return false;
        }
        let expected = "TIQTBL03".as_bytes().to_vec();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (8) {
            if u32::from(self.bytes[usize::try_from(u32::wrapping_add(self.pos, index_idx)).unwrap_or(0)]) != u32::from(expected[usize::try_from(index_idx).unwrap_or(0)]) {
                self.failed = true;
                self.issue = "SnapshotTablesInvalid".to_string();
                return false;
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        self.pos += 8;
        return true;
    }

    pub fn feature_row(&mut self) -> Vec<u32> {
        let mut refs: Vec<u32> = Vec::new();
        let count = self.u16();
        if self.failed {
            return refs;
        }
        for _ in 0..count {
            refs.push(self.u32());
        }
        return refs;
    }

    pub fn text_region(&mut self, count: u32) -> Vec<String> {
        let mut texts: Vec<String> = Vec::new();
        if self.failed {
            return texts;
        }
        let mut lengths: Vec<u32> = Vec::new();
        let mut running = 0u32;
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((count).to_ne_bytes())) {
            let delta = self.u32();
            if self.failed {
                return texts;
            }
            running = u32::wrapping_add(running, delta);
            if running > 2147483647 || (i32::from_ne_bytes((u32::wrapping_add(self.pos, running)).to_ne_bytes())) > (i32::from_ne_bytes((u32::try_from((self.bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                self.failed = true;
                self.issue = "SnapshotTablesInvalid".to_string();
                return texts;
            }
            lengths.push(delta);
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        index_idx = 0u32;
        while (i32::from_ne_bytes((index_idx).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((lengths.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            texts.push(String::from_utf8_lossy(&self.bytes[usize::try_from(self.pos).unwrap_or(0)..usize::try_from(self.pos + lengths[usize::try_from(index_idx).unwrap_or(0)]).unwrap_or(0)]).into_owned());
            self.pos += lengths[usize::try_from(index_idx).unwrap_or(0)];
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return texts;
    }

    pub fn rest_text(&mut self) -> String {
        if !self.need(0) {
            return String::new();
        }
        let value = String::from_utf8_lossy(&self.bytes[usize::try_from(self.pos).unwrap_or(0)..usize::try_from(self.pos + u32::wrapping_sub(u32::try_from((self.bytes.len()) & 0xFFFF_FFFF).unwrap_or(0), self.pos)).unwrap_or(0)]).into_owned();
        self.pos = u32::try_from((self.bytes.len()) & 0xFFFF_FFFF).unwrap_or(0);
        return value;
    }

    fn need(&mut self, length: u32) -> bool {
        if self.failed {
            return false;
        }
        if length > 2147483647 || (i32::from_ne_bytes((u32::wrapping_add(self.pos, length)).to_ne_bytes())) > (i32::from_ne_bytes((u32::try_from((self.bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            self.failed = true;
            self.issue = "SnapshotTablesInvalid".to_string();
            return false;
        }
        return true;
    }
}
