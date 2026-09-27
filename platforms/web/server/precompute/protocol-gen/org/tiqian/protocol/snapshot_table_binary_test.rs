#![cfg(test)]

use crate::org::tiqian::protocol::decode_result::DecodeResult;
use crate::org::tiqian::protocol::snapshot_table_binary::SnapshotTableBinary;
use crate::org::tiqian::protocol::snapshot_table_test_support::SnapshotTableTestSupport;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn golden_vector_pins_the_shared_bytes() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.goldenVectorPinsTheSharedBytes", "org.tiqian.protocol.SnapshotTableBinaryTest.goldenVectorPinsTheSharedBytes", || {
        let mut recorder = TestTraceRecorder::new("SnapshotTableBinaryTest");
        recorder.section(&"goldenVectorPinsTheSharedBytes");
        let _ = SnapshotTableTestSupport::snapshot_table_test_support_assert_hex_bytes((recorder).clone(), &"table-golden",
&"54495154424c3033010000000a00000002000000010000000200000001000000010000000100000001000000000000000100000000000000080000000500000004000000040000000600000003000000040000000200000004000000030000007265706c61792d61417269616c73616e7366732d31e5ae8be4bd93e6b18948616e737a686c696761e5ad970100000004000000000000000000794000000000000079400001020000000200000003000000030000000000000000000000000000000000f83f000000000000f87f0000000000000040000000000000f87f000000000000f87f0500000009000000000000000000000000000000000000000000294000000000000030400000000000007940000600000007000000060000000100080000000e0000007b2266616d696c79223a2246227d0400000076732d307b226261636b656e645265766973696f6e223a227231227d", &SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()).unwrap()).unwrap();
    });
}

#[test]
fn read_then_write_reproduces_the_bytes() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.readThenWriteReproducesTheBytes", "org.tiqian.protocol.SnapshotTableBinaryTest.readThenWriteReproducesTheBytes", || {
        let mut recorder = TestTraceRecorder::new("SnapshotTableBinaryTest");
        recorder.section(&"readThenWriteReproducesTheBytes");
        let encoded = SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()).unwrap();
        {
            let _g = SnapshotTableBinary::snapshot_table_binary_decode(&encoded).unwrap();
            let _ = match _g {
    DecodeResult::TOk { data: _p0 } => SnapshotTableTestSupport::snapshot_table_test_support_assert_hex_bytes((recorder).clone(), &"reencode", SnapshotTableTestSupport::snapshot_table_test_support_hex_bytes(&encoded).as_str(),
&SnapshotTableBinary::snapshot_table_binary_encode_data((_p0).clone())).unwrap(),
    DecodeResult::TErr { issue: _p0 } => {
    recorder.record(format!("{}{}",
            "decode unexpectedly failed: ",
            _p0
        ).as_str()).unwrap();
    TracedAssertions::traced_assertions_assert_true(false, Some("decode of a fresh encoding must succeed".to_string())).unwrap()
},
};
        }
    });
}

#[test]
fn write_then_read_restores_the_content() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.writeThenReadRestoresTheContent", "org.tiqian.protocol.SnapshotTableBinaryTest.writeThenReadRestoresTheContent", || {
        let mut recorder = TestTraceRecorder::new("SnapshotTableBinaryTest");
        recorder.section(&"writeThenReadRestoresTheContent");
        {
            let _g = SnapshotTableBinary::snapshot_table_binary_decode(&SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()).unwrap()).unwrap();
            let _ = match _g {
    DecodeResult::TOk { data: _p0 } => {
    TracedAssertions::traced_assertions_assert_equals_int(1, _p0.replay_string_count, None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(10, u32::try_from((_p0.strings.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_string(&"replay-a", (_p0.strings[0usize]).clone().as_str(), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_string(&"宋体", (_p0.strings[4usize]).clone().as_str(), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((_p0.metric_rows.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, _p0.metric_rows[0usize].families_ref, None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(0, _p0.metric_rows[1usize].italic, None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(2, u32::try_from((_p0.probe_text_refs.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.advance_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.style_font_size.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from(((_p0.features_pool[0usize]).clone().len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_string(&"{\"family\":\"F\"}", (_p0.face_texts[0usize]).clone().as_str(), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_string(&"vs-0", (_p0.value_style_texts[0usize]).clone().as_str(), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_string(&"{\"backendRevision\":\"r1\"}", (_p0.revision_text).to_string().as_str(), None).unwrap()
},
    DecodeResult::TErr { issue: _p0 } => {
    recorder.record(format!("{}{}",
            "decode unexpectedly failed: ",
            _p0
        ).as_str()).unwrap();
    TracedAssertions::traced_assertions_assert_true(false, Some("decode of a fresh encoding must succeed".to_string())).unwrap()
},
};
        }
    });
}

#[test]
fn pool_deduplication_collapses_equal_rows() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.poolDeduplicationCollapsesEqualRows", "org.tiqian.protocol.SnapshotTableBinaryTest.poolDeduplicationCollapsesEqualRows", || {
        let mut recorder = TestTraceRecorder::new("SnapshotTableBinaryTest");
        recorder.section(&"poolDeduplicationCollapsesEqualRows");
        {
            let _g = SnapshotTableBinary::snapshot_table_binary_decode(&SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()).unwrap()).unwrap();
            let _ = match _g {
    DecodeResult::TOk { data: _p0 } => {
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.value_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.advance_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.style_font_size.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(1, u32::try_from((_p0.features_pool.len()) & 0xFFFF_FFFF).unwrap_or(0), None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(0, _p0.probe_style_refs[0usize], None).unwrap();
    TracedAssertions::traced_assertions_assert_equals_int(0, _p0.probe_style_refs[1usize], None).unwrap()
},
    DecodeResult::TErr { issue: _p0 } => {
    recorder.record(format!("{}{}",
            "decode unexpectedly failed: ",
            _p0
        ).as_str()).unwrap();
    TracedAssertions::traced_assertions_assert_true(false, Some("decode must succeed".to_string())).unwrap()
},
};
        }
    });
}

#[test]
fn damaged_files_come_back_as_the_named_issue() {
    testlib::run("org.tiqian.protocol.SnapshotTableBinaryTest.damagedFilesComeBackAsTheNamedIssue", "org.tiqian.protocol.SnapshotTableBinaryTest.damagedFilesComeBackAsTheNamedIssue", || {
        let mut recorder = TestTraceRecorder::new("SnapshotTableBinaryTest");
        recorder.section(&"damagedFilesComeBackAsTheNamedIssue");
        {
            let _g = SnapshotTableBinary::snapshot_table_binary_decode(&"TIQ".as_bytes().to_vec()).unwrap();
            let _ = match _g {
    DecodeResult::TOk { .. } => {
    recorder.record(&"decode unexpectedly succeeded").unwrap();
    TracedAssertions::traced_assertions_assert_true(false, Some("a short file must fail".to_string())).unwrap()
},
    DecodeResult::TErr { issue: _p0 } => TracedAssertions::traced_assertions_assert_equals_string(&"SnapshotTablesInvalid", _p0.as_str(), None).unwrap(),
};
        }
        let mut corrupted = SnapshotTableBinary::snapshot_table_binary_encode(SnapshotTableTestSupport::snapshot_table_test_support_golden_input()).unwrap();
        corrupted[3usize] = 88u8;
        {
            let _g = SnapshotTableBinary::snapshot_table_binary_decode(&corrupted).unwrap();
            let _ = match _g {
    DecodeResult::TOk { .. } => {
    recorder.record(&"decode unexpectedly succeeded").unwrap();
    TracedAssertions::traced_assertions_assert_true(false, Some("a corrupted magic must fail".to_string())).unwrap()
},
    DecodeResult::TErr { issue: _p0 } => TracedAssertions::traced_assertions_assert_equals_string(&"SnapshotTablesInvalid", _p0.as_str(), None).unwrap(),
};
        }
    });
}
