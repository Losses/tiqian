use crate::org::tiqian::linebreak::break_kind::BreakKind;
use crate::org::tiqian::linebreak::break_opportunity::BreakOpportunity;
use crate::org::tiqian::linebreak::line_break_fns::LineBreakFns;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


pub trait LineBreakAnalyzer: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn LineBreakAnalyzer>;
    fn analyze(&self, text: &UStr) -> Vec<BreakOpportunity>;
}

impl Clone for Box<dyn LineBreakAnalyzer> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn LineBreakAnalyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct SimpleCharacterLineBreakAnalyzer {
}

impl SimpleCharacterLineBreakAnalyzer {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn analyze(&self, text: &UStr) -> Vec<BreakOpportunity> {
    let __units = u_string::units(&text);
    let __count = u_string::unit_count(&text);
        let mut result: Vec<BreakOpportunity> = Vec::new();
        if __count == 0 {
            return result;
        }
        let mut index = 1u32;
        let __units1 = u_string::units(&text);
        let __count1 = u_string::unit_count(&text);
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((__count) as i32).to_ne_bytes()) {
            let prev = u_string::unit_at_from(&__units, u32::wrapping_sub(index, 1)).unwrap_or(0);
            let mandatory = LineBreakFns::line_break_fns_is_mandatory_break_code_point(prev) && !(prev == 13 && (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count) as i32).to_ne_bytes())) && u_string::unit_at_from(&__units1, index).as_ref().map_or(false, |v| v == &(10)));
            result.push(BreakOpportunity::new(index, if index == __count || mandatory { BreakKind::Required } else { BreakKind::Allowed }, if mandatory { UString::from("MandatoryBreak") } else { UString::from("SimpleCharacterLineBreakAnalyzer") }.as_ustr(), Some(0)));
            index = u32::wrapping_add(index, 1);
        }
        return result;
    }
}

impl LineBreakAnalyzer for SimpleCharacterLineBreakAnalyzer {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.linebreak.LineBreakAnalyzer.SimpleCharacterLineBreakAnalyzer"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn LineBreakAnalyzer> {
        Box::new(self.clone())
    }

    fn analyze(&self, text: &UStr) -> Vec<BreakOpportunity> {
    let __units2 = u_string::units(&text);
    let __count2 = u_string::unit_count(&text);
        let mut result: Vec<BreakOpportunity> = Vec::new();
        if __count2 == 0 {
            return result;
        }
        let mut index = 1u32;
        let __units3 = u_string::units(&text);
        let __count3 = u_string::unit_count(&text);
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) <= i32::from_ne_bytes(((__count2) as i32).to_ne_bytes()) {
            let prev = u_string::unit_at_from(&__units2, u32::wrapping_sub(index, 1)).unwrap_or(0);
            let mandatory = LineBreakFns::line_break_fns_is_mandatory_break_code_point(prev) && !(prev == 13 && (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((__count2) as i32).to_ne_bytes())) && u_string::unit_at_from(&__units3, index).as_ref().map_or(false, |v| v == &(10)));
            result.push(BreakOpportunity::new(index, if index == __count2 || mandatory { BreakKind::Required } else { BreakKind::Allowed }, if mandatory { UString::from("MandatoryBreak") } else { UString::from("SimpleCharacterLineBreakAnalyzer") }.as_ustr(), Some(0)));
            index = u32::wrapping_add(index, 1);
        }
        return result;
    }
}
