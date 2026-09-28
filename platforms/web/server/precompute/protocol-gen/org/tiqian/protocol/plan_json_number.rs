use crate::runtime::fp_helper::FPHelper;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


static PLAN_JSON_NUMBER_FIVE_POWERS_BUILDER: Mutex<Option<SortedMapTableBuilder<u32, UString>>> = Mutex::new(None);
static PLAN_JSON_NUMBER_FIVE_POWERS: Mutex<Option<SortedMapTable<u32, UString>>> = Mutex::new(None);
static PLAN_JSON_NUMBER_TWO_POWERS_BUILDER: Mutex<Option<SortedMapTableBuilder<u32, UString>>> = Mutex::new(None);
static PLAN_JSON_NUMBER_TWO_POWERS: Mutex<Option<SortedMapTable<u32, UString>>> = Mutex::new(None);

#[derive(Clone, Copy)]
pub struct PlanJsonNumber;

impl PlanJsonNumber {

    pub fn plan_json_number_ecma_json_number(float_value: f64) -> Result<UString, UStringFault> {
        if float_value.is_nan() {
            return Ok(UString::from("NaN").to_ustring());
        }
        if float_value.is_finite() == false {
            return Ok(if float_value < (0 as f64) { UString::from("-Infinity") } else { UString::from("Infinity") });
        }
        if float_value == 0 as f64 {
            return Ok(UString::from("0").to_ustring());
        }
        let negative = float_value < (0 as f64);
        let magnitude_value = if negative { -float_value } else { float_value };
        let shortest = PlanJsonNumber::plan_json_number_shortest_round_trip_digits(magnitude_value)?;
        let digits = PlanJsonNumber::plan_json_number_canonical_float_digits(magnitude_value, (shortest.digits).to_ustring().as_ustr())?;
        let k = u_string::unit_count(&(digits));
        let n = shortest.exponent;
        let sign = if negative { UString::from("-") } else { UString::from("") };
        if i32::from_ne_bytes(((k) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((n) as i32).to_ne_bytes()) && (i32::from_ne_bytes(((n) as i32).to_ne_bytes())) <= 21 {
            return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += sign.as_ustr(); __s += digits.as_ustr(); __s += PlanJsonNumber::plan_json_number_zeros(u32::wrapping_sub(n, k)).as_ustr(); __s }).as_str()));
        }
        if 0 < (i32::from_ne_bytes(((n) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((n) as i32).to_ne_bytes())) <= 21 {
            return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += sign.as_ustr(); __s += u_string::substr(&digits, 0i32, Some(i32::from_ne_bytes(((n) as i32).to_ne_bytes()))).as_ustr(); __s += &(UString::from(".")); __s += u_string::substr(&digits, i32::from_ne_bytes(((n) as i32).to_ne_bytes()), None).as_ustr(); __s }).as_str()));
        }
        if i32::from_ne_bytes(((4294967290u32) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((n) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((n) as i32).to_ne_bytes())) <= 0 {
            return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += sign.as_ustr(); __s += &(UString::from("0.")); __s += PlanJsonNumber::plan_json_number_zeros(u32::from_ne_bytes(((-(n as i32)) as u32).to_ne_bytes())).as_ustr(); __s += digits.as_ustr(); __s }).as_str()));
        }
        let mantissa = if i32::from_ne_bytes(((k) as i32).to_ne_bytes()) > (1) { UString::from(format!("{}", { let mut __s = UString::new(); __s += u_string::substr(&digits, 0i32, Some(1i32)).as_ustr(); __s += &(UString::from(".")); __s += u_string::substr(&digits, 1i32, None).as_ustr(); __s }).as_str()) } else { digits.to_ustring() };
        let ev = i32::wrapping_sub(i32::from_ne_bytes(((n) as i32).to_ne_bytes()), 1);
        let esign = if ev < (0) { UString::from("-") } else { UString::from("+") };
        let absolute_exponent = if ev < (0) { u32::from_ne_bytes(((-(ev as i32)) as u32).to_ne_bytes()) } else { u32::from_ne_bytes(((ev) as u32).to_ne_bytes()) };
        return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += sign.as_ustr(); __s += mantissa.as_ustr(); __s += &(UString::from("e")); __s += esign.as_ustr(); __s += UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(absolute_exponent)).as_str()).as_ustr(); __s }).as_str()));
    }

    pub fn plan_json_number_append_json_string(out: &mut Vec<u16>, value: &UStr) -> Result<(), UStringFault> {
    let __units = u_string::units(&value);
    let __count = u_string::unit_count(&value);
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("\"").is_empty() {
                if !UString::from("\"").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("\"").encode_utf16());
        for i in 0..match u32::try_from(u_string::unit_count(&(value))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units, i).unwrap_or(0);
            if c == 34 {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("\\\"").is_empty() {
                        if !UString::from("\\\"").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from("\\\"").encode_utf16());
            } else {
                if c == 92 {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("\\\\").is_empty() {
                            if !UString::from("\\\\").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("\\\\").encode_utf16());
                } else {
                    if c == 8 {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from("\\b").is_empty() {
                                if !UString::from("\\b").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from("\\b").encode_utf16());
                    } else {
                        if c == 12 {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from("\\f").is_empty() {
                                    if !UString::from("\\f").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from("\\f").encode_utf16());
                        } else {
                            if c == 10 {
                                if let Some(&unit) = out.last() {
                                    if unit >= 55296 && unit <= 56319 && !UString::from("\\n").is_empty() {
                                        if !UString::from("\\n").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                        }
                                    }
                                }
                                out.extend(UString::from("\\n").encode_utf16());
                            } else {
                                if c == 13 {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !UString::from("\\r").is_empty() {
                                            if !UString::from("\\r").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                            }
                                        }
                                    }
                                    out.extend(UString::from("\\r").encode_utf16());
                                } else {
                                    if c == 9 {
                                        if let Some(&unit) = out.last() {
                                            if unit >= 55296 && unit <= 56319 && !UString::from("\\t").is_empty() {
                                                if !UString::from("\\t").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                                }
                                            }
                                        }
                                        out.extend(UString::from("\\t").encode_utf16());
                                    } else {
                                        if i32::from_ne_bytes(((c) as i32).to_ne_bytes()) < (32) {
                                            if let Some(&unit) = out.last() {
                                                if unit >= 55296 && unit <= 56319 && !{ let mut __s = UString::new(); __s += &(UString::from("\\u")); __s += UString::from(format!("{}", format!("{:0w$X}", c, w = usize::try_from(4).unwrap_or_default()).to_lowercase()).as_str()).as_ustr(); __s }.is_empty() {
                                                    if !{ let mut __s = UString::new(); __s += &(UString::from("\\u")); __s += UString::from(format!("{}", format!("{:0w$X}", c, w = usize::try_from(4).unwrap_or_default()).to_lowercase()).as_str()).as_ustr(); __s }.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                                    }
                                                }
                                            }
                                            out.extend({ let mut __s = UString::new(); __s += &(UString::from("\\u")); __s += UString::from(format!("{}", format!("{:0w$X}", c, w = usize::try_from(4).unwrap_or_default()).to_lowercase()).as_str()).as_ustr(); __s }.encode_utf16());
                                        } else {
                                            if c >= 56320 && c <= 57343 {
                                                match out.last() {
                                                    Some(&last) if last >= 55296 && last <= 56319 => {}
                                                    _ => return Err(UStringFault::UnpairedSurrogate { unit: c }),
                                                }
                                            } else if let Some(&last) = out.last() {
                                                if last >= 55296 && last <= 56319 {
                                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                                                }
                                            }
                                            out.push(u16::try_from((c) & 0xFFFF).unwrap_or(0));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("\"").is_empty() {
                if !UString::from("\"").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("\"").encode_utf16());
        Ok(())
    }

    pub fn plan_json_number_shortest_round_trip_digits(magnitude: f64) -> Result<PlanJsonDigitsAndExponent, UStringFault> {
        let d = PlanJsonNumber::plan_json_number_decompose(magnitude)?;
        let f = d.exp2;
        let expansion = PlanJsonNumber::plan_json_number_dyadic_decimal((d.mantissa).to_ustring().as_ustr(), f)?;
        let exact = PlanJsonNumber::plan_json_number_trim_zeros((expansion.digits).to_ustring().as_ustr());
        let n = expansion.exponent;
        let base = (d.mantissa).to_ustring().clone();
        let doubled = PlanJsonNumber::plan_json_number_times_small(base.as_ustr(), 2)?;
        let hi = PlanJsonNumber::plan_json_number_dyadic_decimal_string(PlanJsonNumber::plan_json_number_add_decimal(doubled.as_ustr(), UStr::new(&[49]))?.as_ustr(), u32::wrapping_sub(d.exp2, 1))?;
        let lo: PlanJsonDigitsAndExponent;
        if base == UString::from("4503599627370496") {
            lo = PlanJsonNumber::plan_json_number_dyadic_decimal_string(PlanJsonNumber::plan_json_number_decrement_decimal(PlanJsonNumber::plan_json_number_times_small(base.as_ustr(), 4)?.as_ustr()).as_ustr(), u32::wrapping_sub(d.exp2, 2))?;
        } else {
            lo = PlanJsonNumber::plan_json_number_dyadic_decimal_string(PlanJsonNumber::plan_json_number_decrement_decimal(doubled.as_ustr()).as_ustr(), u32::wrapping_sub(d.exp2, 1))?;
        }
        let inclusive = u32::from_ne_bytes(((i32::from_ne_bytes(((u32::wrapping_sub(u_string::unit_at(&base, u32::wrapping_sub(u_string::unit_count(&(base)), 1)).unwrap_or(0), 48)) as i32).to_ne_bytes()) % 2i32) as u32).to_ne_bytes()) == 0;
        let mut length = 1u32;
        while (i32::from_ne_bytes(((length) as i32).to_ne_bytes())) <= 17 {
            let keep = u_string::substr(&exact, 0i32, Some(i32::from_ne_bytes(((length) as i32).to_ne_bytes())));
            let up = PlanJsonNumber::plan_json_number_increment_decimal(keep.as_ustr());
            let up_n = u32::wrapping_sub(u32::wrapping_add(n, u_string::unit_count(&(up))), length);
            if PlanJsonNumber::plan_json_number_in_interval(keep.as_ustr(), n, (lo).clone(), (hi).clone(), inclusive) || PlanJsonNumber::plan_json_number_in_interval(up.as_ustr(), up_n, (lo).clone(), (hi).clone(), inclusive) {
                let rounded = PlanJsonNumber::plan_json_number_round_to_significant(exact.as_ustr(), length);
                return Ok(PlanJsonDigitsAndExponent { digits: (rounded.digits).to_ustring().clone(), exponent: u32::wrapping_add(n, rounded.exponent) });
            }
            length = u32::wrapping_add(length, 1);
        }
        return Ok(PlanJsonDigitsAndExponent { digits: exact.clone(), exponent: n });
    }

    pub(crate) fn plan_json_number_in_interval(digits: &UStr, n: u32, lo: PlanJsonDigitsAndExponent, hi: PlanJsonDigitsAndExponent, inclusive: bool) -> bool {
        let low = PlanJsonNumber::plan_json_number_compare_decimal(digits, n, (lo.digits).to_ustring().as_ustr(), lo.exponent);
        let high = PlanJsonNumber::plan_json_number_compare_decimal(digits, n, (hi.digits).to_ustring().as_ustr(), hi.exponent);
        return ((i32::from_ne_bytes(((low) as i32).to_ne_bytes())) > (0) || low == 0 && inclusive) && ((high) > 2147483647 || high == 0 && inclusive);
    }

    pub(crate) fn plan_json_number_canonical_float_digits(magnitude: f64, double_digits: &UStr) -> Result<UString, UStringFault> {
    let __units1 = u_string::units(&double_digits);
    let __count1 = u_string::unit_count(&double_digits);
        let d = PlanJsonNumber::plan_json_number_decompose_f32(magnitude);
        let exact = if d.mant24 == 0 { UString::from("0") } else { if d.exp2 <= 2147483647 { PlanJsonNumber::plan_json_number_times_long(PlanJsonNumber::plan_json_number_two_to_the(d.exp2)?.as_ustr(), d.mant24)?.to_ustring() } else { PlanJsonNumber::plan_json_number_times_long(PlanJsonNumber::plan_json_number_five_to_the(u32::from_ne_bytes(((-(d.exp2 as i32)) as u32).to_ne_bytes()))?.as_ustr(), d.mant24)?.to_ustring() }.to_ustring() };
        let stripped = PlanJsonNumber::plan_json_number_trim_zeros(exact.as_ustr());
        if i32::from_ne_bytes(((u_string::unit_count(&(stripped))) as i32).to_ne_bytes()) <= i32::from_ne_bytes(((__count1) as i32).to_ne_bytes()) {
            return Ok(double_digits.to_ustring());
        }
        let rounded = PlanJsonNumber::plan_json_number_round_to_significant(stripped.as_ustr(), __count1);
        return Ok(if u_string::unit_count(&((rounded.digits).to_ustring())) == __count1 { (rounded.digits).to_ustring() } else { double_digits.to_ustring() });
    }

    pub(crate) fn plan_json_number_dyadic_decimal(p: &UStr, f: u32) -> Result<PlanJsonDigitsAndExponent, UStringFault> {
        return Ok(PlanJsonNumber::plan_json_number_dyadic_decimal_string(p, f)?);
    }

    pub(crate) fn plan_json_number_dyadic_decimal_string(p: &UStr, f: u32) -> Result<PlanJsonDigitsAndExponent, UStringFault> {
        let digits = if f > 2147483647 { PlanJsonNumber::plan_json_number_multiply_decimal(PlanJsonNumber::plan_json_number_five_to_the(u32::from_ne_bytes(((-(f as i32)) as u32).to_ne_bytes()))?.as_ustr(), p)?.to_ustring() } else { PlanJsonNumber::plan_json_number_multiply_decimal(PlanJsonNumber::plan_json_number_two_to_the(u32::from_ne_bytes(((f) as u32).to_ne_bytes()))?.as_ustr(), p)?.to_ustring() };
        return Ok(PlanJsonDigitsAndExponent { digits: digits.clone(), exponent: if f > 2147483647 { u32::wrapping_add(u_string::unit_count(&(digits)), f) } else { u_string::unit_count(&(digits)) } });
    }

    pub(crate) fn plan_json_number_multiply_decimal(a: &UStr, b: &UStr) -> Result<UString, UStringFault> {
    let __units2 = u_string::units(&b);
    let __count2 = u_string::unit_count(&b);
        let mut result = UString::from("0").to_ustring();
        let mut shift = 0u32;
        let mut i = __count2;
        let __units3 = u_string::units(&b);
        let __count3 = u_string::unit_count(&b);
        while (i) > (0) {
            let digit = u32::wrapping_sub(u_string::unit_at_from(&__units3, u32::wrapping_sub(i, 1)).unwrap_or(0), 48);
            if digit != 0 {
                let mut part = PlanJsonNumber::plan_json_number_times_small(a, digit)?;
                if i32::from_ne_bytes(((shift) as i32).to_ne_bytes()) > (0) {
                    part += &(PlanJsonNumber::plan_json_number_zeros(shift));
                }
                result = PlanJsonNumber::plan_json_number_add_decimal(result.as_ustr(), part.as_ustr())?;
            }
            shift = u32::wrapping_add(shift, 1);
            i = u32::wrapping_sub(i, 1);
        }
        return Ok(result);
    }

    pub(crate) fn plan_json_number_five_to_the(k: u32) -> Result<UString, UStringFault> {
        if { let __guard = PLAN_JSON_NUMBER_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.is_none() {
            *PLAN_JSON_NUMBER_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()) = Some(SortedTable::sorted_table_map_builder::<u32, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone());
            { let __rhs_value = Some(({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap().clone().build().clone()); *PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()) = __rhs_value; };
        }
        let cached = ({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().get(&(k));
        match &(cached) {
            Some(__option) => {
                return Ok((*__option).clone());
            }
            None => {
            }
        }
        let mut anchor = 0u32;
        let mut digits = UString::from("1").to_ustring();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()))) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()))) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((anchor) as i32).to_ne_bytes())) {
                anchor = ({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()));
                digits = ({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().value_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()));
            }
            i = u32::wrapping_add(i, 1);
        }
        for _ in anchor..k {
            digits = PlanJsonNumber::plan_json_number_times_small(digits.as_ustr(), 5)?;
        }
        ({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_mut().unwrap().put(&(k), &(digits).to_ustring());
        { let __rhs_value1 = Some(({ let __guard = PLAN_JSON_NUMBER_FIVE_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap().clone().build().clone()); *PLAN_JSON_NUMBER_FIVE_POWERS.lock().unwrap_or_else(|e| e.into_inner()) = __rhs_value1; };
        return Ok(digits);
    }

    pub(crate) fn plan_json_number_two_to_the(k: u32) -> Result<UString, UStringFault> {
        if { let __guard = PLAN_JSON_NUMBER_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }.is_none() {
            *PLAN_JSON_NUMBER_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()) = Some(SortedTable::sorted_table_map_builder::<u32, UString>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes())))).clone());
            { let __rhs_value2 = Some(({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap().clone().build().clone()); *PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()) = __rhs_value2; };
        }
        let cached = ({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().get(&(k));
        match &(cached) {
            Some(__option1) => {
                return Ok((*__option1).clone());
            }
            None => {
            }
        }
        let mut anchor = 0u32;
        let mut digits = UString::from("1").to_ustring();
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::from_ne_bytes(((({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().size()) as u32).to_ne_bytes())) as i32).to_ne_bytes())) {
            if i32::from_ne_bytes(((({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()))) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((k) as i32).to_ne_bytes())) && (i32::from_ne_bytes(((({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()))) as i32).to_ne_bytes())) > (i32::from_ne_bytes(((anchor) as i32).to_ne_bytes())) {
                anchor = ({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().key_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()));
                digits = ({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_ref().unwrap().value_at(i32::from_ne_bytes(((i) as i32).to_ne_bytes()));
            }
            i = u32::wrapping_add(i, 1);
        }
        for _ in anchor..k {
            digits = PlanJsonNumber::plan_json_number_times_small(digits.as_ustr(), 2)?;
        }
        ({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).as_mut().unwrap().put(&(k), &(digits).to_ustring());
        { let __rhs_value3 = Some(({ let __guard = PLAN_JSON_NUMBER_TWO_POWERS_BUILDER.lock().unwrap_or_else(|e| e.into_inner()); __guard.clone() }).unwrap().clone().build().clone()); *PLAN_JSON_NUMBER_TWO_POWERS.lock().unwrap_or_else(|e| e.into_inner()) = __rhs_value3; };
        return Ok(digits);
    }

    pub(crate) fn plan_json_number_times_long(digits: &UStr, factor: u32) -> Result<UString, UStringFault> {
        if factor == 0 {
            return Ok(UString::from("0").to_ustring());
        }
        let mut result: Option<UString> = None;
        let mut shift = 0u32;
        let mut remaining = factor;
        while (i32::from_ne_bytes(((remaining) as i32).to_ne_bytes())) > (0) {
            let chunk = u32::from_ne_bytes(((i32::from_ne_bytes(((remaining) as i32).to_ne_bytes()) % 100000000i32) as u32).to_ne_bytes());
            remaining = remaining / (100000000);
            if chunk != 0 {
                let mut part = PlanJsonNumber::plan_json_number_times_small(digits, chunk)?;
                if i32::from_ne_bytes(((shift) as i32).to_ne_bytes()) > (0) {
                    part += &(PlanJsonNumber::plan_json_number_zeros(shift));
                }
                result = Some(match &(result) { None => part.to_ustring(), Some(__option2) => PlanJsonNumber::plan_json_number_add_decimal(__option2.as_ustr(), part.as_ustr())?.to_ustring() }.clone());
            }
            shift = u32::wrapping_add(shift, 8);
        }
        return Ok(match &(result) { None => UString::from("0"), Some(__option3) => __option3.to_ustring() });
    }

    pub(crate) fn plan_json_number_add_decimal(a: &UStr, b: &UStr) -> Result<UString, UStringFault> {
    let __units5 = u_string::units(&b);
    let __units4 = u_string::units(&a);
    let __count5 = u_string::unit_count(&b);
    let __count4 = u_string::unit_count(&a);
        let mut out = Vec::<u16>::new();
        let mut i = i32::wrapping_sub(i32::from_ne_bytes(((__count4) as i32).to_ne_bytes()), 1);
        let mut j = i32::wrapping_sub(i32::from_ne_bytes(((__count5) as i32).to_ne_bytes()), 1);
        let mut carry = 0u32;
        while (i) >= 0 || (j) >= 0 || (i32::from_ne_bytes(((carry) as i32).to_ne_bytes())) > (0) {
            let sum = u32::wrapping_add(u32::wrapping_add(if i >= 0 { u32::wrapping_sub(u_string::unit_at_from(&__units4, u32::from_ne_bytes(((i) as u32).to_ne_bytes())).unwrap_or(0), 48) } else { 0 }, if j >= 0 { u32::wrapping_sub(u_string::unit_at_from(&__units5, u32::from_ne_bytes(((j) as u32).to_ne_bytes())).unwrap_or(0), 48) } else { 0 }), carry);
            if u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((sum) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) >= 56320 && u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((sum) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) <= 57343 {
                match out.last() {
                    Some(&last) if last >= 55296 && last <= 56319 => {}
                    _ => return Err(UStringFault::UnpairedSurrogate { unit: u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((sum) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) }),
                }
            } else if let Some(&last) = out.last() {
                if last >= 55296 && last <= 56319 {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                }
            }
            out.push(u16::try_from((u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((sum) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes()))) & 0xFFFF).unwrap_or(0));
            carry = sum / (10);
            i = i32::wrapping_sub(i, 1);
            j = i32::wrapping_sub(j, 1);
        }
        let text = UString::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?;
        return Ok(PlanJsonNumber::plan_json_number_reverse(text.as_ustr()));
    }

    pub(crate) fn plan_json_number_round_to_significant(exact: &UStr, length: u32) -> PlanJsonDigitsAndExponent {
        if i32::from_ne_bytes(((length) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((u_string::unit_count(&(exact))) as i32).to_ne_bytes()) {
            return PlanJsonDigitsAndExponent { digits: exact.to_ustring(), exponent: 0 };
        }
        let keep = u_string::substr(&exact, 0i32, Some(i32::from_ne_bytes(((length) as i32).to_ne_bytes())));
        let rem = u_string::substr(&exact, i32::from_ne_bytes(((length) as i32).to_ne_bytes()), None);
        let mut up = false;
        if i32::from_ne_bytes(((u_string::unit_at(&rem, 0u32).unwrap_or(0)) as i32).to_ne_bytes()) > (53) {
            up = true;
        } else {
            if u_string::unit_at(&rem, 0u32).as_ref().map_or(false, |v| v == &(53)) {
                let mut tail = 1u32;
                let __units6 = u_string::units(&rem);
                let __count6 = u_string::unit_count(&rem);
                while (i32::from_ne_bytes(((tail) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count6) as i32).to_ne_bytes())) && u_string::unit_at_from(&__units6, tail).as_ref().map_or(false, |v| v == &(48)) {
                    tail = u32::wrapping_add(tail, 1);
                }
                up = i32::from_ne_bytes(((tail) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u_string::unit_count(&(rem))) as i32).to_ne_bytes())) || u32::from_ne_bytes(((i32::from_ne_bytes(((u32::wrapping_sub(u_string::unit_at(&keep, u32::wrapping_sub(u_string::unit_count(&(keep)), 1)).unwrap_or(0), 48)) as i32).to_ne_bytes()) % 2i32) as u32).to_ne_bytes()) != 0;
            }
        }
        let rounded = if up { PlanJsonNumber::plan_json_number_increment_decimal(keep.as_ustr()).to_ustring() } else { keep.to_ustring() };
        return PlanJsonDigitsAndExponent { digits: PlanJsonNumber::plan_json_number_trim_zeros(rounded.as_ustr()).clone(), exponent: u32::wrapping_sub(u_string::unit_count(&(rounded)), length) };
    }

    pub(crate) fn plan_json_number_compare_decimal(a: &UStr, n_a: u32, b: &UStr, n_b: u32) -> u32 {
        let e_a = u32::wrapping_sub(n_a, u_string::unit_count(&(a)));
        let e_b = u32::wrapping_sub(n_b, u_string::unit_count(&(b)));
        let aa = if i32::from_ne_bytes(((e_a) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((e_b) as i32).to_ne_bytes()) { UString::from(format!("{}", { let mut __s = UString::new(); __s += a; __s += PlanJsonNumber::plan_json_number_zeros(u32::wrapping_sub(e_a, e_b)).as_ustr(); __s }).as_str()) } else { a.to_ustring() };
        let bb = if i32::from_ne_bytes(((e_b) as i32).to_ne_bytes()) >= i32::from_ne_bytes(((e_a) as i32).to_ne_bytes()) { UString::from(format!("{}", { let mut __s = UString::new(); __s += b; __s += PlanJsonNumber::plan_json_number_zeros(u32::wrapping_sub(e_b, e_a)).as_ustr(); __s }).as_str()) } else { b.to_ustring() };
        if u_string::unit_count(&(aa)) != u_string::unit_count(&(bb)) {
            return u32::wrapping_sub(u_string::unit_count(&(aa)), u_string::unit_count(&(bb)));
        }
        return PlanJsonNumber::plan_json_number_compare_digit_strings(aa.as_ustr(), bb.as_ustr());
    }

    pub(crate) fn plan_json_number_times_small(digits: &UStr, factor: u32) -> Result<UString, UStringFault> {
    let __units7 = u_string::units(&digits);
    let __count7 = u_string::unit_count(&digits);
        let mut out = Vec::<u16>::new();
        let mut carry = 0u32;
        let mut i = __count7;
        let __units8 = u_string::units(&digits);
        let __count8 = u_string::unit_count(&digits);
        while (i) > (0) {
            let product = u32::wrapping_add(u32::wrapping_mul(u32::wrapping_sub(u_string::unit_at_from(&__units8, u32::wrapping_sub(i, 1)).unwrap_or(0), 48), factor), carry);
            if u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((product) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) >= 56320 && u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((product) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) <= 57343 {
                match out.last() {
                    Some(&last) if last >= 55296 && last <= 56319 => {}
                    _ => return Err(UStringFault::UnpairedSurrogate { unit: u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((product) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) }),
                }
            } else if let Some(&last) = out.last() {
                if last >= 55296 && last <= 56319 {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                }
            }
            out.push(u16::try_from((u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((product) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes()))) & 0xFFFF).unwrap_or(0));
            carry = product / (10);
            i = u32::wrapping_sub(i, 1);
        }
        while (i32::from_ne_bytes(((carry) as i32).to_ne_bytes())) > (0) {
            if u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((carry) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) >= 56320 && u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((carry) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) <= 57343 {
                match out.last() {
                    Some(&last) if last >= 55296 && last <= 56319 => {}
                    _ => return Err(UStringFault::UnpairedSurrogate { unit: u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((carry) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes())) }),
                }
            } else if let Some(&last) = out.last() {
                if last >= 55296 && last <= 56319 {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(last) });
                }
            }
            out.push(u16::try_from((u32::wrapping_add(48, u32::from_ne_bytes(((i32::from_ne_bytes(((carry) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes()))) & 0xFFFF).unwrap_or(0));
            carry = carry / (10);
        }
        let text = UString::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?;
        return Ok(PlanJsonNumber::plan_json_number_reverse(text.as_ustr()));
    }

    pub(crate) fn plan_json_number_increment_decimal(digits: &UStr) -> UString {
        let mut a = u_string::split(&digits, &UString::from(""));
        let mut i = u32::wrapping_sub(u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0), 1);
        loop {
            if a[usize::try_from(i).unwrap_or(0)].clone() != UString::from("9") {
                { while a.len() <= usize::try_from(i).unwrap_or(0) { a.push(UString::new()); } a[usize::try_from(i).unwrap_or(0)] = if u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_add(u_string::unit_at(&(a[usize::try_from(i).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) as u16]) }; };
                return UString::from(format!("{}", { let joined = a; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(""); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str());
            }
            { while a.len() <= usize::try_from(i).unwrap_or(0) { a.push(UString::new()); } a[usize::try_from(i).unwrap_or(0)] = UString::from("0").to_ustring(); };
            if i == 0 {
                return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("1")); __s += UString::from(format!("{}", { let joined1 = a; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(""); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s }).as_str());
            }
            i = u32::wrapping_sub(i, 1);
        }
    }

    pub(crate) fn plan_json_number_decrement_decimal(digits: &UStr) -> UString {
        let mut a = u_string::split(&digits, &UString::from(""));
        let mut i = u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0);
        while (i) > (0) && (a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone() == UString::from("0") {
            { while a.len() <= usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0) { a.push(UString::new()); } a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = UString::from("9").to_ustring(); };
            i = u32::wrapping_sub(i, 1);
        }
        if u32::wrapping_sub(i, 1) <= 2147483647 {
            { while a.len() <= usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0) { a.push(UString::new()); } a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)] = if u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1) > 0xFFFF { u_string::from_units(&[0xD800 + (((u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) >> 10) as u16, 0xDC00 + (((u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(u32::wrapping_sub(u_string::unit_at(&(a[usize::try_from(u32::wrapping_sub(i, 1)).unwrap_or(0)]).clone(), 0u32).unwrap_or(0), 1)) as u16]) }; };
        }
        let out = { let joined2 = a; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(""); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) };
        let mut start = 0;
        while (start) < (i32::wrapping_sub(i32::from_ne_bytes(((u_string::unit_count(&(out))) as i32).to_ne_bytes()), 1)) && u_string::unit_at(&out, u32::from_ne_bytes(((start) as u32).to_ne_bytes())).as_ref().map_or(false, |v| v == &(48)) {
            start = i32::wrapping_add(start, 1);
        }
        return u_string::substr(&out, i32::from_ne_bytes(((start) as i32).to_ne_bytes()), None);
    }

    pub(crate) fn plan_json_number_decompose(v: f64) -> Result<PlanJsonDecomposed, UStringFault> {
        let f = PlanJsonNumber::plan_json_number_decompose_f32(v);
        let mut mantissa = UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(f.mant24)).as_str());
        let mut exponent = f.exp2;
        if i32::from_ne_bytes(((f.mant24) as i32).to_ne_bytes()) >= 8388608 {
            mantissa = PlanJsonNumber::plan_json_number_times_long(mantissa.as_ustr(), 536870912)?;
            exponent = u32::wrapping_sub(exponent, 29);
        }
        while (i32::from_ne_bytes(((u_string::unit_count(&(mantissa))) as i32).to_ne_bytes())) < (16) || u_string::unit_count(&(mantissa)) == 16 && (PlanJsonNumber::plan_json_number_compare_digit_strings(mantissa.as_ustr(), UStr::new(&[52,53,48,51,53,57,57,54,50,55,51,55,48,52,57,54]))) > 2147483647 {
            mantissa = PlanJsonNumber::plan_json_number_times_small(mantissa.as_ustr(), 2)?;
            exponent = u32::wrapping_sub(exponent, 1);
        }
        return Ok(PlanJsonDecomposed { mantissa: mantissa.clone(), exp2: exponent });
    }

    pub(crate) fn plan_json_number_decompose_f32(v: f64) -> PlanJsonF32Decomposed {
        let bits = FPHelper::float_to_i32(v);
        let raw_exponent = bits >> 23 & 255;
        let raw_mantissa = bits & 8388607;
        if u32::from_ne_bytes(((raw_exponent) as u32).to_ne_bytes()) == 0 {
            return PlanJsonF32Decomposed { mant24: u32::from_ne_bytes(((raw_mantissa) as u32).to_ne_bytes()), exp2: 4294967147u32 };
        }
        return PlanJsonF32Decomposed { mant24: u32::from_ne_bytes(((raw_mantissa) as u32).to_ne_bytes()) | 8388608, exp2: u32::from_ne_bytes(((i32::wrapping_sub(raw_exponent, 150)) as u32).to_ne_bytes()) };
    }

    pub(crate) fn plan_json_number_compare_digit_strings(a: &UStr, b: &UStr) -> u32 {
    let __units10 = u_string::units(&b);
    let __units9 = u_string::units(&a);
    let __count10 = u_string::unit_count(&b);
    let __count9 = u_string::unit_count(&a);
        if __count9 != __count10 {
            return u32::wrapping_sub(__count9, __count10);
        }
        let mut i = 0u32;
        let __units11 = u_string::units(&a);
        let __units12 = u_string::units(&b);
        let __count11 = u_string::unit_count(&a);
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count9) as i32).to_ne_bytes())) {
            let da = u_string::unit_at_from(&__units11, i).unwrap_or(0);
            let db = u_string::unit_at_from(&__units12, i).unwrap_or(0);
            if da != db {
                return u32::wrapping_sub(da, db);
            }
            i = u32::wrapping_add(i, 1);
        }
        return 0;
    }

    pub(crate) fn plan_json_number_zeros(n: u32) -> UString {
        let mut s = UString::new();
        for _ in 0..n {
            s += &(UString::from("0"));
        }
        return s;
    }

    pub(crate) fn plan_json_number_trim_zeros(s: &UStr) -> UString {
    let __units13 = u_string::units(&s);
    let __count12 = u_string::unit_count(&s);
        let mut e = __count12;
        while (i32::from_ne_bytes(((e) as i32).to_ne_bytes())) > (1) && u_string::unit_at_from(&__units13, u32::wrapping_sub(e, 1)).as_ref().map_or(false, |v| v == &(48)) {
            e = u32::wrapping_sub(e, 1);
        }
        return u_string::substr(&s, 0i32, Some(i32::from_ne_bytes(((e) as i32).to_ne_bytes())));
    }

    pub(crate) fn plan_json_number_reverse(s: &UStr) -> UString {
    let __units14 = u_string::units(&s);
    let __count13 = u_string::unit_count(&s);
        let mut r = UString::new();
        let mut i = __count13;
        let __units15 = u_string::units(&s);
        let __count14 = u_string::unit_count(&s);
        while (i) > (0) {
            r += &(u_string::char_at_from(&__units15, u32::wrapping_sub(i, 1)));
            i = u32::wrapping_sub(i, 1);
        }
        return r;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanJsonDigitsAndExponent {
    pub digits: UString,
    pub exponent: u32,
}

pub fn compare_plan_json_digits_and_exponent(a: &PlanJsonDigitsAndExponent, b: &PlanJsonDigitsAndExponent) -> i32 {
    let cmp_digits = SortedTable::sorted_table_compare_strings(a.digits.as_ustr(), b.digits.as_ustr());
    if cmp_digits != 0 { return cmp_digits; }
    let cmp_exponent = if a.exponent < b.exponent { -1 } else if a.exponent > b.exponent { 1 } else { 0 };
    if cmp_exponent != 0 { return cmp_exponent; }
    0
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanJsonDecomposed {
    pub mantissa: UString,
    pub exp2: u32,
}

pub fn compare_plan_json_decomposed(a: &PlanJsonDecomposed, b: &PlanJsonDecomposed) -> i32 {
    let cmp_mantissa = SortedTable::sorted_table_compare_strings(a.mantissa.as_ustr(), b.mantissa.as_ustr());
    if cmp_mantissa != 0 { return cmp_mantissa; }
    let cmp_exp2 = if a.exp2 < b.exp2 { -1 } else if a.exp2 > b.exp2 { 1 } else { 0 };
    if cmp_exp2 != 0 { return cmp_exp2; }
    0
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlanJsonF32Decomposed {
    pub mant24: u32,
    pub exp2: u32,
}

pub fn compare_plan_json_f32_decomposed(a: &PlanJsonF32Decomposed, b: &PlanJsonF32Decomposed) -> i32 {
    let cmp_mant24 = if a.mant24 < b.mant24 { -1 } else if a.mant24 > b.mant24 { 1 } else { 0 };
    if cmp_mant24 != 0 { return cmp_mant24; }
    let cmp_exp2 = if a.exp2 < b.exp2 { -1 } else if a.exp2 > b.exp2 { 1 } else { 0 };
    if cmp_exp2 != 0 { return cmp_exp2; }
    0
}
