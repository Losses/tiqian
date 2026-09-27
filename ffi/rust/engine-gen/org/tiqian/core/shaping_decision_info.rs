use crate::org::tiqian::core::text_range::TextRange;


#[derive(Debug, Clone, PartialEq)]
pub struct ShapingDecisionInfo {
    pub range: TextRange,
    pub source_text: String,
    pub display_text: String,
    pub font_key: String,
    pub glyph_count: u32,
    pub advance: f64,
    pub source: String,
    pub reason: String,
    pub glyphs_without_ink_bounds: u32,
    pub missing_glyphs: u32,
    pub resolved_face: Option<String>,
    pub script: Option<String>,
    pub language: Option<String>,
    pub strategy: Option<String>,
    pub feature_evidence: Option<String>,
    pub capability_issue: Option<String>,
}

impl ShapingDecisionInfo {
    pub fn new(range: TextRange, source_text: &str, display_text: &str, font_key: &str, glyph_count: u32, advance: f64, source: &str, reason: &str, glyphs_without_ink_bounds: Option<u32>, missing_glyphs: Option<u32>, resolved_face: Option<String>, script: Option<String>,
language: Option<String>, strategy: Option<String>, feature_evidence: Option<String>, capability_issue: Option<String>) -> Self {
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
            source_text: source_text.to_string(),
            display_text: display_text.to_string(),
            font_key: font_key.to_string(),
            glyph_count,
            advance,
            source: source.to_string(),
            reason: reason.to_string(),
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

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ShapingDecisionInfo(",
            "range=",
            (self.range).clone().to_string(),
            ", ",
            "sourceText=",
            (self.source_text).to_string(),
            ", ",
            "displayText=",
            (self.display_text).to_string(),
            ", ",
            "fontKey=",
            (self.font_key).to_string(),
            ", ",
            "glyphCount=",
            crate::runtime::int_text::IntText::int_text(self.glyph_count),
            ", ",
            "advance=",
            self.advance,
            ", ",
            "source=",
            (self.source).to_string(),
            ", ",
            "reason=",
            (self.reason).to_string(),
            ", ",
            "glyphsWithoutInkBounds=",
            crate::runtime::int_text::IntText::int_text(self.glyphs_without_ink_bounds),
            ", ",
            "missingGlyphs=",
            crate::runtime::int_text::IntText::int_text(self.missing_glyphs),
            ", ",
            "resolvedFace=",
            match (self.resolved_face).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "script=",
            match (self.script).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "language=",
            match (self.language).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "strategy=",
            match (self.strategy).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "featureEvidence=",
            match (self.feature_evidence).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ", ",
            "capabilityIssue=",
            match (self.capability_issue).clone() { Some(ref v) => v.to_string(), None => "null".to_string() },
            ")"
        );
    }
}
