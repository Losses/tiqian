use crate::org::tiqian::protocol::face_info::FaceInfo;
use crate::org::tiqian::protocol::snapshot_typography::SnapshotTypography;
use crate::runtime::u_string;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct ContextFingerprintTestSupport;

impl ContextFingerprintTestSupport {
    pub fn context_fingerprint_test_support_hex_bytes(bytes: &[u8]) -> UString {
        let digits = UString::from("0123456789abcdef").to_ustring();
        let mut out_b = UString::new();
        let mut index_idx = 0u32;
        while (i32::from_ne_bytes(((index_idx) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((bytes.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
            let byte = u32::from(bytes[usize::try_from(index_idx).unwrap_or(0)]);
            {
                let c = u_string::unit_at(&digits, byte >> 4 & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
            }
            {
                let c = u_string::unit_at(&digits, byte & 15).unwrap_or(0);
                out_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
            }
            index_idx = u32::wrapping_add(index_idx, 1);
        }
        return out_b;
    }

    pub fn context_fingerprint_test_support_harfbuzz_version() -> UString {
        return UString::from("harfrust-0.13.0").to_ustring();
    }

    pub fn context_fingerprint_test_support_typography() -> SnapshotTypography {
        return SnapshotTypography { font_families: vec![UString::from("Dela Gothic One").to_ustring()], font_size_px: 18.0f64, line_height_px: 27.0f64, locale: UString::from("").to_ustring(), font_weight: 400, italic: false, first_line_indent_ic: 0.0f64,
line_length_grid_enabled: false, letter_spacing_px: 0.0f64, font_feature_settings: UString::from("").to_ustring(), font_variation_settings: UString::from("").to_ustring(), font_variant_numeric: UString::from("").to_ustring() };
    }

    pub fn context_fingerprint_test_support_typography_at(font_size_px: f64) -> SnapshotTypography {
        return SnapshotTypography { font_families: vec![UString::from("Dela Gothic One").to_ustring()], font_size_px: font_size_px, line_height_px: 27.0f64, locale: UString::from("").to_ustring(), font_weight: 400, italic: false, first_line_indent_ic: 0.0f64,
line_length_grid_enabled: false, letter_spacing_px: 0.0f64, font_feature_settings: UString::from("").to_ustring(), font_variation_settings: UString::from("").to_ustring(), font_variant_numeric: UString::from("").to_ustring() };
    }

    pub fn context_fingerprint_test_support_face() -> FaceInfo {
        return FaceInfo { family: UString::from("Test Face").to_ustring(), style: UString::from("normal").to_ustring(), weight: vec![400.0f64, 700.0f64], unicode_range: UString::from("U+4E00-9FFF").to_ustring(),
public_url: UString::from("https://example.com/font.woff2").to_ustring(), source_sha256: UString::from("abcd0123").to_ustring(), sfnt_sha256: UString::from("ef01").to_ustring(), face_index: 1.0f64, source_order: 3,
axis_tags: vec![UString::from("wght").to_ustring(), UString::from("wdth").to_ustring()], local_names: vec![UString::from("Test Face").to_ustring()] };
    }
}
