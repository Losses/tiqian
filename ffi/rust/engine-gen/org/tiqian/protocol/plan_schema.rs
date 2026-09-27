use crate::runtime::u_string::UStr;


#[derive(Clone, Copy)]
pub struct PlanSchema;

impl PlanSchema {
    pub const PLAN_SCHEMA_PLAN_SCHEMA: u32 = 1;
    pub const PLAN_SCHEMA_PLAN_LAYOUT_REVISION: &UStr = unsafe { &*(&[0x0074u16, 0x0069u16, 0x0071u16, 0x0069u16, 0x0061u16, 0x006Eu16, 0x002Du16, 0x006Cu16, 0x0061u16, 0x0079u16, 0x006Fu16, 0x0075u16, 0x0074u16, 0x002Du16, 0x0076u16, 0x0032u16] as *const [u16] as *const crate::runtime::u_string::UStr) };
}
