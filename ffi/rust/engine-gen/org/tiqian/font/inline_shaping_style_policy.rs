use crate::runtime::u_string::UString;
use std::sync::LazyLock;


pub static INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES: LazyLock<Vec<UString>> = LazyLock::new(|| vec![
    UString::from("font-feature-settings").to_ustring(),
    UString::from("font-variation-settings").to_ustring(),
    UString::from("font-stretch").to_ustring(),
    UString::from("font-kerning").to_ustring(),
    UString::from("font-optical-sizing").to_ustring(),
    UString::from("font-variant-ligatures").to_ustring(),
    UString::from("font-variant-alternates").to_ustring(),
    UString::from("font-variant-east-asian").to_ustring(),
    UString::from("font-variant-caps").to_ustring(),
    UString::from("font-variant-numeric").to_ustring(),
    UString::from("font-variant-position").to_ustring(),
    UString::from("font-language-override").to_ustring(),
    UString::from("font-size-adjust").to_ustring(),
    UString::from("word-spacing").to_ustring(),
    UString::from("text-transform").to_ustring(),
    UString::from("text-rendering").to_ustring(),
]);

#[derive(Clone, Copy)]
pub struct InlineShapingStylePolicy;

impl InlineShapingStylePolicy {

    pub fn inline_shaping_style_policy_first_divergent_property(element_values: &[UString], paragraph_values: &[UString]) -> Option<UString> {
        let mut n = if i32::from_ne_bytes(((u32::try_from((element_values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) < (i32::from_ne_bytes(((u32::try_from((paragraph_values.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { u32::try_from((element_values.len()) & 0xFFFF_FFFF).unwrap_or(0) } else { u32::try_from((paragraph_values.len()) & 0xFFFF_FFFF).unwrap_or(0) };
        if ({ let v: u32 = n; i32::from_ne_bytes(v.to_ne_bytes()) }) > (i32::from_ne_bytes(((u32::try_from(((*INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            n = u32::try_from(((*INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes(((i) as i32).to_ne_bytes())) < ({ let v: u32 = n; i32::from_ne_bytes(v.to_ne_bytes()) }) {
            if element_values[usize::try_from(i).unwrap_or(0)].clone() != (paragraph_values[usize::try_from(i).unwrap_or(0)]).clone() {
                return Some(((*INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES).clone()[usize::try_from(i).unwrap_or(0)]).clone());
            }
            i = u32::wrapping_add(i, 1);
        }
        return None;
    }
}
