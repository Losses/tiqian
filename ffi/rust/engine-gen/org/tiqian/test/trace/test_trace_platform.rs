use crate::runtime::fs::Fs;


#[derive(Clone, Copy)]
pub struct TestTracePlatform;

impl TestTracePlatform {
    pub const TEST_TRACE_PLATFORM_UPDATE_MODE: bool = true;
    pub const TEST_TRACE_PLATFORM_DOUBLE_ARITHMETIC: bool = true;
    pub const TEST_TRACE_PLATFORM_DIRECTORY: &str = "engine-haxe/out/haxe-traces";

    pub fn test_trace_platform_write_golden(class_name: &str, text: &str) {
        Fs::make_dirs(TestTracePlatform::TEST_TRACE_PLATFORM_DIRECTORY.to_string().as_str());
        Fs::write_text(format!("{}{}{}{}",
            TestTracePlatform::TEST_TRACE_PLATFORM_DIRECTORY.to_string(),
            "/",
            class_name,
            ".txt"
        ).as_str(), text);
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
