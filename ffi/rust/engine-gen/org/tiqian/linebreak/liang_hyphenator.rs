use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Clone, PartialEq)]
pub struct LiangHyphenator {
    pub(crate) patterns: SortedMapTable<String, Vec<u32>>,
    pub(crate) exceptions: SortedMapTable<String, Vec<u32>>,
    pub(crate) left_min: u32,
    pub(crate) right_min: u32,
}

impl LiangHyphenator {
    pub fn new(patterns: SortedMapTable<String, Vec<u32>>, exceptions: Option<SortedMapTable<String, Vec<u32>>>, left_min: Option<u32>, right_min: Option<u32>) -> Self {
        let left_min = left_min.unwrap_or_else(|| 2);
        let right_min = right_min.unwrap_or_else(|| 3);
        Self {
            patterns,
            exceptions: exceptions.unwrap_or(SortedTable::sorted_table_map_builder::<String, Vec<u32>>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str()))).build()),
            left_min: left_min,
            right_min: right_min,
        }
    }

    pub fn hyphenate(&self, word: &str) -> Vec<u32> {
    let __units = u_string::units(&word);
    let __count = u_string::unit_count(&word);
        if i32::from_ne_bytes((__count).to_ne_bytes()) < (i32::from_ne_bytes((u32::wrapping_add(self.left_min, self.right_min)).to_ne_bytes())) {
            return vec![];
        }
        let lower = word.to_lowercase();
        let explicit = (self.exceptions).clone().get(&(lower).to_string());
        match &(explicit) {
            Some(__option) => {
                let mut out: Vec<u32> = vec![];
                for &v in __option {
                    if ({ let v: u32 = v; i32::from_ne_bytes(v.to_ne_bytes()) }) >= i32::from_ne_bytes((self.left_min).to_ne_bytes()) && ({ let v: u32 = v; i32::from_ne_bytes(v.to_ne_bytes()) }) <= i32::from_ne_bytes((u32::wrapping_sub(__count, self.right_min)).to_ne_bytes()) {
                        out.push(v);
                    }
                }
                return out;
            }
            None => {
            }
        }
        let work = format!("{}{}{}",
            ".",
            lower,
            "."
        );
        let mut _g: Vec<u32> = vec![];
        for _ in 0..u32::wrapping_add(u_string::unit_count(&(work)), 1) {
            _g.push(0);
        }
        let mut levels = (_g).clone();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(work))).to_ne_bytes())) {
            let mut key = String::new();
            let mut j = u32::wrapping_add(i, 1);
            while (i32::from_ne_bytes((j).to_ne_bytes())) <= i32::from_ne_bytes((u_string::unit_count(&(work))).to_ne_bytes()) {
                key += &(u_string::substring(&work, i32::from_ne_bytes((u32::wrapping_sub(j, 1)).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes((u32::wrapping_sub(j, 1)).to_ne_bytes()), 1)));
                let pattern = (self.patterns).clone().get(&(key).to_string());
                match &(pattern) {
                    Some(__option1) => {
                        let mut k = 0u32;
                        while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((__option1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            if ({ let v: u32 = __option1[usize::try_from(k).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = levels[usize::try_from(u32::wrapping_add(i, k)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                { while levels.len() <= usize::try_from(u32::wrapping_add(i, k)).unwrap_or(0) { levels.push(0); } levels[usize::try_from(u32::wrapping_add(i, k)).unwrap_or(0)] = __option1[usize::try_from(k).unwrap_or(0)]; };
                            }
                            k = u32::wrapping_add(k, 1);
                        }
                    }
                    None => {
                    }
                }
                j = u32::wrapping_add(j, 1);
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut result: Vec<u32> = vec![];
        let mut m = 0;
        while (m) < (i32::wrapping_sub(i32::from_ne_bytes((__count).to_ne_bytes()), 1)) {
            let offset = i32::wrapping_add(m, 1);
            if offset >= i32::from_ne_bytes((self.left_min).to_ne_bytes()) && (offset) <= i32::wrapping_sub(i32::from_ne_bytes((__count).to_ne_bytes()), i32::from_ne_bytes((self.right_min).to_ne_bytes())) && u32::from_ne_bytes(({ let v: u32 =
levels[usize::try_from(i32::wrapping_add(m, 2)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) } % 2i32).to_ne_bytes()) == 1 {
                result.push(u32::from_ne_bytes((offset).to_ne_bytes()));
            }
            m = i32::wrapping_add(m, 1);
        }
        return result;
    }
}

impl Hyphenator for LiangHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.linebreak.LiangHyphenator.LiangHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, word: &str) -> Vec<u32> {
    let __units1 = u_string::units(&word);
    let __count1 = u_string::unit_count(&word);
        if i32::from_ne_bytes((__count1).to_ne_bytes()) < (i32::from_ne_bytes((u32::wrapping_add(self.left_min, self.right_min)).to_ne_bytes())) {
            return vec![];
        }
        let lower = word.to_lowercase();
        let explicit = (self.exceptions).clone().get(&(lower).to_string());
        match &(explicit) {
            Some(__option2) => {
                let mut out: Vec<u32> = vec![];
                for &v in __option2 {
                    if ({ let v: u32 = v; i32::from_ne_bytes(v.to_ne_bytes()) }) >= i32::from_ne_bytes((self.left_min).to_ne_bytes()) && ({ let v: u32 = v; i32::from_ne_bytes(v.to_ne_bytes()) }) <= i32::from_ne_bytes((u32::wrapping_sub(__count1, self.right_min)).to_ne_bytes()) {
                        out.push(v);
                    }
                }
                return out;
            }
            None => {
            }
        }
        let work = format!("{}{}{}",
            ".",
            lower,
            "."
        );
        let mut _g: Vec<u32> = vec![];
        for _ in 0..u32::wrapping_add(u_string::unit_count(&(work)), 1) {
            _g.push(0);
        }
        let mut levels = (_g).clone();
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < (i32::from_ne_bytes((u_string::unit_count(&(work))).to_ne_bytes())) {
            let mut key = String::new();
            let mut j = u32::wrapping_add(i, 1);
            while (i32::from_ne_bytes((j).to_ne_bytes())) <= i32::from_ne_bytes((u_string::unit_count(&(work))).to_ne_bytes()) {
                key += &(u_string::substring(&work, i32::from_ne_bytes((u32::wrapping_sub(j, 1)).to_ne_bytes()), i32::wrapping_add(i32::from_ne_bytes((u32::wrapping_sub(j, 1)).to_ne_bytes()), 1)));
                let pattern = (self.patterns).clone().get(&(key).to_string());
                match &(pattern) {
                    Some(__option3) => {
                        let mut k = 0u32;
                        while (i32::from_ne_bytes((k).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((__option3.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            if ({ let v: u32 = __option3[usize::try_from(k).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) > ({ let v: u32 = levels[usize::try_from(u32::wrapping_add(i, k)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) }) {
                                { while levels.len() <= usize::try_from(u32::wrapping_add(i, k)).unwrap_or(0) { levels.push(0); } levels[usize::try_from(u32::wrapping_add(i, k)).unwrap_or(0)] = __option3[usize::try_from(k).unwrap_or(0)]; };
                            }
                            k = u32::wrapping_add(k, 1);
                        }
                    }
                    None => {
                    }
                }
                j = u32::wrapping_add(j, 1);
            }
            i = u32::wrapping_add(i, 1);
        }
        let mut result: Vec<u32> = vec![];
        let mut m = 0;
        while (m) < (i32::wrapping_sub(i32::from_ne_bytes((__count1).to_ne_bytes()), 1)) {
            let offset = i32::wrapping_add(m, 1);
            if offset >= i32::from_ne_bytes((self.left_min).to_ne_bytes()) && (offset) <= i32::wrapping_sub(i32::from_ne_bytes((__count1).to_ne_bytes()), i32::from_ne_bytes((self.right_min).to_ne_bytes())) && u32::from_ne_bytes(({ let v: u32 =
levels[usize::try_from(i32::wrapping_add(m, 2)).unwrap_or(0)]; i32::from_ne_bytes(v.to_ne_bytes()) } % 2i32).to_ne_bytes()) == 1 {
                result.push(u32::from_ne_bytes((offset).to_ne_bytes()));
            }
            m = i32::wrapping_add(m, 1);
        }
        return result;
    }
}
