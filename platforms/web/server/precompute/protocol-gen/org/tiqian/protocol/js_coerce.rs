use crate::org::tiqian::protocol::wire_field::WireField;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct JsCoerce;

impl JsCoerce {
    pub fn js_coerce_to_number(value: WireValue) -> f64 {
        return match value {
            WireValue::WNull => 0.0f64,
            WireValue::WBool { value: _p0 } => if _p0 { 1.0f64 } else { 0.0f64 },
            WireValue::WNum { value: _p0 } => _p0,
            WireValue::WStr { value: _p0 } => JsCoerce::js_coerce_to_number_from_string(_p0.as_ustr()),
            WireValue::WArr { .. } => JsCoerce::js_coerce_to_number_from_string(JsCoerce::js_coerce_to_string((value).clone()).as_ustr()),
            WireValue::WObj { .. } => JsCoerce::js_coerce_to_number_from_string(JsCoerce::js_coerce_to_string((value).clone()).as_ustr()),
        };
    }

    pub fn js_coerce_to_string(value: WireValue) -> UString {
        return match value {
            WireValue::WNull => UString::from("null").to_ustring().to_ustring(),
            WireValue::WBool { value: _p0 } => if _p0 { UString::from("true") } else { UString::from("false") },
            WireValue::WNum { value: _p0 } => JsCoerce::js_coerce_number_to_string(_p0),
            WireValue::WStr { value: _p0 } => _p0,
            WireValue::WArr { items: _p0 } => JsCoerce::js_coerce_join_items(&_p0),
            WireValue::WObj { .. } => UString::from("[object Object]").to_ustring(),
        };
    }

    pub(crate) fn js_coerce_join_items(items: &Vec<WireValue>) -> UString {
        let mut parts: Vec<UString> = Vec::new();
        let mut item_index_idx = 0u32;
        while (i32::from_ne_bytes(((item_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let item = (items[usize::try_from(item_index_idx).unwrap_or(0)]).clone();
            parts.push(JsCoerce::js_coerce_item_text((item).clone()));
            item_index_idx = u32::wrapping_add(item_index_idx, 1);
        }
        return UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(","); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str());
    }

    pub(crate) fn js_coerce_item_text(item: WireValue) -> UString {
        return match item {
            WireValue::WNull => UString::from("").to_ustring().to_ustring(),
            WireValue::WBool { .. } => JsCoerce::js_coerce_to_string((item).clone()),
            WireValue::WNum { .. } => JsCoerce::js_coerce_to_string((item).clone()),
            WireValue::WStr { .. } => JsCoerce::js_coerce_to_string((item).clone()),
            WireValue::WArr { .. } => JsCoerce::js_coerce_to_string((item).clone()),
            WireValue::WObj { .. } => JsCoerce::js_coerce_to_string((item).clone()),
        };
    }

    pub fn js_coerce_number_to_string(value: f64) -> UString {
        if value.is_nan() {
            return UString::from("NaN").to_ustring();
        }
        if value == f64::INFINITY {
            return UString::from("Infinity").to_ustring();
        }
        if value == f64::NEG_INFINITY {
            return UString::from("-Infinity").to_ustring();
        }
        if value == 0.0f64 {
            return UString::from("0").to_ustring();
        }
        if value == i32::from_ne_bytes(((u32::from_ne_bytes(((match f64::from(f64::floor(value)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes())) as i32).to_ne_bytes()) as f64 && ((value).abs()) < (1e21f64) {
            let mut rest = (value).abs();
            let mut digits = UString::new();
            while (rest) >= 10 as f64 {
                let digit = u32::from_ne_bytes(((match f64::from(f64::floor(rest % 10 as f64)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes());
                digits = UString::from(format!("{}", { let mut __s = UString::new(); __s += (if u32::wrapping_add(48, digit) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_add(48, digit)) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(48, digit)) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_add(48, digit)) as u16]) }).as_ustr(); __s += digits.as_ustr(); __s }).as_str());
                rest = i32::from_ne_bytes(((u32::from_ne_bytes(((match f64::from(f64::floor(rest / format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0))) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes())) as i32).to_ne_bytes()) as f64 as f64;
            }
            digits = UString::from(format!("{}", { let mut __s = UString::new(); __s += (if u32::wrapping_add(48, u32::from_ne_bytes(((match f64::from(f64::floor(rest)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes())) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_add(48, u32::from_ne_bytes(((match f64::from(f64::floor(rest)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes()))) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(48, u32::from_ne_bytes(((match f64::from(f64::floor(rest)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes()))) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_add(48, u32::from_ne_bytes(((match f64::from(f64::floor(rest)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes()))) as u16]) }).as_ustr(); __s += digits.as_ustr(); __s }).as_str());
            return if value < (0 as f64) { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("-")); __s += digits.as_ustr(); __s }).as_str()) } else { digits.to_ustring() };
        }
        return UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(value)).as_str());
    }

    pub fn js_coerce_to_number_from_string(text: &UStr) -> f64 {
        let trimmed = JsCoerce::js_coerce_js_trim(text);
        if u_string::unit_count(&(trimmed)) == 0 {
            return 0.0f64;
        }
        let mut negative = false;
        let mut body = (trimmed).clone();
        let first = u_string::substring(&body, 0i32, i32::wrapping_add(0i32, 1));
        if first == UString::from("-") {
            negative = true;
            body = u_string::substr(&body, 1i32, None);
        } else {
            if first == UString::from("+") {
                body = u_string::substr(&body, 1i32, None);
            }
        }
        if body == UString::from("Infinity") {
            return if negative { f64::NEG_INFINITY } else { f64::INFINITY };
        }
        if body == UString::from("NaN") {
            return f64::NAN;
        }
        let radix = JsCoerce::js_coerce_radix_prefix(body.as_ustr());
        if i32::from_ne_bytes(((radix) as i32).to_ne_bytes()) > (0) {
            let digits = u_string::substr(&body, 2i32, None);
            let mut value = 0.0f64;
            let mut index_idx = 0u32;
            let __units = u_string::units(&digits);
            let __count = u_string::unit_count(&digits);
            while (i32::from_ne_bytes(((index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
                let digit = JsCoerce::js_coerce_hex_digit(*(u_string::unit_at_from(&__units, index_idx)).as_ref().unwrap());
                if digit > 2147483647 {
                    return f64::NAN;
                }
                value = value * format!("{}", (i32::from_ne_bytes(((radix) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) + format!("{}", (i32::from_ne_bytes(((digit) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0);
                index_idx = u32::wrapping_add(index_idx, 1);
            }
            return if negative { -value } else { value };
        }
        if !JsCoerce::js_coerce_is_strict_decimal(body.as_ustr()) {
            return f64::NAN;
        }
        let parsed = u_string::parse_f64(&(body));
        return if negative { -parsed } else { parsed };
    }

    pub(crate) fn js_coerce_radix_prefix(body: &UStr) -> u32 {
        if i32::from_ne_bytes(((u_string::unit_count(&(body))) as i32).to_ne_bytes()) < (3) {
            return 0;
        }
        let marker = u_string::substr(&body, 0i32, Some(2i32));
        if marker == UString::from("0x") || marker == UString::from("0X") {
            return 16;
        }
        if marker == UString::from("0o") || marker == UString::from("0O") {
            return 8;
        }
        if marker == UString::from("0b") || marker == UString::from("0B") {
            return 2;
        }
        return 0;
    }

    pub(crate) fn js_coerce_hex_digit(code: u32) -> u32 {
        if i32::from_ne_bytes(((code) as i32).to_ne_bytes()) >= 48 && (i32::from_ne_bytes(((code) as i32).to_ne_bytes())) <= 57 {
            return u32::wrapping_sub(code, 48);
        }
        if i32::from_ne_bytes(((code) as i32).to_ne_bytes()) >= 65 && (i32::from_ne_bytes(((code) as i32).to_ne_bytes())) <= 70 {
            return u32::wrapping_sub(code, 55);
        }
        if i32::from_ne_bytes(((code) as i32).to_ne_bytes()) >= 97 && (i32::from_ne_bytes(((code) as i32).to_ne_bytes())) <= 102 {
            return u32::wrapping_sub(code, 87);
        }
        return 4294967295u32;
    }

    pub(crate) fn js_coerce_is_strict_decimal(body: &UStr) -> bool {
    let __units1 = u_string::units(&body);
    let __count1 = u_string::unit_count(&body);
        let mut index = 0u32;
        let mut seen_digit = false;
        let __units2 = u_string::units(&body);
        let __count2 = u_string::unit_count(&body);
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) && JsCoerce::js_coerce_is_decimal_digit(*(u_string::unit_at_from(&__units1, index)).as_ref().unwrap()) {
            index = u32::wrapping_add(index, 1);
            seen_digit = true;
        }
        if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) && u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(46)) {
            index = u32::wrapping_add(index, 1);
            let __units3 = u_string::units(&body);
            let __count3 = u_string::unit_count(&body);
            while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) && JsCoerce::js_coerce_is_decimal_digit(*(u_string::unit_at_from(&__units1, index)).as_ref().unwrap()) {
                index = u32::wrapping_add(index, 1);
                seen_digit = true;
            }
        }
        if !seen_digit {
            return false;
        }
        if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) && (u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(101)) || u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(69))) {
            index = u32::wrapping_add(index, 1);
            if i32::from_ne_bytes(((index) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) && (u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(43)) || u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(45))) {
                index = u32::wrapping_add(index, 1);
            }
            let mut exponent_digits = false;
            let __units4 = u_string::units(&body);
            let __count4 = u_string::unit_count(&body);
            while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count1) as i32).to_ne_bytes())) && JsCoerce::js_coerce_is_decimal_digit(*(u_string::unit_at_from(&__units1, index)).as_ref().unwrap()) {
                index = u32::wrapping_add(index, 1);
                exponent_digits = true;
            }
            if !exponent_digits {
                return false;
            }
        }
        return index == __count1;
    }

    pub(crate) fn js_coerce_is_decimal_digit(code: u32) -> bool {
        return (i32::from_ne_bytes(((code) as i32).to_ne_bytes())) >= 48 && (i32::from_ne_bytes(((code) as i32).to_ne_bytes())) <= 57;
    }

    pub(crate) fn js_coerce_js_trim(text: &UStr) -> UString {
    let __units5 = u_string::units(&text);
    let __count5 = u_string::unit_count(&text);
        let mut start = 0u32;
        let mut end = __count5;
        while (i32::from_ne_bytes(((start) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) && JsCoerce::js_coerce_is_js_whitespace(*(u_string::unit_at_from(&__units5, start)).as_ref().unwrap()) {
            start = u32::wrapping_add(start, 1);
        }
        while (i32::from_ne_bytes(((end) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((start) as i32).to_ne_bytes())) && JsCoerce::js_coerce_is_js_whitespace(*(u_string::unit_at_from(&__units5, u32::wrapping_sub(end, 1))).as_ref().unwrap()) {
            end = u32::wrapping_sub(end, 1);
        }
        return if start == 0 && end == __count5 { text.to_ustring() } else { u_string::substr(&text, i32::from_ne_bytes(((start) as i32).to_ne_bytes()), Some(i32::from_ne_bytes(((u32::wrapping_sub(end, start)) as i32).to_ne_bytes()))).to_ustring() };
    }

    pub(crate) fn js_coerce_is_js_whitespace(code: u32) -> bool {
        return code == 32 || code == 9 || code == 10 || code == 13 || code == 12 || code == 11 || code == 160;
    }

    pub fn js_coerce_render_json(value: WireValue) -> UString {
        return match value {
            WireValue::WNull => UString::from("null").to_ustring().to_ustring(),
            WireValue::WBool { value: _p0 } => if _p0 { UString::from("true") } else { UString::from("false") },
            WireValue::WNum { value: _p0 } => JsCoerce::js_coerce_number_to_string(_p0),
            WireValue::WStr { value: _p0 } => JsCoerce::js_coerce_quote_json(_p0.as_ustr()),
            WireValue::WArr { items: _p0 } => JsCoerce::js_coerce_render_array(&_p0),
            WireValue::WObj { fields: _p0 } => JsCoerce::js_coerce_render_object(&_p0),
        };
    }

    pub(crate) fn js_coerce_render_array(items: &Vec<WireValue>) -> UString {
        let mut parts: Vec<UString> = Vec::new();
        let mut item_index_idx = 0u32;
        while (i32::from_ne_bytes(((item_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((items.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let item = (items[usize::try_from(item_index_idx).unwrap_or(0)]).clone();
            parts.push(JsCoerce::js_coerce_render_json((item).clone()));
            item_index_idx = u32::wrapping_add(item_index_idx, 1);
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub(crate) fn js_coerce_render_object(fields: &Vec<WireField>) -> UString {
        let mut parts: Vec<UString> = Vec::new();
        let mut field_index_idx = 0u32;
        while (i32::from_ne_bytes(((field_index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((fields.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let field = (fields[usize::try_from(field_index_idx).unwrap_or(0)]).clone();
            parts.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += JsCoerce::js_coerce_quote_json((field.name).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from(":")); __s += JsCoerce::js_coerce_render_json((field.value).clone()).as_ustr(); __s }).as_str()));
            field_index_idx = u32::wrapping_add(field_index_idx, 1);
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("{")); __s += UString::from(format!("{}", { let joined2 = parts; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("}")); __s }).as_str());
    }

    pub(crate) fn js_coerce_quote_json(text: &UStr) -> UString {
    let __units6 = u_string::units(&text);
    let __count6 = u_string::unit_count(&text);
        let mut out_b = UString::new();
        out_b += &(UString::from("\""));
        let mut index_idx = 0u32;
        let __units7 = u_string::units(&text);
        let __count7 = u_string::unit_count(&text);
        while (i32::from_ne_bytes(((index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count6) as i32).to_ne_bytes())) {
            let code = u_string::unit_at_from(&__units7, index_idx).unwrap_or(0);
            if code == 34 {
                out_b += &(UString::from("\\\""));
            } else {
                if code == 92 {
                    out_b += &(UString::from("\\\\"));
                } else {
                    if code == 8 {
                        out_b += &(UString::from("\\b"));
                    } else {
                        if code == 12 {
                            out_b += &(UString::from("\\f"));
                        } else {
                            if code == 10 {
                                out_b += &(UString::from("\\n"));
                            } else {
                                if code == 13 {
                                    out_b += &(UString::from("\\r"));
                                } else {
                                    if code == 9 {
                                        out_b += &(UString::from("\\t"));
                                    } else {
                                        if i32::from_ne_bytes(((code) as i32).to_ne_bytes()) < (32) {
                                            out_b += &(UString::from("\\u00"));
                                            {
                                                let x = u_string::substring(&UString::from("0123456789abcdef"), i32::from_ne_bytes(((code >> 4 & 15) as i32).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes(((code >> 4 & 15) as i32).to_ne_bytes()), 1));
                                                out_b += &(x.to_string());
                                            }
                                            {
                                                let x = u_string::substring(&UString::from("0123456789abcdef"), i32::from_ne_bytes(((code & 15) as i32).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes(((code & 15) as i32).to_ne_bytes()), 1));
                                                out_b += &(x.to_string());
                                            }
                                        } else {
                                            let c = code;
                                            out_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        out_b += &(UString::from("\""));
        return out_b;
    }
}
