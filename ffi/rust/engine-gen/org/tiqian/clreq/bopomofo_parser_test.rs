#![cfg(test)]

use crate::org::tiqian::clreq::bopomofo_parser::BopomofoParser;
use crate::org::tiqian::clreq::bopomofo_reading::BopomofoReading;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[test]
fn yinping_has_no_mark() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.yinpingHasNoMark", "org.tiqian.clreq.BopomofoParserTest.yinpingHasNoMark", || {
        TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,80,97,114,115,101,114,84,101,115,116]))).section(UStr::new(&[121,105,110,112,105,110,103,72,97,115,78,111,77,97,114,107]));
        let reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[12563,12584,12581]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![
    UString::from("ㄓ").to_ustring(),
    UString::from("ㄨ").to_ustring(),
    UString::from("ㄥ").to_ustring(),
], &reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, reading.tone, None).unwrap();
    });
}

#[test]
fn suffix_marks_are_tone_and_stripped() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.suffixMarksAreToneAndStripped", "org.tiqian.clreq.BopomofoParserTest.suffixMarksAreToneAndStripped", || {
        TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,80,97,114,115,101,114,84,101,115,116]))).section(UStr::new(&[115,117,102,102,105,120,77,97,114,107,115,65,114,101,84,111,110,101,65,110,100,83,116,114,105,112,112,101,100]));
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec![UString::from("ㄔ").to_ustring(), UString::from("ㄤ").to_ustring()].to_vec(), BopomofoTone::Yangping), BopomofoParser::bopomofo_parser_parse(UStr::new(&[12564,12580,714])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec![UString::from("ㄋ").to_ustring(), UString::from("ㄧ").to_ustring()].to_vec(), BopomofoTone::Shang), BopomofoParser::bopomofo_parser_parse(UStr::new(&[12555,12583,711])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec![UString::from("ㄑ").to_ustring(), UString::from("ㄩ").to_ustring()].to_vec(), BopomofoTone::Qu), BopomofoParser::bopomofo_parser_parse(UStr::new(&[12561,12585,715])), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec![UString::from("ㄇ").to_ustring(), UString::from("ㄚ").to_ustring()].to_vec(), BopomofoTone::Yinping), BopomofoParser::bopomofo_parser_parse(UStr::new(&[12551,12570,713])), None).unwrap();
    });
}

#[test]
fn neutral_tone_is_prefixed() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.neutralToneIsPrefixed", "org.tiqian.clreq.BopomofoParserTest.neutralToneIsPrefixed", || {
        TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,80,97,114,115,101,114,84,101,115,116]))).section(UStr::new(&[110,101,117,116,114,97,108,84,111,110,101,73,115,80,114,101,102,105,120,101,100]));
        let reading = BopomofoParser::bopomofo_parser_parse(UStr::new(&[729,12553,12572]));
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec![UString::from("ㄉ").to_ustring(), UString::from("ㄜ").to_ustring()], &reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Neutral, reading.tone, None).unwrap();
    });
}

#[test]
fn single_symbol() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.singleSymbol", "org.tiqian.clreq.BopomofoParserTest.singleSymbol", || {
        TestTraceRecorder::new(&(UStr::new(&[66,111,112,111,109,111,102,111,80,97,114,115,101,114,84,101,115,116]))).section(UStr::new(&[115,105,110,103,108,101,83,121,109,98,111,108]));
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec![UString::from("ㄦ").to_ustring()].to_vec(), BopomofoTone::Yangping), BopomofoParser::bopomofo_parser_parse(UStr::new(&[12582,714])), None).unwrap();
    });
}
