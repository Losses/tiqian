#[derive(Clone, Copy)]
pub struct UString;

impl UString {
    pub fn u_string_count(s: &str) -> i32 {
        let mut total = 0i32;
        let mut cursor = 0i32;
        let stop = i32::from_ne_bytes(u32::try_from(((s).len()) & 4294967295).unwrap_or(0).to_ne_bytes());
        while (cursor) < (stop) {
            total = i32::wrapping_add(total, 1);
            cursor = i32::from_ne_bytes(u32::try_from((usize::try_from(cursor).unwrap_or(0) + (s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0').len_utf8()) & 4294967295).unwrap_or(0).to_ne_bytes());
        }
        return total;
    }

    pub fn u_string_at(s: &str, index: i32) -> Option<i32> {
        if index < (0) {
            return None;
        }
        let mut remaining = index;
        let mut cursor = 0i32;
        let stop = i32::from_ne_bytes(u32::try_from(((s).len()) & 4294967295).unwrap_or(0).to_ne_bytes());
        while (cursor) < (stop) {
            if remaining == 0 {
                return Some(i32::try_from(u32::from((s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0'))).unwrap_or(0));
            }
            remaining = i32::wrapping_sub(remaining, 1);
            cursor = i32::from_ne_bytes(u32::try_from((usize::try_from(cursor).unwrap_or(0) + (s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0').len_utf8()) & 4294967295).unwrap_or(0).to_ne_bytes());
        }
        return None;
    }

    pub fn u_string_slice(s: &str, from: i32, to: i32) -> String {
        let total = UString::u_string_count(s);
        let mut start = if from < (0) { 0 } else { from };
        if start > (total) {
            start = total;
        }
        let mut stop = if to > (total) { total } else { to };
        if stop < (0) {
            stop = 0i32;
        }
        if start >= stop {
            return String::new();
        }
        let mut ordinal = 0i32;
        let mut start_cursor = 0i32;
        let mut cursor = 0i32;
        while (ordinal) < (stop) {
            if ordinal == start {
                start_cursor = cursor;
            }
            cursor = i32::from_ne_bytes(u32::try_from((usize::try_from(cursor).unwrap_or(0) + (s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0').len_utf8()) & 4294967295).unwrap_or(0).to_ne_bytes());
            ordinal = i32::wrapping_add(ordinal, 1);
        }
        return (s)[usize::try_from(start_cursor).unwrap_or(0)..usize::try_from(cursor).unwrap_or(0)].to_string();
    }

    pub fn u_string_to_code_points(s: &str) -> Vec<i32> {
        let mut out: Vec<i32> = vec![];
        let mut cursor = 0i32;
        let stop = i32::from_ne_bytes(u32::try_from(((s).len()) & 4294967295).unwrap_or(0).to_ne_bytes());
        while (cursor) < (stop) {
            out.push(i32::try_from(u32::from((s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0'))).unwrap_or(0));
            cursor = i32::from_ne_bytes(u32::try_from((usize::try_from(cursor).unwrap_or(0) + (s)[usize::try_from(cursor).unwrap_or(0)..].chars().next().unwrap_or('\0').len_utf8()) & 4294967295).unwrap_or(0).to_ne_bytes());
        }
        return out;
    }

    pub fn u_string_from_code_point(code: i32) -> String {
        return char::from_u32(u32::from_ne_bytes((code).to_ne_bytes())).unwrap_or('\0').to_string();
    }

    pub fn u_string_from_code_points(codes: &Vec<i32>) -> String {
        let mut out = String::new();
        for index in 0..match u32::try_from(codes.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            out += &(char::from_u32(u32::from_ne_bytes((codes[usize::try_from(index).unwrap_or(0)]).to_ne_bytes())).unwrap_or('\0').to_string());
        }
        return out;
    }
}
// Business ABI adapters over the resident UString class: Int arguments
// arrive unsigned (u32) and results return u32; the class works in i32.
// slice and substring keep i32 bounds because negative bounds are part
// of their clamping contract. The class lives in this same module, so
// the adapters name it directly without an import.
pub fn count(s: &str) -> u32 {
    u32::try_from(UString::u_string_count(s)).unwrap_or(0)
}

// The String.length member counts UTF-16 code units on every target
// (stdlib/15). The storage here is UTF-8, so a walk over chars() counts
// scalar values and gives one count for a surrogate pair; encode_utf16
// yields the units the member reports.
pub fn unit_count(s: &str) -> u32 {
    u32::try_from(s.encode_utf16().count()).unwrap_or(0)
}

// The UTF-16 code-unit vector of a string, built once. Per-character
// loops that read length and per-index units lower against this vector
// instead of rescanning the UTF-8 source on every access, which turns a
// per-character scan into a quadratic blow-up on large blocks.
pub fn units(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

// The single UTF-16 unit at `index` read from a precomputed unit vector,
// the O(1) form of String.charCodeAt. It answers None past the last unit,
// matching the unit_at read it replaces, so the call-site unwrap (or the
// nullable Option context) rides the same machinery unchanged.
pub fn unit_at_from(units: &[u16], index: u32) -> Option<u32> {
    units.get(usize::try_from(index).unwrap_or(0)).map(|u| u32::from(*u))
}

// The single UTF-16 unit at `index` as an owned one-unit String, the
// O(1) form of String.charAt; an out-of-range index yields the empty
// string, matching charAt past the end.
pub fn char_at_from(units: &[u16], index: u32) -> String {
    let i = usize::try_from(index).unwrap_or(0);
    if i < units.len() {
        String::from_utf16_lossy(&units[i..i + 1])
    } else {
        String::new()
    }
}

// The code-point read that std.UString.at lowers to: the index counts
// characters and the value is one code point, so a surrogate pair
// occupies one address and yields its combined code point.
pub fn at(s: &str, index: u32) -> Option<u32> {
    let mut remaining = index;
    for c in s.chars() {
        if remaining == 0 {
            return Some(u32::from(c));
        }
        remaining -= 1;
    }
    None
}

// The unit read that String.charCodeAt lowers to (stdlib spec 15): the
// index counts UTF-16 code units, so each half of a surrogate pair
// carries its own address and the value is the unit, never the combined
// code point. An index past the last unit answers None, never a panic.
pub fn unit_at(s: &str, index: u32) -> Option<u32> {
    let mut remaining = index;
    for unit in s.encode_utf16() {
        if remaining == 0 {
            return Some(u32::from(unit));
        }
        remaining -= 1;
    }
    None
}

pub fn split(s: &str, separator: &str) -> Vec<String> {
    let source: Vec<u16> = s.encode_utf16().collect();
    let needle: Vec<u16> = separator.encode_utf16().collect();
    if needle.is_empty() {
        let mut out = Vec::new();
        for unit in source {
            out.push(String::from_utf16_lossy(&[unit]));
        }
        return out;
    }
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut cursor = 0usize;
    while cursor + needle.len() <= source.len() {
        if source[cursor..cursor + needle.len()] == needle[..] {
            out.push(String::from_utf16_lossy(&source[start..cursor]));
            cursor += needle.len();
            start = cursor;
        } else {
            cursor += 1;
        }
    }
    out.push(String::from_utf16_lossy(&source[start..]));
    out
}

pub fn slice(s: &str, from: i32, to: i32) -> String {
    UString::u_string_slice(s, from, to)
}

pub fn to_code_points(s: &str) -> Vec<u32> {
    let mut out = Vec::new();
    for code in UString::u_string_to_code_points(s) {
        out.push(u32::try_from(code).unwrap_or(0));
    }
    out
}

pub fn from_code_point(code: u32) -> String {
    UString::u_string_from_code_point(i32::try_from(code).unwrap_or(0))
}

pub fn from_code_points(codes: &Vec<u32>) -> String {
    let mut inner = Vec::with_capacity(codes.len());
    for index in 0..codes.len() {
        inner.push(i32::try_from(codes[index]).unwrap_or(0));
    }
    UString::u_string_from_code_points(&mut inner)
}

// substring keeps i32 bounds for the same clamping reason as slice:
// negative bounds are part of the haxe substring contract.
pub fn substring(s: &str, from: i32, to: i32) -> String {
    let mut start = if from < 0 { 0u32 } else { u32::try_from(from).unwrap_or(0) };
    let mut end = if to < 0 { 0u32 } else { u32::try_from(to).unwrap_or(0) };
    if start > end {
        let tmp = start;
        start = end;
        end = tmp;
    }
    let byte_start = unit_index(s, start, true);
    let byte_end = unit_index(s, end, false);
    s[byte_start..byte_end].to_string()
}

pub fn substring_from(s: &str, from: i32) -> String {
    let start = if from < 0 { 0u32 } else { u32::try_from(from).unwrap_or(0) };
    s[unit_index(s, start, true)..].to_string()
}

// substr keeps i32 bounds like substring, and it addresses the same
// UTF-16 unit sequence: the position and the length count units, and
// the two unit bounds convert to byte boundaries through unit_index
// before the slice. A negative pos counts from the end of the unit
// sequence per the std contract. A negative len is unspecified in the
// std (std/String.hx), so this runtime returns the empty string,
// matching the JavaScript target, and features/08 rules the shared
// domain to non-negative len values.
pub fn substr(s: &str, pos: i32, len: Option<i32>) -> String {
    match len {
        Some(l) if l < 0 => return String::new(),
        _ => {}
    }
    let units = i64::try_from(s.encode_utf16().count()).unwrap_or(0);
    let start = if pos < 0 {
        let back = i64::from(pos).saturating_neg();
        if units > back { units - back } else { 0 }
    } else {
        if i64::from(pos) > units { units } else { i64::from(pos) }
    };
    let end = match len {
        None => units,
        Some(l) => {
            let raw = start + i64::from(l);
            if raw > units { units } else { raw }
        }
    };
    let byte_start = unit_index(s, u32::try_from(start).unwrap_or(0), true);
    let byte_end = unit_index(s, u32::try_from(end).unwrap_or(0), false);
    s[byte_start..byte_end].to_string()
}

// Haxe Std.parseFloat lowers here. The token must match the full decimal
// grammar after trimming the fixed whitespace set; any partial or
// nonfinite spelling yields NaN.
pub fn parse_f64(s: &str) -> f64 {
    let t = trim_fixed(s);
    if !valid_decimal_token(t.as_bytes()) {
        return f64::NAN;
    }
    t.parse::<f64>().unwrap_or(f64::NAN)
}

// Binary32 edge of the same lowering for the float-precision=f32 lane.
pub fn parse_f32(s: &str) -> f32 {
    let t = trim_fixed(s);
    if !valid_decimal_token(t.as_bytes()) {
        return f32::NAN;
    }
    t.parse::<f32>().unwrap_or(f32::NAN)
}

// Haxe Std.parseInt lowers here: an optional sign, an optional 0x or 0X
// prefix, digits of the implied radix, and the i32 range gate; every
// other shape yields None.
pub fn parse_i32(s: &str) -> Option<i32> {
    let t = trim_fixed(s);
    let b = t.as_bytes();
    let mut i = 0;
    let negative = i < b.len() && b[i] == 0x2D;
    if i < b.len() && (b[i] == 0x2B || b[i] == 0x2D) {
        i += 1;
    }
    let hexadecimal = i + 1 < b.len() && b[i] == 0x30 && (b[i + 1] == 0x78 || b[i + 1] == 0x58);
    if hexadecimal {
        i += 2;
    }
    let start = i;
    while i < b.len() {
        let matched = if hexadecimal { b[i].is_ascii_hexdigit() } else { b[i].is_ascii_digit() };
        if !matched {
            break;
        }
        i += 1;
    }
    if i == start || i != b.len() {
        return None;
    }
    let radix = if hexadecimal { 16u32 } else { 10u32 };
    let magnitude = match u32::from_str_radix(&t[start..], radix) {
        Ok(value) => value,
        Err(_) => return None,
    };
    let signed = if negative { -i64::from(magnitude) } else { i64::from(magnitude) };
    i32::try_from(signed).ok()
}

// The whitespace set of the Haxe scanners: space plus the ASCII control
// range 9 through 13. Unicode whitespace beyond it stays in the token and
// fails validation.
fn trim_fixed(s: &str) -> &str {
    s.trim_matches(|c: char| c == ' ' || matches!(c, '\t'..='\r'))
}

fn valid_decimal_token(b: &[u8]) -> bool {
    let mut i = 0;
    if i < b.len() && (b[i] == 0x2B || b[i] == 0x2D) {
        i += 1;
    }
    let mut digits = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
        digits += 1;
    }
    if i < b.len() && b[i] == 0x2E {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
            digits += 1;
        }
    } else if digits == 0 {
        return false;
    }
    if digits == 0 {
        return false;
    }
    if i < b.len() && (b[i] == 0x65 || b[i] == 0x45) {
        i += 1;
        if i < b.len() && (b[i] == 0x2B || b[i] == 0x2D) {
            i += 1;
        }
        let mut exponent_digits = 0;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
            exponent_digits += 1;
        }
        if exponent_digits == 0 {
            return false;
        }
    }
    i == b.len()
}

// UTF-16 unit boundary to byte boundary, the index space of the haxe
// substring and substr contracts. A bound that falls inside a
// surrogate pair moves to the far side: `from` advances past the pair,
// `to` retreats before it, so a Rust slice never splits a pair; the
// subset only produces code-point-aligned bounds, where every target
// agrees.
fn unit_index(s: &str, unit: u32, round_up: bool) -> usize {
    let mut u: u32 = 0;
    for (b, c) in s.char_indices() {
        if u >= unit {
            return b;
        }
        let w = u32::try_from(c.len_utf16()).unwrap_or(0);
        if u + w > unit {
            return if round_up { b + c.len_utf8() } else { b };
        }
        u += w;
    }
    s.len()
}

// String.indexOf with a start position (stdlib spec 15): the start and
// the returned index both count UTF-16 code units, matching the tiqian
// ABI. A negative start is treated as 0 (Haxe/JS semantics); a start
// past the last unit, or one that lands mid-surrogate (rounded up to
// the next char), yields -1. The match index converts back to units so
// the caller sees the same index space as the no-start find() form.
pub fn find_from(s: &str, needle: &str, start: i32) -> i32 {
    let start_unit = if start < 0 { 0u32 } else { u32::try_from(start).unwrap_or(u32::MAX) };
    let byte_start = unit_index(s, start_unit, true);
    let rest = &s[byte_start..];
    match rest.find(needle) {
        Some(byte_rel) => {
            let unit = byte_to_unit(s, byte_start + byte_rel);
            i32::try_from(unit).unwrap_or(-1)
        }
        None => -1,
    }
}

fn byte_to_unit(s: &str, byte: usize) -> u32 {
    let mut units = 0u32;
    for (b, c) in s.char_indices() {
        if b >= byte {
            break;
        }
        units += u32::try_from(c.len_utf16()).unwrap_or(0);
    }
    units
}

