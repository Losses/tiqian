use crate::org::tiqian::clreq::bopomofo_reading::BopomofoReading;
use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct BopomofoParser;

impl BopomofoParser {
    const BOPOMOFO_PARSER_NEUTRAL_MARK: u32 = 729;
    const BOPOMOFO_PARSER_YANGPING_MARK: u32 = 714;
    const BOPOMOFO_PARSER_SHANG_MARK: u32 = 711;
    const BOPOMOFO_PARSER_QU_MARK: u32 = 715;
    const BOPOMOFO_PARSER_YINPING_MACRON_MARK: u32 = 713;

    pub fn bopomofo_parser_parse(reading: &str) -> BopomofoReading {
    let __units = u_string::units(&reading);
    let __count = u_string::unit_count(&reading);
        if __count == 0 {
            return BopomofoReading::new(vec![].to_vec(), BopomofoTone::Yinping);
        }
        if u_string::unit_at_from(&__units, 0u32).as_ref().map_or(false, |v| v == &(BopomofoParser::BOPOMOFO_PARSER_NEUTRAL_MARK)) {
            return BopomofoReading::new(BopomofoParser::bopomofo_parser_symbols_of(u_string::substring_from(&reading, 1i32).as_str()).to_vec(), BopomofoTone::Neutral);
        }
        let last = u_string::unit_at_from(&__units, u32::wrapping_sub(__count, 1)).unwrap_or(0);
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
        let body = if has_suffix_mark { u_string::substring(&reading, 0i32, i32::from_ne_bytes((u32::wrapping_sub(__count, 1)).to_ne_bytes())).to_string() } else { reading.to_string() };
        return BopomofoReading::new(BopomofoParser::bopomofo_parser_symbols_of(body.as_str()).to_vec(), tone);
    }

    pub(crate) fn bopomofo_parser_symbols_of(body: &str) -> Vec<String> {
    let __units1 = u_string::units(&body);
    let __count1 = u_string::unit_count(&body);
        let mut symbols: Vec<String> = vec![];
        let mut index = 0u32;
        let __units2 = u_string::units(&body);
        let __count2 = u_string::unit_count(&body);
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((__count1).to_ne_bytes())) {
            symbols.push(u_string::char_at_from(&__units2, index));
            index = u32::wrapping_add(index, 1);
        }
        return symbols;
    }
}
