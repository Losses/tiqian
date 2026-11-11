use crate::runtime::u_string;
use crate::runtime::u_string::UStr;


pub trait Hyphenator: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn Hyphenator>;
    fn hyphenate(&self, word: &UStr) -> Vec<u32>;
}

impl Clone for Box<dyn Hyphenator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn Hyphenator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct NoHyphenator {
}

impl NoHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, _word: &UStr) -> Vec<u32> {
        return vec![];
    }
}

impl Hyphenator for NoHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.linebreak.Hyphenator.NoHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, _word: &UStr) -> Vec<u32> {
        return vec![];
    }
}

#[derive(Clone, PartialEq)]
pub struct TailHyphenator {
}

impl TailHyphenator {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn hyphenate(&self, word: &UStr) -> Vec<u32> {
        return vec![u32::wrapping_sub(u_string::unit_count(&(word)), 5)];
    }
}

impl Hyphenator for TailHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.linebreak.Hyphenator.TailHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, word: &UStr) -> Vec<u32> {
        return vec![u32::wrapping_sub(u_string::unit_count(&(word)), 5)];
    }
}

#[derive(Clone, PartialEq)]
pub struct SyllableHyphenator {
    pub(crate) points: Vec<u32>,
}

impl SyllableHyphenator {
    pub fn new(points: Vec<u32>) -> Self {
        Self {
            points,
        }
    }

    pub fn hyphenate(&self, _word: &UStr) -> Vec<u32> {
        return ((self.points).clone()).clone();
    }
}

impl Hyphenator for SyllableHyphenator {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.linebreak.Hyphenator.SyllableHyphenator"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Hyphenator> {
        Box::new(self.clone())
    }

    fn hyphenate(&self, _word: &UStr) -> Vec<u32> {
        return ((self.points).clone()).clone();
    }
}
