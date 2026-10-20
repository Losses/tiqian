use crate::org::tiqian::clreq::bopomofo_tone::BopomofoTone;
use crate::runtime::u_string;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct BopomofoReading {
    pub symbols: Vec<UString>,
    pub tone: BopomofoTone,
}

impl BopomofoReading {
    pub fn new(symbols: Vec<UString>, tone: BopomofoTone) -> Self {
        Self {
            symbols,
            tone,
        }
    }

    pub fn copy(&self) -> BopomofoReading {
        let mut copied_symbols: Vec<UString> = vec![];
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((self.symbols).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            copied_symbols.push((self.symbols[usize::try_from(index).unwrap_or(0)]).clone());
            index = u32::wrapping_add(index, 1);
        }
        return BopomofoReading::new(copied_symbols.to_vec(), self.tone);
    }

    pub fn hash_code(&self) -> u32 {
        let mut hash = 17u32;
        hash = u32::wrapping_add(u32::wrapping_mul(hash, 31), self.tone_index());
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from(((self.symbols).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let symbol = (self.symbols[usize::try_from(index).unwrap_or(0)]).clone();
            let mut unit_index = 0u32;
            let __units = u_string::units(&symbol);
            let __count = u_string::unit_count(&symbol);
            while (i32::from_ne_bytes(((unit_index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) {
                hash = u32::wrapping_add(u32::wrapping_mul(hash, 31), u_string::unit_at_from(&__units, unit_index).unwrap_or(0));
                unit_index = u32::wrapping_add(unit_index, 1);
            }
            index = u32::wrapping_add(index, 1);
        }
        return hash;
    }

    fn tone_index(&self) -> u32 {
        let value = self.tone;
        return match value {
            BopomofoTone::Yinping => 0,
            BopomofoTone::Yangping => 1,
            BopomofoTone::Shang => 2,
            BopomofoTone::Qu => 3,
            BopomofoTone::Neutral => 4,
            BopomofoTone::Ru => 5,
        };
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("BopomofoReading(")); __s += &(UString::from("symbols=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.symbols).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("tone=")); __s += UString::from(self.tone.name()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

fn bopomofo_reading_tone_order(v: &BopomofoTone) -> i32 {
    match v {
        BopomofoTone::Yinping => 0,
        BopomofoTone::Yangping => 1,
        BopomofoTone::Shang => 2,
        BopomofoTone::Ru => 5,
        BopomofoTone::Qu => 3,
        BopomofoTone::Neutral => 4,
    }
}
pub fn compare_bopomofo_reading(a: &BopomofoReading, b: &BopomofoReading) -> i32 {
    let mut cmp_symbols = 0; for (av, bv) in a.symbols.iter().zip(b.symbols.iter()) { cmp_symbols = match av.cmp(bv) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; if cmp_symbols != 0 { break; } }
    if cmp_symbols == 0 { cmp_symbols = match a.symbols.len().cmp(&b.symbols.len()) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 }; }
    if cmp_symbols != 0 { return cmp_symbols; }
    let cmp_tone = match bopomofo_reading_tone_order(&a.tone).cmp(&bopomofo_reading_tone_order(&b.tone)) { core::cmp::Ordering::Less => -1, core::cmp::Ordering::Equal => 0, core::cmp::Ordering::Greater => 1 };
    if cmp_tone != 0 { return cmp_tone; }
    0
}
