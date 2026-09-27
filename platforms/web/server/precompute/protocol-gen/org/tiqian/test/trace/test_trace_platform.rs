use crate::runtime::fs::Fs;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct TestTracePlatform;

impl TestTracePlatform {
    pub const TEST_TRACE_PLATFORM_UPDATE_MODE: bool = true;
    pub const TEST_TRACE_PLATFORM_DOUBLE_ARITHMETIC: bool = true;
    pub const TEST_TRACE_PLATFORM_DIRECTORY: &UStr = unsafe { &*(&[0x0065u16, 0x006Eu16, 0x0067u16, 0x0069u16, 0x006Eu16, 0x0065u16, 0x002Du16, 0x0068u16, 0x0061u16, 0x0078u16, 0x0065u16, 0x002Fu16, 0x006Fu16, 0x0075u16, 0x0074u16, 0x002Fu16, 0x0068u16, 0x0061u16, 0x0078u16, 0x0065u16, 0x002Du16, 0x0074u16, 0x0072u16, 0x0061u16, 0x0063u16, 0x0065u16, 0x0073u16] as *const [u16] as *const crate::runtime::u_string::UStr) };

    pub fn test_trace_platform_write_golden(class_name: &UStr, text: &UStr) {
        Fs::make_dirs(TestTracePlatform::TEST_TRACE_PLATFORM_DIRECTORY.to_ustring().as_ustr());
        Fs::write_text((UString::from(format!("{}", { let mut __s = UString::new(); __s += TestTracePlatform::TEST_TRACE_PLATFORM_DIRECTORY.to_ustring().as_ustr(); __s += &(UString::from("/")); __s += class_name; __s += &(UString::from(".txt")); __s }).as_str())).as_ustr(), text);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MkdirOptions {
    pub recursive: bool,
}

pub fn compare_mkdir_options(a: &MkdirOptions, b: &MkdirOptions) -> i32 {
    let cmp_recursive = if a.recursive < b.recursive { -1 } else if a.recursive > b.recursive { 1 } else { 0 };
    if cmp_recursive != 0 { return cmp_recursive; }
    0
}
