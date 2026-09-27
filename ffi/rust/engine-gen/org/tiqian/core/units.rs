use crate::org::tiqian::core::ic::Ic;


#[derive(Clone, Copy)]
pub struct FloatIc;

impl FloatIc {
    pub fn float_ic_ic(value: f64) -> Ic {
        return Ic(value);
    }
}

#[derive(Clone, Copy)]
pub struct IntIc;

impl IntIc {
    pub fn int_ic_ic(value: u32) -> Ic {
        return Ic(value as f64);
    }
}
