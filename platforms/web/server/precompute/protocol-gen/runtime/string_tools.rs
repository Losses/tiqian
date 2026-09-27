use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct StringTools;

impl StringTools {
    pub fn string_tools_is_space(s: &str, pos: i32) -> bool {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        if i32::from_ne_bytes((__count).to_ne_bytes()) == 0 || (pos) < (0) || (pos) >= i32::from_ne_bytes((__count).to_ne_bytes()) {
            return false;
        }
        let c = u_string::unit_at_from(&__units, u32::from_ne_bytes((pos).to_ne_bytes())).unwrap_or(0);
        return (c) > (8) && (c) < (14) || c == 32;
    }

    pub fn string_tools_ltrim(s: &str) -> String {
        let l = i32::from_ne_bytes((u_string::unit_count(&(s))).to_ne_bytes());
        let mut r = 0i32;
        while (r) < (l) && StringTools::string_tools_is_space(s, r) {
            r = i32::wrapping_add(r, 1);
        }
        return if r > (0) { u_string::substr(&s, r, Some(i32::wrapping_sub(l, r))).to_string() } else { s.to_string() };
    }

    pub fn string_tools_rtrim(s: &str) -> String {
        let l = i32::from_ne_bytes((u_string::unit_count(&(s))).to_ne_bytes());
        let mut r = 0i32;
        while (r) < (l) && StringTools::string_tools_is_space(s, i32::wrapping_sub(i32::wrapping_sub(l, r), 1)) {
            r = i32::wrapping_add(r, 1);
        }
        return if r > (0) { u_string::substr(&s, 0i32, Some(i32::wrapping_sub(l, r))).to_string() } else { s.to_string() };
    }

    pub fn string_tools_lpad(s: &str, c: &str, l: i32) -> String {
        if i32::from_ne_bytes((u_string::unit_count(&(c))).to_ne_bytes()) <= 0 {
            return s.to_string();
        }
        let mut buf_b = String::new();
        let remaining = i32::wrapping_sub(l, i32::from_ne_bytes((u_string::unit_count(&(s))).to_ne_bytes()));
        while (i32::from_ne_bytes((u_string::unit_count(&(buf_b))).to_ne_bytes())) < (remaining) {
            buf_b += &(c.to_string());
        }
        buf_b += &(s.to_string());
        return buf_b;
    }

    pub fn string_tools_rpad(s: &str, c: &str, l: i32) -> String {
        if i32::from_ne_bytes((u_string::unit_count(&(c))).to_ne_bytes()) <= 0 {
            return s.to_string();
        }
        let mut buf_b = String::new();
        buf_b += &(s.to_string());
        let remaining = i32::wrapping_sub(l, i32::from_ne_bytes((u_string::unit_count(&(s))).to_ne_bytes()));
        while (i32::from_ne_bytes((u_string::unit_count(&(buf_b))).to_ne_bytes())) < (remaining) {
            buf_b += &(c.to_string());
        }
        return buf_b;
    }

    pub fn string_tools_replace(s: &str, sub: &str, by: &str) -> String {
        return { let joined = u_string::split(&s, &sub); let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(by)); } let _ = write!(out, "{}", joined[index]); index += 1; } out };
    }
}
