use crate::org::tiqian::protocol::plan_json_number::PlanJsonNumber;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;


#[derive(Clone, Copy)]
pub struct ReplayKey;

impl ReplayKey {
    pub fn replay_key_shape_replay_key(display_text: &UStr, font_families: &Vec<UString>, font_weight: f64, italic: bool, locale: &UStr, role: Option<UString>, source_text: &UStr) -> ShapeReplayKey {
        return ShapeReplayKey { display_text: display_text.to_ustring(), serialized_families: ReplayKey::replay_key_serialize_families(&font_families).clone(), font_weight: font_weight, italic: italic, locale: locale.to_ustring(),
role: match &(role) { None => UString::from("null"), Some(__option) => __option.to_ustring() }.clone(), source_text: source_text.to_ustring() };
    }

    pub fn replay_key_metric_replay_key(font_families: &Vec<UString>, font_weight: f64, italic: bool, role: Option<UString>, face_selection_text: Option<UString>) -> MetricReplayKey {
        return MetricReplayKey { serialized_families: ReplayKey::replay_key_serialize_families(&font_families).clone(), font_weight: font_weight, italic: italic, role: match &(role) { None => UString::from("null"), Some(__option1) => __option1.to_ustring() }.clone(),
face_selection_text: match &(face_selection_text) { None => UString::from("null"), Some(__option2) => __option2.to_ustring() }.clone() };
    }

    pub fn replay_key_render_shape_replay_key(key: ShapeReplayKey) -> Result<UString, UStringFault> {
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !match (UString::from("[")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("[").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.display_text).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.serialized_families).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_number(&mut out, key.font_weight)?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_bool(&mut out, key.italic)?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.locale).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.role).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.source_text).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !match (UString::from("]")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?);
    }

    pub fn replay_key_render_metric_replay_key(key: MetricReplayKey) -> Result<UString, UStringFault> {
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                if !match (UString::from("[")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("[").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.serialized_families).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_number(&mut out, key.font_weight)?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_bool(&mut out, key.italic)?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.role).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                if !match (UString::from(",")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",").encode_utf16());
        let _ = ReplayKey::replay_key_add_json_string(&mut out, (key.face_selection_text).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !match (UString::from("]")).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("]").encode_utf16());
        return Ok(UString::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?);
    }

    pub fn replay_key_normalized_replay_number(value: f64, font_size: f64) -> Result<Option<f64>, UStringFault> {
        if !(value).is_finite() {
            return Ok(None);
        }
        let q = value / font_size;
        if !(q).is_finite() {
            return Ok(Some(q));
        }
        let normalized = ReplayKey::replay_key_round_twelve_decimals(q)?;
        return Ok(if normalized == 0.0f64 { Some(0.0f64) } else { Some(normalized) });
    }

    pub(crate) fn replay_key_serialize_families(font_families: &Vec<UString>) -> UString {
        return UString::from(format!("{}", { let joined = font_families; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(""); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str());
    }

    pub(crate) fn replay_key_add_json_number(out: &mut Vec<u16>, v: f64) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(v)?.is_empty() {
                if !match (PlanJsonNumber::plan_json_number_ecma_json_number(v)?).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(v)?.encode_utf16());
        Ok(())
    }

    pub(crate) fn replay_key_add_json_bool(out: &mut Vec<u16>, b: bool) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !if b { UString::from("true") } else { UString::from("false") }.is_empty() {
                if !match (if b { UString::from("true") } else { UString::from("false") }).encode_utf16().next() { Some(head) => head >= 56320 && head <= 57343, None => false } {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(if b { UString::from("true") } else { UString::from("false") }.encode_utf16());
        Ok(())
    }

    pub(crate) fn replay_key_add_json_string(out: &mut Vec<u16>, s: &UStr) -> Result<(), UStringFault> {
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, s)?;
        Ok(())
    }

    pub(crate) fn replay_key_round_twelve_decimals(x: f64) -> Result<f64, UStringFault> {
        if x == 0.0f64 {
            return Ok(0.0f64);
        }
        let negative = x < (0 as f64);
        let a = if negative { -x } else { x };
        let text = ReplayKey::replay_key_fixed12(a)?;
        let parsed = u_string::parse_f64(&(text));
        let result = if negative { -parsed } else { parsed };
        return Ok(if result == 0.0f64 { 0.0f64 } else { result });
    }

    pub(crate) fn replay_key_fixed12(a: f64) -> Result<UString, UStringFault> {
        let s = PlanJsonNumber::plan_json_number_ecma_json_number(a)?;
        let int_part: UString;
        let frac_part: UString;
        let e_pos = u_string::find_from(&(s), UString::from("e").as_ustr(), 0);
        if e_pos >= 0 {
            let mant = u_string::substr(&s, 0i32, Some(i32::from_ne_bytes(((e_pos) as i32).to_ne_bytes())));
            let exp_str = u_string::substr(&s, i32::from_ne_bytes(((i32::wrapping_add(e_pos, 1)) as i32).to_ne_bytes()), None);
            let exp_first = u_string::unit_at(&exp_str, 0u32).unwrap_or(0);
            let negative_exp = exp_first == 45;
            let exp_body = if exp_first == 45 || exp_first == 43 { u_string::substr(&exp_str, 1i32, None).to_ustring() } else { exp_str.to_ustring() };
            let exp_mag = ReplayKey::replay_key_parse_unsigned_decimal(exp_body.as_ustr());
            let mdot = u_string::find_from(&(mant), UString::from(".").as_ustr(), 0);
            let digits: UString;
            let lead: u32;
            if mdot >= 0 {
                digits = UString::from(format!("{}", { let mut __s = UString::new(); __s += u_string::substr(&mant, 0i32, Some(i32::from_ne_bytes(((mdot) as i32).to_ne_bytes()))).as_ustr(); __s += u_string::substr(&mant, i32::from_ne_bytes(((i32::wrapping_add(mdot, 1)) as i32).to_ne_bytes()), None).as_ustr(); __s }).as_str());
                lead = u32::from_ne_bytes(((mdot) as u32).to_ne_bytes());
            } else {
                lead = u_string::unit_count(&(mant));
                digits = mant;
            }
            if negative_exp {
                int_part = UString::from("0").to_ustring();
                frac_part = UString::from(format!("{}", { let mut __s = UString::new(); __s += ReplayKey::replay_key_zeros(u32::wrapping_sub(exp_mag, lead)).as_ustr(); __s += digits.as_ustr(); __s }).as_str());
            } else {
                let point_pos = u32::wrapping_add(lead, exp_mag);
                let digits_len = u_string::unit_count(&(digits));
                if i32::from_ne_bytes(((point_pos) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((digits_len) as i32).to_ne_bytes()) {
                    int_part = UString::from(format!("{}", { let mut __s = UString::new(); __s += digits.as_ustr(); __s += ReplayKey::replay_key_zeros(u32::wrapping_sub(point_pos, digits_len)).as_ustr(); __s }).as_str());
                    frac_part = UString::from("").to_ustring();
                } else {
                    int_part = u_string::substr(&digits, 0i32, Some(i32::from_ne_bytes(((point_pos) as i32).to_ne_bytes())));
                    frac_part = u_string::substr(&digits, i32::from_ne_bytes(((point_pos) as i32).to_ne_bytes()), None);
                }
            }
        } else {
            let dot = u_string::find_from(&(s), UString::from(".").as_ustr(), 0);
            if dot >= 0 {
                int_part = u_string::substr(&s, 0i32, Some(i32::from_ne_bytes(((dot) as i32).to_ne_bytes())));
                frac_part = u_string::substr(&s, i32::from_ne_bytes(((i32::wrapping_add(dot, 1)) as i32).to_ne_bytes()), None);
            } else {
                int_part = s;
                frac_part = UString::from("").to_ustring();
            }
        }
        let rounded_frac: UString;
        let mut carry = false;
        if i32::from_ne_bytes(((u_string::unit_count(&(frac_part))) as i32).to_ne_bytes()) <= 12 {
            rounded_frac = UString::from(format!("{}", { let mut __s = UString::new(); __s += frac_part.as_ustr(); __s += ReplayKey::replay_key_zeros(u32::wrapping_sub(12, u_string::unit_count(&(frac_part)))).as_ustr(); __s }).as_str());
        } else {
            let next = u32::wrapping_sub(u_string::unit_at(&frac_part, 12u32).unwrap_or(0), 48);
            if i32::from_ne_bytes(((next) as i32).to_ne_bytes()) >= 5 {
                let inc = ReplayKey::replay_key_increment_decimal(u_string::substr(&frac_part, 0i32, Some(12i32)).as_ustr());
                if i32::from_ne_bytes(((u_string::unit_count(&(inc))) as i32).to_ne_bytes()) > (12) {
                    rounded_frac = ReplayKey::replay_key_zeros(12);
                    carry = true;
                } else {
                    rounded_frac = inc;
                }
            } else {
                rounded_frac = u_string::substr(&frac_part, 0i32, Some(12i32));
            }
        }
        let final_int = if carry { ReplayKey::replay_key_increment_decimal(int_part.as_ustr()).to_ustring() } else { int_part.to_ustring() };
        return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += final_int.as_ustr(); __s += &(UString::from(".")); __s += rounded_frac.as_ustr(); __s }).as_str()));
    }

    pub(crate) fn replay_key_parse_unsigned_decimal(s: &UStr) -> u32 {
    let __units = u_string::units(&s);
    let __count = u_string::unit_count(&s);
        let mut n = 0u32;
        let len = __count;
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((len) as i32).to_ne_bytes())) {
            n = u32::wrapping_add(u32::wrapping_mul(n, 10), u32::wrapping_sub(u_string::unit_at_from(&__units, i).unwrap_or(0), 48));
            i = u32::wrapping_add(i, 1);
        }
        return n;
    }

    pub(crate) fn replay_key_increment_decimal(s: &UStr) -> UString {
    let __units1 = u_string::units(&s);
    let __count1 = u_string::unit_count(&s);
        let mut i = __count1;
        while (i) > (0) && matches!(u_string::unit_at_from(&__units1, u32::wrapping_sub(i, 1)).as_ref(), Some(__v) if __v == &(57)) {
            i = u32::wrapping_sub(i, 1);
        }
        if u32::wrapping_sub(i, 1) > 2147483647 {
            return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("1")); __s += ReplayKey::replay_key_zeros(__count1).as_ustr(); __s }).as_str());
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += u_string::substr(&s, 0i32, Some(i32::from_ne_bytes(((u32::wrapping_sub(i, 1)) as i32).to_ne_bytes()))).as_ustr(); __s += (if u32::wrapping_add(u_string::unit_at_from(&__units1, u32::wrapping_sub(i, 1)).unwrap_or(0), 1) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_add(u_string::unit_at_from(&__units1, u32::wrapping_sub(i, 1)).unwrap_or(0), 1)) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u_string::unit_at_from(&__units1, u32::wrapping_sub(i, 1)).unwrap_or(0), 1)) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_add(u_string::unit_at_from(&__units1, u32::wrapping_sub(i, 1)).unwrap_or(0), 1)) as u16]) }).as_ustr(); __s += ReplayKey::replay_key_zeros(u32::wrapping_sub(u32::wrapping_sub(__count1, u32::wrapping_sub(i, 1)), 1)).as_ustr(); __s }).as_str());
    }

    pub(crate) fn replay_key_zeros(n: u32) -> UString {
        let mut s = UString::new();
        for _ in 0..n {
            s += &(UString::from("0"));
        }
        return s;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeReplayKey {
    pub display_text: UString,
    pub serialized_families: UString,
    pub font_weight: f64,
    pub italic: bool,
    pub locale: UString,
    pub role: UString,
    pub source_text: UString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricReplayKey {
    pub serialized_families: UString,
    pub font_weight: f64,
    pub italic: bool,
    pub role: UString,
    pub face_selection_text: UString,
}
