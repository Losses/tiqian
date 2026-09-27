use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct ScriptAwareFontMetricsNormalizerTestSupport;

impl ScriptAwareFontMetricsNormalizerTestSupport {
    pub fn script_aware_font_metrics_normalizer_test_support_req(key: &UStr, role: FontRole) -> FontMetricsRequest {
        return FontMetricsRequest::new(key, 16 as f64 as f64, role, &(UStr::new(&[122,104,45,72,97,110,115])), Some(vec![]), Some(400), Some(false), Some(UString::from("")));
    }
}
