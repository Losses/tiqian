pub struct IntText;

impl IntText {
    pub fn int_text(v: u32) -> String {
        i32::from_ne_bytes(v.to_ne_bytes()).to_string()
    }
}
