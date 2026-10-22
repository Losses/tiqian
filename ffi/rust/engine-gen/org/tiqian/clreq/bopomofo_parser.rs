use crate::org::tiqian::clreq::bopomofo_reading::BopomofoReading;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct BopomofoParser;

impl BopomofoParser {
    const BOPOMOFO_PARSER_NEUTRAL_MARK: u32 = 729;
    const BOPOMOFO_PARSER_YANGPING_MARK: u32 = 714;
    const BOPOMOFO_PARSER_SHANG_MARK: u32 = 711;
    const BOPOMOFO_PARSER_QU_MARK: u32 = 715;
    const BOPOMOFO_PARSER_YINPING_MACRON_MARK: u32 = 713;

    pub fn bopomofo_parser_parse(reading: &UStr) -> BopomofoReading {
    let __units = reading.as_slice();
    let __count = u_string::unit_count(&reading);
        if __count == 0 {
            return BopomofoReading::new(vec![].to_vec(), BopomofoTone::Yinping);
        }
        if matches!(u_string::unit_at_from(__units, 0u32).as_ref(), Some(__v) if __v == &(BopomofoParser::BOPOMOFO_PARSER_NEUTRAL_MARK)) {
            return BopomofoReading::new(BopomofoParser::bopomofo_parser_symbols_of(u_string::substring_from(&reading, 1i32).as_ustr()).to_vec(), BopomofoTone::Neutral);
        }
        let last = u_string::unit_at_from(__units, u32::wrapping_sub(__count, 1)).unwrap_or(0);
        let mut tone = BopomofoTone::Yinping;
        if last == BopomofoParser::BOPOMOFO_PARSER_YANGPING_MARK {
            tone = BopomofoTone::Yangping;
        } else {
            if last == BopomofoParser::BOPOMOFO_PARSER_SHANG_MARK {
                tone = BopomofoTone::Shang;
            } else {
                if last == BopomofoParser::BOPOMOFO_PARSER_QU_MARK {
                    tone = BopomofoTone::Qu;
                }
            }
        }
        let has_suffix_mark = last == BopomofoParser::BOPOMOFO_PARSER_YANGPING_MARK || last == BopomofoParser::BOPOMOFO_PARSER_SHANG_MARK || last == BopomofoParser::BOPOMOFO_PARSER_QU_MARK || last == BopomofoParser::BOPOMOFO_PARSER_YINPING_MACRON_MARK;
        let body = if has_suffix_mark { u_string::substring(&reading, 0i32, (u32::wrapping_sub(__count, 1)) as i32).to_ustring() } else { reading.to_ustring() };
        return BopomofoReading::new(BopomofoParser::bopomofo_parser_symbols_of(body.as_ustr()).to_vec(), tone);
    }

    pub(crate) fn bopomofo_parser_symbols_of(body: &UStr) -> Vec<UString> {
    let __units1 = body.as_slice();
    let __count1 = u_string::unit_count(&body);
        let mut symbols: Vec<UString> = vec![];
        let mut index = 0u32;
        while ((index) as i32) < ((__count1) as i32) {
            symbols.push(u_string::char_at_from(__units1, index));
            index = u32::wrapping_add(index, 1);
        }
        return symbols;
    }
}
