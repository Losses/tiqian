use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;
use std::sync::Mutex;


#[derive(Clone)]
pub struct TraceSectionState {
    pub name: UString,
    pub lines: Vec<u16>,
}

impl TraceSectionState {
    pub fn new(name: &UStr) -> Self {
        Self {
            name: name.to_ustring(),
            lines: Vec::<u16>::new(),
        }
    }
}

#[derive(Clone)]
pub struct TraceClassState {
    pub class_name: UString,
    pub sections: Vec<TraceSectionState>,
}

impl TraceClassState {
    pub fn new(class_name: &UStr) -> Self {
        Self {
            class_name: class_name.to_ustring(),
            sections: vec![],
        }
    }
}

static TEST_TRACE_STORE_CLASSES: Mutex<Vec<TraceClassState>> = Mutex::new(vec![]);

#[derive(Clone, Copy)]
pub struct TestTraceStore;

impl TestTraceStore {

    pub fn test_trace_store_open(class_name: &UStr, section_name: &UStr) {
        let mut class_state = TestTraceStore::test_trace_store_find_class(class_name);
        if class_state.is_none() {
            class_state = Some(TraceClassState::new(class_name).clone());
            TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner()).push(class_state.clone().unwrap());
        }
        let section = TestTraceStore::test_trace_store_find_section((class_state).as_ref().unwrap().clone(), section_name);
        if section.is_none() {
            (class_state).as_mut().unwrap().sections.push(TraceSectionState::new(section_name));
        }
    }

    pub fn test_trace_store_append(class_name: &UStr, section_name: &UStr, line: &UStr) -> Result<(), UStringFault> {
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
                if !line.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        (section).as_mut().unwrap().lines.extend(line.encode_utf16());
        if let Some(&unit) = (section).as_mut().unwrap().lines.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(concat!("\n",
"")).is_empty() {
                if !UString::from(concat!("\n",
"")).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        (section).as_mut().unwrap().lines.extend(UString::from(concat!("\n",
"")).encode_utf16());
        Ok(())
    }

    pub fn test_trace_store_class_text(class_name: &UStr) -> Result<UString, UStringFault> {
        let class_state = TestTraceStore::test_trace_store_find_class(class_name);
        if class_state.is_none() {
            return Ok(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("class: ")); __s += class_name; __s += &(UString::from(concat!("\n",
""))); __s }).as_str()));
        }
        let mut output = Vec::<u16>::new();
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("class: ").is_empty() {
                if !UString::from("class: ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        output.extend(UString::from("class: ").encode_utf16());
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !class_name.is_empty() {
                if !class_name.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        output.extend(class_name.encode_utf16());
        if let Some(&unit) = output.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(concat!("\n",
"")).is_empty() {
                if !UString::from(concat!("\n",
"")).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        output.extend(UString::from(concat!("\n",
"")).encode_utf16());
        let mut order: Vec<u32> = Vec::new();
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((((class_state).as_ref().unwrap().sections).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let mut insertion = u32::try_from((order.len()) & 0xFFFF_FFFF).unwrap_or(0);
            while (i32::from_ne_bytes(((insertion) as i32).to_ne_bytes())) > (0) && (((((class_state).as_ref().unwrap().sections).clone()[usize::try_from(order[usize::try_from(u32::wrapping_sub(insertion, 1)).unwrap_or(0)]).unwrap_or(0)]).clone().name).to_ustring().as_ustr()) > (((((class_state).as_ref().unwrap().sections).clone()[usize::try_from(index).unwrap_or(0)]).clone().name).to_ustring().as_ustr()) {
                insertion = u32::wrapping_sub(insertion, 1);
            }
            { let _a = &mut (order); let _n = i32::try_from(_a.len()).unwrap_or(0); let _pos = i32::from_ne_bytes(((insertion) as i32).to_ne_bytes()); let _at = if _pos < 0 { let _tail = _n + _pos; if _tail < 0 { 0 } else { _tail } } else if _pos > _n { _n } else { _pos }; _a.insert(usize::try_from(_at).unwrap_or(0), index); };
            index = u32::wrapping_add(index, 1);
        }
        index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((order.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let section = (((class_state).as_ref().unwrap().sections).clone()[usize::try_from(order[usize::try_from(index).unwrap_or(0)]).unwrap_or(0)]).clone();
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("test: ").is_empty() {
                    if !UString::from("test: ").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend(UString::from("test: ").encode_utf16());
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !(section.name).to_ustring().is_empty() {
                    if !(section.name).to_ustring().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend((section.name).to_ustring().encode_utf16());
            if let Some(&unit) = output.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(concat!("\n",
"")).is_empty() {
                    if !UString::from(concat!("\n",
"")).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            output.extend(UString::from(concat!("\n",
"")).encode_utf16());
            let section_text = UString::from_utf16(section.lines.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(section.lines[section.lines.len() - 1]) })?;
            if i32::from_ne_bytes(((u_string::unit_count(&(section_text))) as i32).to_ne_bytes()) > (0) {
                if let Some(&unit) = output.last() {
                    if unit >= 55296 && unit <= 56319 && !section_text.is_empty() {
                        if !section_text.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                output.extend(section_text.encode_utf16());
            }
            index = u32::wrapping_add(index, 1);
        }
        return Ok(UString::from_utf16(output.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(output[output.len() - 1]) })?);
    }

    pub(crate) fn test_trace_store_find_class(class_name: &UStr) -> Option<TraceClassState> {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((match u32::try_from(TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner()).len()) { Ok(value) => value, Err(_) => u32::MAX }) as i32).to_ne_bytes())) {
            if TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(index) { Ok(value) => value, Err(_) => 0usize }].clone().class_name.to_ustring() == class_name {
                return Some((TEST_TRACE_STORE_CLASSES.lock().unwrap_or_else(|e| e.into_inner())[match usize::try_from(index) { Ok(value) => value, Err(_) => 0usize }]).clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return None;
    }

    pub(crate) fn test_trace_store_find_section(class_state: TraceClassState, section_name: &UStr) -> Option<TraceSectionState> {
        let mut index = 0u32;
        while (i32::from_ne_bytes(((index) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((class_state.sections.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            if class_state.sections[usize::try_from(index).unwrap_or(0)].clone().name.to_ustring() == section_name {
                return Some((class_state.sections[usize::try_from(index).unwrap_or(0)]).clone());
            }
            index = u32::wrapping_add(index, 1);
        }
        return None;
    }
}
