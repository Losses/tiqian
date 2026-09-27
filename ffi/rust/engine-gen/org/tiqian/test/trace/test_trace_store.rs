use crate::runtime::u_string;
use crate::std::u_string_exception::UStringFault;
use std::sync::Mutex;


#[derive(Clone)]
pub struct TraceSectionState {
    pub name: String,
    pub lines: Vec<u16>,
}

impl TraceSectionState {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            lines: Vec::<u16>::new(),
        }
    }
}

#[derive(Clone)]
pub struct TraceClassState {
    pub class_name: String,
    pub sections: Vec<TraceSectionState>,
}

impl TraceClassState {
    pub fn new(class_name: &str) -> Self {
        Self {
            class_name: class_name.to_string(),
            sections: vec![],
        }
    }
}

static TEST_TRACE_STORE_CLASSES: Mutex<Vec<TraceClassState>> = Mutex::new(vec![]);

#[derive(Clone, Copy)]
pub struct TestTraceStore;

impl TestTraceStore {

    pub fn test_trace_store_open(class_name: &str, section_name: &str) {
        let mut class_state = TestTraceStore::test_trace_store_find_class(class_name);
        if class_state.is_none() {
            class_state = Some(TraceClassState::new(class_name).clone());
            TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner()).push(class_state.clone().unwrap());
        }
        let section = TestTraceStore::test_trace_store_find_section((class_state).as_ref().unwrap().clone(), section_name);
        if section.is_none() {
            ((class_state).as_ref().unwrap().sections).clone().push(TraceSectionState::new(section_name));
        }
    }

    pub fn test_trace_store_append(class_name: &str, section_name: &str, line: &str) -> Result<(), UStringFault> {
        let class_state = TestTraceStore::test_trace_store_find_class(class_name);
        if class_state.is_none() {
            return Ok(());
        }
        let mut section = TestTraceStore::test_trace_store_find_section((class_state).as_ref().unwrap().clone(), section_name);
        if section.is_none() {
            return Ok(());
        }
        if let Some(&unit) = (section).as_mut().unwrap().lines.last() {
            if unit >= 55296 && unit <= 56319 && !line.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        (section).as_mut().unwrap().lines.extend(line.encode_utf16());
        if let Some(&unit) = (section).as_mut().unwrap().lines.last() {
            if unit >= 55296 && unit <= 56319 && !concat!("\n",
"").is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        (section).as_mut().unwrap().lines.extend(concat!("\n",
"").encode_utf16());
        Ok(())
    }

    pub fn test_trace_store_class_text(class_name: &str) -> Result<String, UStringFault> {
        let class_state = TestTraceStore::test_trace_store_find_class(class_name);
        if class_state.is_none() {
            return Ok(format!("{}{}{}",
            "class: ",
            class_name,
            concat!("\n",
"")
        ));
        }
        let mut output = Vec::<u16>::new();
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !"class: ".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        output.extend("class: ".encode_utf16());
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !class_name.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        output.extend(class_name.encode_utf16());
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !concat!("\n",
"").is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        output.extend(concat!("\n",
"").encode_utf16());
        let mut order: Vec<u32> = Vec::new();
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((((class_state).as_ref().unwrap().sections).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let mut insertion = u32::try_from((order.len()) & 0xFFFF_FFFF).unwrap_or(0);
            while (i32::from_ne_bytes((insertion).to_ne_bytes())) > (0) && ((((((class_state).as_ref().unwrap().sections).clone()[usize::try_from(order[usize::try_from(u32::wrapping_sub(insertion, 1)).unwrap_or(0)]).unwrap_or(0)]).clone().name).to_string()).as_str()) >
((((((class_state).as_ref().unwrap().sections).clone()[usize::try_from(index).unwrap_or(0)]).clone().name).to_string()).as_str()) {
                insertion = u32::wrapping_sub(insertion, 1);
            }
            { let _a = &mut (order); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes((insertion).to_ne_bytes()); let _at = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos };
_a.insert(usize::try_from(_at).unwrap_or(0), index); };
            index = u32::wrapping_add(index, 1);
        }
        index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((order.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            let section = (((class_state).as_ref().unwrap().sections).clone()[usize::try_from(order[usize::try_from(index).unwrap_or(0)]).unwrap_or(0)]).clone();
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !"test: ".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend("test: ".encode_utf16());
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !(section.name).to_string().is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend((section.name).to_string().encode_utf16());
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !concat!("\n",
"").is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            output.extend(concat!("\n",
"").encode_utf16());
            let section_text = String::from_utf16(section.lines.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(section.lines[section.lines.len() - 1]) })?;
            if i32::from_ne_bytes((u_string::unit_count(&(section_text))).to_ne_bytes()) > (0) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !section_text.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                output.extend(section_text.encode_utf16());
            }
            index = u32::wrapping_add(index, 1);
        }
        return Ok(String::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub(crate) fn test_trace_store_find_class(class_name: &str) -> Option<TraceClassState> {
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((match u32::try_from(TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner()).len()) { Ok(value) => value, Err(_) => u32::MAX }).to_ne_bytes())) {
            if TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(index) { Ok(value) => value, Err(_) => 0usize }].clone().class_name.to_string() == class_name {
                return Some((TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(index) { Ok(value) => value, Err(_) => 0usize }]).clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return None;
    }

    pub(crate) fn test_trace_store_find_section(class_state: TraceClassState, section_name: &str) -> Option<TraceSectionState> {
        let mut index = 0u32;
        while (i32::from_ne_bytes((index).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((class_state.sections.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            if class_state.sections[usize::try_from(index).unwrap_or(0)].clone().name.to_string() == section_name {
                return Some((class_state.sections[usize::try_from(index).unwrap_or(0)]).clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return None;
    }
}
