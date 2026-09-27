use crate::org::tiqian::core::text_range::TextRange;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Debug, Clone, PartialEq)]
pub struct ShapingDecisionInfo {
    pub range: TextRange,
    pub source_text: UString,
    pub display_text: UString,
    pub font_key: UString,
    pub glyph_count: u32,
    pub advance: f64,
    pub source: UString,
    pub reason: UString,
    pub glyphs_without_ink_bounds: u32,
    pub missing_glyphs: u32,
    pub resolved_face: Option<UString>,
    pub script: Option<UString>,
    pub language: Option<UString>,
    pub strategy: Option<UString>,
    pub feature_evidence: Option<UString>,
    pub capability_issue: Option<UString>,
}

impl ShapingDecisionInfo {
    pub fn new(range: TextRange, source_text: &UStr, display_text: &UStr, font_key: &UStr, glyph_count: u32, advance: f64, source: &UStr, reason: &UStr, glyphs_without_ink_bounds: Option<u32>, missing_glyphs: Option<u32>, resolved_face: Option<UString>, script: Option<UString>, language: Option<UString>, strategy: Option<UString>, feature_evidence: Option<UString>, capability_issue: Option<UString>) -> Self {
        let glyphs_without_ink_bounds = glyphs_without_ink_bounds.unwrap_or_else(|| 0);
        let missing_glyphs = missing_glyphs.unwrap_or_else(|| 0);
        let resolved_face = resolved_face.or_else(|| None);
        let script = script.or_else(|| None);
        let language = language.or_else(|| None);
        let strategy = strategy.or_else(|| None);
        let feature_evidence = feature_evidence.or_else(|| None);
        let capability_issue = capability_issue.or_else(|| None);
        Self {
            range,
            source_text: source_text.to_ustring(),
            display_text: display_text.to_ustring(),
            font_key: font_key.to_ustring(),
            glyph_count,
            advance,
            source: source.to_ustring(),
            reason: reason.to_ustring(),
            glyphs_without_ink_bounds: glyphs_without_ink_bounds,
            missing_glyphs: missing_glyphs,
            resolved_face: resolved_face,
            script: script,
            language: language,
            strategy: strategy,
            feature_evidence: feature_evidence,
            capability_issue: capability_issue,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ShapingDecisionInfo(")); __s += &(UString::from("range=")); __s += UString::from(format!("{}", (self.range).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("sourceText=")); __s += (self.source_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontKey=")); __s += (self.font_key).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("glyphCount=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.glyph_count)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += (self.source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("glyphsWithoutInkBounds=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.glyphs_without_ink_bounds)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("missingGlyphs=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.missing_glyphs)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("resolvedFace=")); __s += match &((self.resolved_face).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("script=")); __s += match &((self.script).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("language=")); __s += match &((self.language).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("strategy=")); __s += match &((self.strategy).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("featureEvidence=")); __s += match &((self.feature_evidence).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("capabilityIssue=")); __s += match &((self.capability_issue).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(")")); __s }).as_str());
    }
}
