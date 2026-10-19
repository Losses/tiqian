use crate::org::tiqian::protocol::font_contract::FontContract;
use crate::org::tiqian::protocol::js_coerce::JsCoerce;
use crate::org::tiqian::protocol::snapshot_typography::SnapshotTypography;
use crate::org::tiqian::protocol::wire_value::WireValue;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct FontContractTestSupport;

impl FontContractTestSupport {
    pub fn font_contract_test_support_typography() -> SnapshotTypography {
        return SnapshotTypography { font_families: vec![UString::from("Dela Gothic One").to_ustring()], font_size_px: 18.0f64, line_height_px: 27.0f64, locale: UString::from("").to_ustring(), font_weight: 400, italic: false, first_line_indent_ic: 0.0f64,
line_length_grid_enabled: false, letter_spacing_px: 0.0f64, font_feature_settings: UString::from("").to_ustring(), font_variation_settings: UString::from("").to_ustring(), font_variant_numeric: UString::from("").to_ustring() };
    }

    pub fn font_contract_test_support_run(key: WireValue, text: WireValue, spans: WireValue) -> Option<UString> {
        let probe = FontContract::font_contract_required_cjk_dash_contract_input((key).clone(), (text).clone(), (spans).clone(), FontContractTestSupport::font_contract_test_support_typography());
        return match &(probe) { None => None, Some(__option) => Some(JsCoerce::js_coerce_render_json((*__option).clone()).to_ustring()) };
    }
}
