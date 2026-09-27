#[derive(Clone, Copy)]
pub struct LineBreakFns;

impl LineBreakFns {
    pub fn line_break_fns_is_mandatory_break_code_point(code_point: u32) -> bool {
        return code_point == 10 || code_point == 11 || code_point == 12 || code_point == 13 || code_point == 133 || code_point == 8232 || code_point == 8233;
    }

    pub fn line_break_fns_is_zero_width_space_code_point(code_point: u32) -> bool {
        return code_point == 8203;
    }
}
