#![cfg(test)]

use crate::org::tiqian::clreq::bopomofo_parser::BopomofoParser;
use crate::org::tiqian::clreq::bopomofo_reading::BopomofoReading;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::runtime::test as testlib;


#[test]
fn yinping_has_no_mark() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.yinpingHasNoMark", "org.tiqian.clreq.BopomofoParserTest.yinpingHasNoMark", || {
        TestTraceRecorder::new("BopomofoParserTest").section(&"yinpingHasNoMark");
        let reading = BopomofoParser::bopomofo_parser_parse(&"ㄓㄨㄥ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄓ".to_string(), "ㄨ".to_string(), "ㄥ".to_string()], &reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Yinping, reading.tone, None).unwrap();
    });
}

#[test]
fn suffix_marks_are_tone_and_stripped() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.suffixMarksAreToneAndStripped", "org.tiqian.clreq.BopomofoParserTest.suffixMarksAreToneAndStripped", || {
        TestTraceRecorder::new("BopomofoParserTest").section(&"suffixMarksAreToneAndStripped");
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec!["ㄔ".to_string(), "ㄤ".to_string()].to_vec(), BopomofoTone::Yangping), BopomofoParser::bopomofo_parser_parse(&"ㄔㄤˊ"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec!["ㄋ".to_string(), "ㄧ".to_string()].to_vec(), BopomofoTone::Shang), BopomofoParser::bopomofo_parser_parse(&"ㄋㄧˇ"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec!["ㄑ".to_string(), "ㄩ".to_string()].to_vec(), BopomofoTone::Qu), BopomofoParser::bopomofo_parser_parse(&"ㄑㄩˋ"), None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec!["ㄇ".to_string(), "ㄚ".to_string()].to_vec(), BopomofoTone::Yinping), BopomofoParser::bopomofo_parser_parse(&"ㄇㄚˉ"), None).unwrap();
    });
}

#[test]
fn neutral_tone_is_prefixed() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.neutralToneIsPrefixed", "org.tiqian.clreq.BopomofoParserTest.neutralToneIsPrefixed", || {
        TestTraceRecorder::new("BopomofoParserTest").section(&"neutralToneIsPrefixed");
        let reading = BopomofoParser::bopomofo_parser_parse(&"˙ㄉㄜ");
        let _ = TracedAssertions::traced_assertions_assert_equals_string_array(&vec!["ㄉ".to_string(), "ㄜ".to_string()], &reading.symbols, None).unwrap();
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_tone(BopomofoTone::Neutral, reading.tone, None).unwrap();
    });
}

#[test]
fn single_symbol() {
    testlib::run("org.tiqian.clreq.BopomofoParserTest.singleSymbol", "org.tiqian.clreq.BopomofoParserTest.singleSymbol", || {
        TestTraceRecorder::new("BopomofoParserTest").section(&"singleSymbol");
        let _ = TracedAssertions::traced_assertions_assert_equals_bopomofo_reading(BopomofoReading::new(vec!["ㄦ".to_string()].to_vec(), BopomofoTone::Yangping), BopomofoParser::bopomofo_parser_parse(&"ㄦˊ"), None).unwrap();
    });
}
