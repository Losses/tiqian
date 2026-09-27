#![cfg(test)]

use crate::org::tiqian::protocol::snapshot_table_binary::SnapshotTableBinary;
use crate::org::tiqian::protocol::snapshot_table_test_support::SnapshotTableTestSupport;
use crate::org::tiqian::protocol::table_data::TableData;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn golden_vector_pins_the_shared_bytes() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.goldenVectorPinsTheSharedBytes", "org.tiqian.protocol.SnapshotTableBinaryTest.goldenVectorPinsTheSharedBytes", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,66,105,110,97,114,121,84,101,115,116])));
        recorder.section(UStr::new(&[103,111,108,100,101,110,86,101,99,116,111,114,80,105,110,115,84,104,101,83,104,97,114,101,100,66,121,116,101,115]));
        let _ = SnapshotTableTestSupport::snapshot_table_test_support_assert_hex_bytes((recorder).clone(), UStr::new(&[116,97,98,108,101,45,103,111,108,100,101,110]), UStr::new(&[53,52,52,57,53,49,53,52,52,50,52,99,51,48,51,51,48,49,48,48,48,48,48,48,48,97,48,48,48,48,48,48,48,50,48,48,48,48,48,48,48,49,48,48,48,48,48,48,48,50,48,48,48,48,48,48,48,49,48,48,48,48,48,48,48,49,48,48,48,48,48,48,48,49,48,48,48,48,48,48,48,49,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,49,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,56,48,48,48,48,48,48,48,53,48,48,48,48,48,48,48,52,48,48,48,48,48,48,48,52,48,48,48,48,48,48,48,54,48,48,48,48,48,48,48,51,48,48,48,48,48,48,48,52,48,48,48,48,48,48,48,50,48,48,48,48,48,48,48,52,48,48,48,48,48,48,48,51,48,48,48,48,48,48,55,50,54,53,55,48,54,99,54,49,55,57,50,100,54,49,52,49,55,50,54,57,54,49,54,99,55,51,54,49,54,101,55,51,54,54,55,51,50,100,51,49,101,53,97,101,56,98,101,52,98,100,57,51,101,54,98,49,56,57,52,56,54,49,54,101,55,51,55,97,54,56,54,99,54,57,54,55,54,49,101,53,97,100,57,55,48,49,48,48,48,48,48,48,48,52,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,55,57,52,48,48,48,48,48,48,48,48,48,48,48,48,48,55,57,52,48,48,48,48,49,48,50,48,48,48,48,48,48,48,50,48,48,48,48,48,48,48,51,48,48,48,48,48,48,48,51,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,102,56,51,102,48,48,48,48,48,48,48,48,48,48,48,48,102,56,55,102,48,48,48,48,48,48,48,48,48,48,48,48,48,48,52,48,48,48,48,48,48,48,48,48,48,48,48,48,102,56,55,102,48,48,48,48,48,48,48,48,48,48,48,48,102,56,55,102,48,53,48,48,48,48,48,48,48,57,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,50,57,52,48,48,48,48,48,48,48,48,48,48,48,48,48,51,48,52,48,48,48,48,48,48,48,48,48,48,48,48,48,55,57,52,48,48,48,48,54,48,48,48,48,48,48,48,55,48,48,48,48,48,48,48,54,48,48,48,48,48,48,48,49,48,48,48,56,48,48,48,48,48,48,48,101,48,48,48,48,48,48,55,98,50,50,54,54,54,49,54,100,54,57,54,99,55,57,50,50,51,97,50,50,52,54,50,50,55,100,48,52,48,48,48,48,48,48,55,54,55,51,50,100,51,48,55,98,50,50,54,50,54,49,54,51,54,98,54,53,54,101,54,52,53,50,54,53,55,54,54,57,55,51,54,57,54,102,54,101,50,50,51,97,50,50,55,50,51,49,50,50,55,100]), &SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input())).unwrap();
    });
}

#[test]
fn read_then_write_reproduces_the_bytes() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.readThenWriteReproducesTheBytes", "org.tiqian.protocol.SnapshotTableBinaryTest.readThenWriteReproducesTheBytes", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,66,105,110,97,114,121,84,101,115,116])));
        recorder.section(UStr::new(&[114,101,97,100,84,104,101,110,87,114,105,116,101,82,101,112,114,111,100,117,99,101,115,84,104,101,66,121,116,101,115]));
        let encoded = SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input());
        let mut data = TableData::new();
        let issue = SnapshotTableBinary::snapshot_table_binary_decode_into(&encoded, &mut data);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), issue.as_ustr(), None).unwrap();
        let _ = SnapshotTableTestSupport::snapshot_table_test_support_assert_hex_bytes((recorder).clone(), UStr::new(&[114,101,101,110,99,111,100,101]), SnapshotTableTestSupport::snapshot_table_test_support_hex_bytes(&encoded).as_ustr(), &SnapshotTableBinary::snapshot_table_binary_encode_data((data).clone())).unwrap();
    });
}

#[test]
fn write_then_read_restores_the_content() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.writeThenReadRestoresTheContent", "org.tiqian.protocol.SnapshotTableBinaryTest.writeThenReadRestoresTheContent", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,66,105,110,97,114,121,84,101,115,116])));
        recorder.section(UStr::new(&[119,114,105,116,101,84,104,101,110,82,101,97,100,82,101,115,116,111,114,101,115,84,104,101,67,111,110,116,101,110,116]));
        let mut data = TableData::new();
        let issue = SnapshotTableBinary::snapshot_table_binary_decode_into(&SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()), &mut data);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), issue.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, data.replay_string_count, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(10, u32::try_from((data.strings.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[114,101,112,108,97,121,45,97]), (data.strings[0usize]).clone().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[23435,20307]), (data.strings[4usize]).clone().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((data.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, data.metric_rows[0usize].families_ref, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, data.metric_rows[1usize].italic, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((data.probe_text_refs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.advance_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.style_font_size.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((data.features_pool[0usize]).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[123,34,102,97,109,105,108,121,34,58,34,70,34,125]), (data.face_texts[0usize]).clone().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[118,115,45,48]), (data.value_style_texts[0usize]).clone().as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[123,34,98,97,99,107,101,110,100,82,101,118,105,115,105,111,110,34,58,34,114,49,34,125]), (data.revision_text).to_ustring().as_ustr(), None).unwrap();
    });
}

#[test]
fn pool_deduplication_collapses_equal_rows() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.poolDeduplicationCollapsesEqualRows", "org.tiqian.protocol.SnapshotTableBinaryTest.poolDeduplicationCollapsesEqualRows", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,66,105,110,97,114,121,84,101,115,116])));
        recorder.section(UStr::new(&[112,111,111,108,68,101,100,117,112,108,105,99,97,116,105,111,110,67,111,108,108,97,112,115,101,115,69,113,117,97,108,82,111,119,115]));
        let mut data = TableData::new();
        let issue = SnapshotTableBinary::snapshot_table_binary_decode_into(&SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()), &mut data);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), issue.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.advance_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.style_font_size.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((data.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, data.probe_style_refs[0usize], None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, data.probe_style_refs[1usize], None).unwrap();
    });
}

#[test]
fn damaged_files_come_back_as_the_named_issue() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.damagedFilesComeBackAsTheNamedIssue", "org.tiqian.protocol.SnapshotTableBinaryTest.damagedFilesComeBackAsTheNamedIssue", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,66,105,110,97,114,121,84,101,115,116])));
        recorder.section(UStr::new(&[100,97,109,97,103,101,100,70,105,108,101,115,67,111,109,101,66,97,99,107,65,115,84,104,101,78,97,109,101,100,73,115,115,117,101]));
        let mut short = TableData::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,115,73,110,118,97,108,105,100]), SnapshotTableBinary::snapshot_table_binary_decode_into(&UString::from("TIQ").as_bytes().to_vec(), &mut short).as_ustr(), None).unwrap();
        let mut corrupted = SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input());
        corrupted[3usize] = 88u8;
        let mut bad_magic = TableData::new();
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,115,73,110,118,97,108,105,100]), SnapshotTableBinary::snapshot_table_binary_decode_into(&corrupted, &mut bad_magic).as_ustr(), None).unwrap();
    });
}

#[test]
fn pool_refs_survive_decode_reencode() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.poolRefsSurviveDecodeReencode", "org.tiqian.protocol.SnapshotTableBinaryTest.poolRefsSurviveDecodeReencode", || {
        let mut recorder = TestTraceRecorder::new(&(UStr::new(&[83,110,97,112,115,104,111,116,84,97,98,108,101,66,105,110,97,114,121,84,101,115,116])));
        recorder.section(UStr::new(&[112,111,111,108,82,101,102,115,83,117,114,118,105,118,101,68,101,99,111,100,101,82,101,101,110,99,111,100,101]));
        let input = SnapshotTableTestSupport::snapshot_table_test_support_two_pool_input();
        let bytes = SnapshotTableBinary::snapshot_table_binary_encode((input).clone());
        let mut data = TableData::new();
        let issue = SnapshotTableBinary::snapshot_table_binary_decode_into(&bytes, &mut data);
        let _ = TracedAssertions::traced_assertions_assert_equals_string(UStr::new(&[]), issue.as_ustr(), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((data.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(0, data.metric_rows[0usize].value_pool_ref, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_int(1, data.metric_rows[1usize].value_pool_ref, None).unwrap();
        let _ = SnapshotTableTestSupport::snapshot_table_test_support_assert_hex_bytes((recorder).clone(), UStr::new(&[114,101,102,114,101,101,122,101]), SnapshotTableTestSupport::snapshot_table_test_support_hex_bytes(&bytes).as_ustr(), &SnapshotTableBinary::snapshot_table_binary_encode_data((data).clone())).unwrap();
    });
}
