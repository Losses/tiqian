use crate::runtime::u_string;


#[derive(Clone, Copy)]
pub struct TestCore;

impl TestCore {
    pub fn test_core_ok(condition: bool, message: &str) {
        if !condition {
            panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, &"", &"", false));
        }
    }

    pub fn test_core_fail(message: &str) {
        panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, &"", &"", false));
    }

    pub fn test_core_equals_bool(expected: bool, actual: bool, message: &str) {
        if expected != actual {
            panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, TestCore::test_core_format_bool(expected).as_str(), TestCore::test_core_format_bool(actual).as_str(), true));
        }
    }

    pub fn test_core_equals_int(expected: i32, actual: i32, message: &str) {
        if expected != actual {
            panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, TestCore::test_core_format_int(expected).as_str(), TestCore::test_core_format_int(actual).as_str(), true));
        }
    }

    pub fn test_core_equals_float(expected: f64, actual: f64, message: &str) {
        if expected != actual {
            panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, TestCore::test_core_format_float(expected).as_str(), TestCore::test_core_format_float(actual).as_str(), true));
        }
    }

    pub fn test_core_equals_string(expected: &str, actual: &str, message: &str) {
        if expected != actual {
            panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, TestCore::test_core_format_string(expected).as_str(), TestCore::test_core_format_string(actual).as_str(), true));
        }
    }

    pub fn test_core_report_failure(message: &str, expected_str: &str, actual_str: &str) {
        panic!("{}", TestCore::test_core_format_canonical_message(crate::runtime::test::current_test_id().as_str(), message, expected_str, actual_str, true));
    }

    pub fn test_core_format_bool(v: bool) -> String {
        if v {
            return "true".to_string();
        }
        return "false".to_string();
    }

    pub fn test_core_format_int(v: i32) -> String {
        return (v).to_string();
    }

    pub fn test_core_format_float(v: f64) -> String {
        if v != v {
            return "NaN".to_string();
        }
        if v == f64::INFINITY {
            return "Infinity".to_string();
        }
        if v == f64::NEG_INFINITY {
            return "-Infinity".to_string();
        }
        if v == 0.0f64 {
            return "0".to_string();
        }
        let raw = (v).to_string();
        let mut s = (raw).clone();
        let mut negative = false;
        if u_string::unit_at(&s, 0u32).as_ref().map_or(false, |v| v == &(45)) {
            negative = true;
            s = u_string::substring_from(&s, 1i32);
        }
        let exponent_parts = u_string::split(&s, &"e");
        let mut exponent = 0i32;
        if i32::from_ne_bytes(u32::try_from(((exponent_parts).len()) & 4294967295).unwrap_or(0).to_ne_bytes()) == 2 {
            let exponent_text = (exponent_parts[1usize]).clone();
            let exponent_value = u_string::parse_i32(&(exponent_text));
            exponent = match &(exponent_value) { None => 0, Some(__option) => *__option };
            s = (exponent_parts[0usize]).clone();
        }
        let decimal_parts = u_string::split(&s, &".");
        let has_dot = i32::from_ne_bytes(u32::try_from(((decimal_parts).len()) & 4294967295).unwrap_or(0).to_ne_bytes()) == 2;
        let mut fraction = String::new();
        if has_dot {
            fraction = (decimal_parts[1usize]).clone();
        }
        let mut digits = format!("{}{}",
            (decimal_parts[0usize]).clone(),
            fraction
        );
        let mut decimal_position = i32::wrapping_add(i32::from_ne_bytes((u_string::unit_count(&((decimal_parts[0usize]).clone()))).to_ne_bytes()), exponent);
        while (i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes())) > (1) && u_string::unit_at(&digits, 0u32).as_ref().map_or(false, |v| v == &(48)) {
            digits = u_string::substring_from(&digits, 1i32);
            decimal_position = i32::wrapping_sub(decimal_position, 1);
        }
        if digits == "0" {
            return "0".to_string();
        }
        if decimal_position >= -5 && (decimal_position) <= 21 {
            let mut plain = if decimal_position <= 0 { TestCore::test_core_plain_leading(digits.as_str(), decimal_position).to_string() } else { if decimal_position >= i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes()) {
TestCore::test_core_plain_trailing(digits.as_str(), decimal_position).to_string() } else { format!("{}{}{}",
            u_string::substring(&digits, 0i32, decimal_position),
            ".",
            u_string::substring_from(&digits, decimal_position)
        ).to_string() }.to_string() };
            while (i32::from_ne_bytes((u_string::unit_count(&(plain))).to_ne_bytes())) > (0) && u_string::unit_at(&plain, u32::from_ne_bytes((i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(plain))).to_ne_bytes()), 1)).to_ne_bytes())).as_ref().map_or(false, |v| v ==
&(48)) && (i32::from_ne_bytes(u32::try_from(((u_string::split(&plain, &".")).len()) & 4294967295).unwrap_or(0).to_ne_bytes())) > (1) {
                plain = u_string::substring(&plain, 0i32, i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(plain))).to_ne_bytes()), 1));
            }
            if i32::from_ne_bytes((u_string::unit_count(&(plain))).to_ne_bytes()) > (0) && u_string::unit_at(&plain, u32::from_ne_bytes((i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(plain))).to_ne_bytes()), 1)).to_ne_bytes())).as_ref().map_or(false, |v| v ==
&(46)) {
                plain = u_string::substring(&plain, 0i32, i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(plain))).to_ne_bytes()), 1));
            }
            return format!("{}{}",
            (if negative { "-".to_string() } else { "".to_string() }),
            plain
        );
        }
        while (i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes())) > (1) && u_string::unit_at(&digits, u32::from_ne_bytes((i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes()), 1)).to_ne_bytes())).as_ref().map_or(false, |v| v ==
&(48)) {
            digits = u_string::substring(&digits, 0i32, i32::wrapping_sub(i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes()), 1));
        }
        let sci_exponent = i32::wrapping_sub(decimal_position, 1);
        let mantissa = if i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes()) == 1 { digits.to_string() } else { format!("{}{}{}",
            u_string::substring(&digits, 0i32, 1i32),
            ".",
            u_string::substring_from(&digits, 1i32)
        ).to_string() };
        return format!("{}{}{}{}{}",
            (if negative { "-".to_string() } else { "".to_string() }),
            mantissa,
            "e",
            (if sci_exponent >= 0 { "+".to_string() } else { "".to_string() }),
            sci_exponent
        );
    }

    pub(crate) fn test_core_plain_leading(digits: &str, decimal_position: i32) -> String {
        let mut head = "0.".to_string();
        for _ in 0..-decimal_position {
            head += &("0");
        }
        return format!("{}{}",
            head,
            digits
        );
    }

    pub(crate) fn test_core_plain_trailing(digits: &str, decimal_position: i32) -> String {
        let mut tail = String::new();
        for _ in 0..i32::wrapping_sub(decimal_position, i32::from_ne_bytes((u_string::unit_count(&(digits))).to_ne_bytes())) {
            tail += &("0");
        }
        return format!("{}{}",
            digits,
            tail
        );
    }

    pub fn test_core_format_string(v: &str) -> String {
        return format!("{}{}{}",
            "\"",
            TestCore::test_core_escape_json(v),
            "\""
        );
    }

    pub fn test_core_format_bytes(b: &[u8]) -> String {
        let mut out = String::new();
        for index in 0..match u32::try_from(b.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let value = i32::from(b[usize::try_from(index).unwrap_or(0)]);
            out += &(TestCore::test_core_hex_digit(value >> 4 & 15));
            out += &(TestCore::test_core_hex_digit(value & 15));
        }
        return out;
    }

    pub fn test_core_escape_json(s: &str) -> String {
        let mut out = String::new();
        let mut cursor = 0i32;
        let stop = i32::from_ne_bytes(u32::try_from(((s).len()) & 4294967295).unwrap_or(0).to_ne_bytes());
        while (cursor) < (stop) {
            let code = i32::try_from(u32::from((s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0'))).unwrap_or(0);
            if code == 34 {
                out += &("\\\"");
            } else {
                if code == 92 {
                    out += &("\\\\");
                } else {
                    if code == 10 {
                        out += &("\\n");
                    } else {
                        if code == 13 {
                            out += &("\\r");
                        } else {
                            if code == 9 {
                                out += &("\\t");
                            } else {
                                if code < (32) {
                                    out += &(format!("{}{}{}{}{}",
            "\\u",
            TestCore::test_core_hex_digit(code >> 12 & 15),
            TestCore::test_core_hex_digit(code >> 8 & 15),
            TestCore::test_core_hex_digit(code >> 4 & 15),
            TestCore::test_core_hex_digit(code & 15)
        ));
                                } else {
                                    out += &((s)[usize::try_from(cursor).unwrap_or(0)..usize::try_from(i32::from_ne_bytes(u32::try_from((usize::try_from(cursor).unwrap_or(0) + (s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0').len_utf8()) &
4294967295).unwrap_or(0).to_ne_bytes())).unwrap_or(0)].to_string());
                                }
                            }
                        }
                    }
                }
            }
            cursor = i32::from_ne_bytes(u32::try_from((usize::try_from(cursor).unwrap_or(0) + (s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0').len_utf8()) & 4294967295).unwrap_or(0).to_ne_bytes());
        }
        return out;
    }

    pub fn test_core_format_canonical_message(id: &str, message: &str, expected_str: &str, actual_str: &str, is_equals: bool) -> String {
        let mut out = format!("{}{}",
            "test failed: ",
            id
        );
        if message != "" {
            out += &(format!("{}{}",
            concat!("\n",
"  message: "),
            message
        ));
        }
        if is_equals {
            out += &(format!("{}{}",
            concat!("\n",
"  expected: "),
            expected_str
        ));
            out += &(format!("{}{}",
            concat!("\n",
"  actual:   "),
            actual_str
        ));
        }
        return out;
    }

    pub fn test_core_result_line(id: &str, name: &str, failed: bool, message: &str) -> String {
        if failed {
            return format!("{}{}{}{}{}{}{}",
            "{\"id\":\"",
            TestCore::test_core_escape_json(id),
            "\",\"name\":\"",
            TestCore::test_core_escape_json(name),
            "\",\"verdict\":\"fail\",\"message\":\"",
            TestCore::test_core_escape_json(message),
            concat!("\"}\n",
"")
        );
        }
        return format!("{}{}{}{}{}",
            "{\"id\":\"",
            TestCore::test_core_escape_json(id),
            "\",\"name\":\"",
            TestCore::test_core_escape_json(name),
            concat!("\",\"verdict\":\"pass\"}\n",
"")
        );
    }

    pub fn test_core_not_applicable_line(id: &str, name: &str) -> String {
        return format!("{}{}{}{}{}",
            "{\"id\":\"",
            TestCore::test_core_escape_json(id),
            "\",\"name\":\"",
            TestCore::test_core_escape_json(name),
            concat!("\",\"verdict\":\"not_applicable\"}\n",
"")
        );
    }

    pub(crate) fn test_core_hex_digit(nibble: i32) -> String {
        if nibble < (10) {
            return char::from_u32(u32::from_ne_bytes((i32::wrapping_add(48, nibble)).to_ne_bytes())).unwrap_or('\0').to_string();
        }
        return char::from_u32(u32::from_ne_bytes((i32::wrapping_add(87, nibble)).to_ne_bytes())).unwrap_or('\0').to_string();
    }
}
