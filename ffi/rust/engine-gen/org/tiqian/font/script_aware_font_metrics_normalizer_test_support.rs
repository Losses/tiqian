use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_role::FontRole;


#[derive(Clone, Copy)]
pub struct ScriptAwareFontMetricsNormalizerTestSupport;

impl ScriptAwareFontMetricsNormalizerTestSupport {
    pub fn script_aware_font_metrics_normalizer_test_support_req(key: &str, role: FontRole) -> FontMetricsRequest {
        return FontMetricsRequest::new(key, 16 as f64 as f64, role, "zh-Hans", Some(vec![]), Some(400), Some(false), Some("".to_string()));
    }
}
