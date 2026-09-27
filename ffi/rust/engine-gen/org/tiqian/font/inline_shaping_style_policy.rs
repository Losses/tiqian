use std::sync::LazyLock;


pub static INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES: LazyLock<Vec<String>> = LazyLock::new(|| vec![
    "font-feature-settings".to_string(),
    "font-variation-settings".to_string(),
    "font-stretch".to_string(),
    "font-kerning".to_string(),
    "font-optical-sizing".to_string(),
    "font-variant-ligatures".to_string(),
    "font-variant-alternates".to_string(),
    "font-variant-east-asian".to_string(),
    "font-variant-caps".to_string(),
    "font-variant-numeric".to_string(),
    "font-variant-position".to_string(),
    "font-language-override".to_string(),
    "font-size-adjust".to_string(),
    "word-spacing".to_string(),
    "text-transform".to_string(),
    "text-rendering".to_string(),
]);

#[derive(Clone, Copy)]
pub struct InlineShapingStylePolicy;

impl InlineShapingStylePolicy {

    pub fn inline_shaping_style_policy_first_divergent_property(element_values: &[String], paragraph_values: &[String]) -> Option<String> {
        let mut n = if i32::from_ne_bytes((u32::try_from((element_values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) < (i32::from_ne_bytes((u32::try_from((paragraph_values.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { u32::try_from((element_values.len()) &
0xFFFF_FFFF).unwrap_or(0) } else { u32::try_from((paragraph_values.len()) & 0xFFFF_FFFF).unwrap_or(0) };
        if ({ let v: u32 = n; i32::from_ne_bytes(v.to_ne_bytes()) }) > (i32::from_ne_bytes((u32::try_from(((*INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
            n = u32::try_from(((*INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES).clone().len()) & 0xFFFF_FFFF).unwrap_or(0);
        }
        let mut i = 0u32;
        while (i32::from_ne_bytes((i).to_ne_bytes())) < ({ let v: u32 = n; i32::from_ne_bytes(v.to_ne_bytes()) }) {
            if element_values[usize::try_from(i).unwrap_or(0)].clone() != (paragraph_values[usize::try_from(i).unwrap_or(0)]).clone() {
                return Some(((*INLINE_SHAPING_STYLE_POLICY_UNSUPPORTED_INLINE_SHAPING_PROPERTIES).clone()[usize::try_from(i).unwrap_or(0)]).clone());
            }
            i = u32::wrapping_add(i, 1);
        }
        return None;
    }
}
