use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::glyph::Glyph;
use crate::org::tiqian::core::glyph_run::GlyphRun;
use crate::org::tiqian::core::rect::Rect;
use crate::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_metric_source::FontMetricSource;
use crate::org::tiqian::font::font_metrics::FontMetricsRequest;
use crate::org::tiqian::font::font_metrics::FontMetricsResolver;
use crate::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::shaping::text_shaper::ShapingInput;
use crate::org::tiqian::shaping::text_shaper::ShapingResult;
use crate::org::tiqian::shaping::text_shaper::TextShaperShapeFault;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::u_string;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub struct ShapingEvidenceEntry {
    pub key: ShapingEvidenceKey,
    pub result: RecordedShapingResult,
}

impl ShapingEvidenceEntry {
    pub fn new(key: ShapingEvidenceKey, result: RecordedShapingResult) -> Self {
        Self {
            key,
            result,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ShapingEvidenceEntry(")); __s += &(UString::from("key=")); __s += UString::from(format!("{}", (self.key).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("result=")); __s += UString::from(format!("{}", (self.result).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricsEvidenceEntry {
    pub key: MetricsEvidenceKey,
    pub result: RecordedFontMetrics,
}

impl MetricsEvidenceEntry {
    pub fn new(key: MetricsEvidenceKey, result: RecordedFontMetrics) -> Self {
        Self {
            key,
            result,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("MetricsEvidenceEntry(")); __s += &(UString::from("key=")); __s += UString::from(format!("{}", (self.key).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("result=")); __s += UString::from(format!("{}", (self.result).clone().to_string()).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct ShapingEvidence {
    pub meta: SortedMapTable<UString, UString>,
    pub shaping: Vec<ShapingEvidenceEntry>,
    pub metrics: Vec<MetricsEvidenceEntry>,
}

impl ShapingEvidence {
    pub fn new(meta: SortedMapTable<UString, UString>, shaping: Vec<ShapingEvidenceEntry>, metrics: Vec<MetricsEvidenceEntry>) -> Self {
        Self {
            meta,
            shaping,
            metrics,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ShapingEvidence(")); __s += &(UString::from("meta=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('{');
        let n = (self.meta).clone().size();
        let mut i = 0;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}={}", (self.meta).clone().key_at(i), (self.meta).clone().value_at(i));
            i += 1;
        }
        out.push('}');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("shaping=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.shaping).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("metrics=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.metrics).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShapingEvidenceKey {
    pub display_text: UString,
    pub font_key: UString,
    pub font_family: UString,
    pub role: UString,
    pub style_font_families: Vec<UString>,
    pub font_size: f64,
    pub font_weight: u32,
    pub italic: bool,
    pub locale: UString,
    pub open_type_features: Vec<UString>,
}

impl ShapingEvidenceKey {
    pub fn new(display_text: &UStr, font_key: &UStr, font_family: &UStr, role: &UStr, style_font_families: Vec<UString>, font_size: f64, font_weight: u32, italic: bool, locale: &UStr, open_type_features: Vec<UString>) -> Self {
        Self {
            display_text: display_text.to_ustring(),
            font_key: font_key.to_ustring(),
            font_family: font_family.to_ustring(),
            role: role.to_ustring(),
            style_font_families,
            font_size,
            font_weight,
            italic,
            locale: locale.to_ustring(),
            open_type_features,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ShapingEvidenceKey(")); __s += &(UString::from("displayText=")); __s += (self.display_text).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontKey=")); __s += (self.font_key).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontFamily=")); __s += (self.font_family).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += (self.role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("styleFontFamilies=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.style_font_families).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontSize=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.font_size)); __s += &(UString::from(", ")); __s += &(UString::from("fontWeight=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.font_weight)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("italic=")); __s += UString::from(format!("{}", (self.italic).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("locale=")); __s += (self.locale).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("openTypeFeatures=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.open_type_features).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, Copy)]
pub struct ShapingEvidenceFns;

impl ShapingEvidenceFns {
    pub fn shaping_evidence_fns_shaping_input_to_evidence_key(input: ShapingInput) -> ShapingEvidenceKey {
        return ShapingEvidenceKey::new((input.display_text).to_ustring().as_ustr(), (((input.font_decision).clone().candidate).clone().key).to_ustring().as_ustr(), (((input.font_decision).clone().candidate).clone().family).to_ustring().as_ustr(), UString::from((input.font_decision).clone().role.name()).as_ustr(), ((input.style).clone().font_families).clone(), (input.style).clone().font_size, (input.style).clone().font_weight, (input.style).clone().italic, ((input.style).clone().locale).to_ustring().as_ustr(), (input.open_type_features).clone());
    }

    pub fn shaping_evidence_fns_shaping_result(evidence: ShapingEvidence, key: ShapingEvidenceKey) -> Option<RecordedShapingResult> {
        for i in 0..match u32::try_from(evidence.shaping.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if ShapingEvidenceFns::shaping_evidence_fns_shaping_key_equals(((evidence.shaping[usize::try_from(i).unwrap_or(0)]).clone().key).clone(), (key).clone()) {
                return Some((((evidence.shaping[usize::try_from(i).unwrap_or(0)]).clone().result).clone()).clone());
            }
        }
        return None;
    }

    pub fn shaping_evidence_fns_shaping_key_equals(a: ShapingEvidenceKey, b: ShapingEvidenceKey) -> bool {
        if a == b {
            return true;
        }
        if a.display_text.to_ustring() != (b.display_text).to_ustring() {
            return false;
        }
        if a.font_key.to_ustring() != (b.font_key).to_ustring() {
            return false;
        }
        if a.font_family.to_ustring() != (b.font_family).to_ustring() {
            return false;
        }
        if a.role.to_ustring() != (b.role).to_ustring() {
            return false;
        }
        if !ShapingEvidenceFns::shaping_evidence_fns_string_array_equals(&a.style_font_families, &b.style_font_families) {
            return false;
        }
        if a.font_size != b.font_size {
            return false;
        }
        if a.font_weight != b.font_weight {
            return false;
        }
        if a.italic != b.italic {
            return false;
        }
        if a.locale.to_ustring() != (b.locale).to_ustring() {
            return false;
        }
        if !ShapingEvidenceFns::shaping_evidence_fns_string_array_equals(&a.open_type_features, &b.open_type_features) {
            return false;
        }
        return true;
    }

    pub fn shaping_evidence_fns_string_array_equals(a: &[UString], b: &[UString]) -> bool {
        if u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) != u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) {
            return false;
        }
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if a[usize::try_from(i).unwrap_or(0)].clone() != (b[usize::try_from(i).unwrap_or(0)]).clone() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedGlyph {
    pub id: u32,
    pub advance: f64,
    pub x: f64,
    pub y: f64,
    pub bounds: Option<Rect>,
    pub halt_advance: Option<f64>,
    pub halt_placement_x: Option<f64>,
}

impl RecordedGlyph {
    pub fn new(id: u32, advance: f64, x: f64, y: f64, bounds: Option<Rect>, halt_advance: Option<f64>, halt_placement_x: Option<f64>) -> Self {
        Self {
            id,
            advance,
            x,
            y,
            bounds,
            halt_advance,
            halt_placement_x,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RecordedGlyph(")); __s += &(UString::from("id=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.id)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("x=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.x)); __s += &(UString::from(", ")); __s += &(UString::from("y=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.y)); __s += &(UString::from(", ")); __s += &(UString::from("bounds=")); __s += (match &(self.bounds) { None => UString::from("null"), Some(__option) => UString::from(format!("{}", __option.to_string()).as_str()) }).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("haltAdvance=")); __s += &(match self.halt_advance { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("haltPlacementX=")); __s += &(match self.halt_placement_x { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedShapingDecision {
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

impl RecordedShapingDecision {
    pub fn new(glyph_count: u32, advance: f64, source: &UStr, reason: &UStr, glyphs_without_ink_bounds: u32, missing_glyphs: u32, resolved_face: Option<UString>, script: Option<UString>, language: Option<UString>, strategy: Option<UString>, feature_evidence: Option<UString>, capability_issue: Option<UString>) -> Self {
        Self {
            glyph_count,
            advance,
            source: source.to_ustring(),
            reason: reason.to_ustring(),
            glyphs_without_ink_bounds,
            missing_glyphs,
            resolved_face,
            script,
            language,
            strategy,
            feature_evidence,
            capability_issue,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RecordedShapingDecision(")); __s += &(UString::from("glyphCount=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.glyph_count)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("advance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.advance)); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += (self.source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("reason=")); __s += (self.reason).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("glyphsWithoutInkBounds=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.glyphs_without_ink_bounds)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("missingGlyphs=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.missing_glyphs)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("resolvedFace=")); __s += match &((self.resolved_face).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("script=")); __s += match &((self.script).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("language=")); __s += match &((self.language).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("strategy=")); __s += match &((self.strategy).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("featureEvidence=")); __s += match &((self.feature_evidence).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(", ")); __s += &(UString::from("capabilityIssue=")); __s += match &((self.capability_issue).clone()) { Some(v) => v.as_ustr(), None => UStr::new(&[]) }; __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedShapingResult {
    pub cluster_advance: f64,
    pub run_advance: f64,
    pub run_features: Vec<UString>,
    pub glyphs: Vec<RecordedGlyph>,
    pub decisions: Vec<RecordedShapingDecision>,
}

impl RecordedShapingResult {
    pub fn new(cluster_advance: f64, run_advance: f64, run_features: Vec<UString>, glyphs: Vec<RecordedGlyph>, decisions: Vec<RecordedShapingDecision>) -> Self {
        Self {
            cluster_advance,
            run_advance,
            run_features,
            glyphs,
            decisions,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RecordedShapingResult(")); __s += &(UString::from("clusterAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.cluster_advance)); __s += &(UString::from(", ")); __s += &(UString::from("runAdvance=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.run_advance)); __s += &(UString::from(", ")); __s += &(UString::from("runFeatures=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.run_features).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("glyphs=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.glyphs).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("decisions=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decisions).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricsEvidenceKey {
    pub font_key: UString,
    pub font_size: f64,
    pub role: UString,
    pub locale: UString,
    pub font_families: Vec<UString>,
    pub font_weight: u32,
    pub italic: bool,
    pub face_selection_text: UString,
}

impl MetricsEvidenceKey {
    pub fn new(font_key: &UStr, font_size: f64, role: &UStr, locale: &UStr, font_families: Vec<UString>, font_weight: u32, italic: bool, face_selection_text: &UStr) -> Self {
        Self {
            font_key: font_key.to_ustring(),
            font_size,
            role: role.to_ustring(),
            locale: locale.to_ustring(),
            font_families,
            font_weight,
            italic,
            face_selection_text: face_selection_text.to_ustring(),
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("MetricsEvidenceKey(")); __s += &(UString::from("fontKey=")); __s += (self.font_key).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontSize=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.font_size)); __s += &(UString::from(", ")); __s += &(UString::from("role=")); __s += (self.role).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("locale=")); __s += (self.locale).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontFamilies=")); __s += UString::from(format!("{}", {
        let mut out = String::new();
        out.push('[');
        let arr = (self.font_families).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i]);
            i += 1;
        }
        out.push(']');
        out
    }).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("fontWeight=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(self.font_weight)).as_str())); __s += &(UString::from(", ")); __s += &(UString::from("italic=")); __s += UString::from(format!("{}", (self.italic).to_string()).as_str()).as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("faceSelectionText=")); __s += (self.face_selection_text).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, Copy)]
pub struct MetricsEvidenceFns;

impl MetricsEvidenceFns {
    pub fn metrics_evidence_fns_font_metrics_request_to_evidence_key(request: FontMetricsRequest) -> MetricsEvidenceKey {
        return MetricsEvidenceKey::new((request.font_key).to_ustring().as_ustr(), request.font_size, UString::from(request.role.name()).as_ustr(), (request.locale).to_ustring().as_ustr(), (request.font_families).clone(), request.font_weight, request.italic, (request.face_selection_text).to_ustring().as_ustr());
    }

    pub fn metrics_evidence_fns_metrics_result(evidence: ShapingEvidence, key: MetricsEvidenceKey) -> Option<RecordedFontMetrics> {
        for i in 0..match u32::try_from(evidence.metrics.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            if MetricsEvidenceFns::metrics_evidence_fns_metrics_key_equals(((evidence.metrics[usize::try_from(i).unwrap_or(0)]).clone().key).clone(), (key).clone()) {
                return Some((((evidence.metrics[usize::try_from(i).unwrap_or(0)]).clone().result).clone()).clone());
            }
        }
        return None;
    }

    pub fn metrics_evidence_fns_metrics_key_equals(a: MetricsEvidenceKey, b: MetricsEvidenceKey) -> bool {
        if a == b {
            return true;
        }
        if a.font_key.to_ustring() != (b.font_key).to_ustring() {
            return false;
        }
        if a.font_size != b.font_size {
            return false;
        }
        if a.role.to_ustring() != (b.role).to_ustring() {
            return false;
        }
        if a.locale.to_ustring() != (b.locale).to_ustring() {
            return false;
        }
        if !ShapingEvidenceFns::shaping_evidence_fns_string_array_equals(&a.font_families, &b.font_families) {
            return false;
        }
        if a.font_weight != b.font_weight {
            return false;
        }
        if a.italic != b.italic {
            return false;
        }
        if a.face_selection_text.to_ustring() != (b.face_selection_text).to_ustring() {
            return false;
        }
        return true;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordedFontMetrics {
    pub ascent: f64,
    pub descent: f64,
    pub leading: f64,
    pub source: UString,
    pub typo_ascent: Option<f64>,
    pub typo_descent: Option<f64>,
}

impl RecordedFontMetrics {
    pub fn new(ascent: f64, descent: f64, leading: f64, source: &UStr, typo_ascent: Option<f64>, typo_descent: Option<f64>) -> Self {
        Self {
            ascent,
            descent,
            leading,
            source: source.to_ustring(),
            typo_ascent,
            typo_descent,
        }
    }

    pub fn to_raw_font_metrics(&self) -> Result<RawFontMetrics, TextRangeError> {
        let mut source_value: Option<FontMetricSource> = None;
        if self.source.to_ustring() == UString::from("RawTables") {
            source_value = Some(FontMetricSource::RawTables);
        } else {
            if self.source.to_ustring() == UString::from("OpenTypeBase") {
                source_value = Some(FontMetricSource::OpenTypeBase);
            } else {
                if self.source.to_ustring() == UString::from("GlyphSampling") {
                    source_value = Some(FontMetricSource::GlyphSampling);
                } else {
                    if self.source.to_ustring() == UString::from("ManualOverride") {
                        source_value = Some(FontMetricSource::ManualOverride);
                    } else {
                        if self.source.to_ustring() == UString::from("SynthesizedIdeographicBox") {
                            source_value = Some(FontMetricSource::SynthesizedIdeographicBox);
                        }
                    }
                }
            }
        }
        if source_value == None {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("Unknown font metric source: ")); __s += (self.source).to_ustring().as_ustr(); __s }).as_str()) });
        }
        return Ok(RawFontMetrics::new(self.ascent, self.descent, Some(self.leading), source_value, self.typo_ascent, self.typo_descent));
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("RecordedFontMetrics(")); __s += &(UString::from("ascent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.ascent)); __s += &(UString::from(", ")); __s += &(UString::from("descent=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.descent)); __s += &(UString::from(", ")); __s += &(UString::from("leading=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(self.leading)); __s += &(UString::from(", ")); __s += &(UString::from("source=")); __s += (self.source).to_ustring().as_ustr(); __s += &(UString::from(", ")); __s += &(UString::from("typoAscent=")); __s += &(match self.typo_ascent { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(", ")); __s += &(UString::from("typoDescent=")); __s += &(match self.typo_descent { Some(v) => UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(v)).as_str()), None => UString::from("null") }); __s += &(UString::from(")")); __s }).as_str());
    }
}

#[derive(Clone, PartialEq)]
pub struct RecordedEvidenceTextShaper {
    pub(crate) evidence: ShapingEvidence,
}

impl RecordedEvidenceTextShaper {
    pub fn new(evidence: ShapingEvidence) -> Self {
        Self {
            evidence,
        }
    }

    pub fn shape(&self, input: ShapingInput) -> Result<ShapingResult, TextRangeError> {
        let key = ShapingEvidenceFns::shaping_evidence_fns_shaping_input_to_evidence_key((input).clone());
        let recorded = ShapingEvidenceFns::shaping_evidence_fns_shaping_result((self.evidence).clone(), (key).clone());
        if recorded.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("No recorded shaping evidence for ")); __s += UString::from(format!("{}", key.to_string()).as_str()).as_ustr(); __s += &(UString::from(" — re-record on the JVM with ")); __s += &(UString::from("TIQIAN_RECORD_SHAPING=1 ./gradlew :engine:jvmTest --tests '*ShapingEvidenceRecorder*'")); __s }).as_str()) });
        }
        let source_text = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        let font_key = (((input.font_decision).clone().candidate).clone().key).to_ustring();
        let cluster = Cluster::new((input.range).clone(), source_text.as_ustr(), font_key.as_ustr(), (recorded).as_ref().unwrap().cluster_advance, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let mut glyphs: Vec<Glyph> = vec![];
        {
            let _g1 = ((recorded).as_ref().unwrap().glyphs).clone().clone();
            for g in &_g1 {
                glyphs.push(Glyph::new(g.id, (input.range).clone(), g.advance, Some(g.x), Some(g.y), None, (g.bounds).clone(), g.halt_advance, g.halt_placement_x));
            }
        }
        let run = GlyphRun::new((input.range).clone(), font_key.as_ustr(), glyphs.to_vec(), (recorded).as_ref().unwrap().run_advance, Some(((recorded).as_ref().unwrap().run_features).clone()));
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        {
            let _g1 = ((recorded).as_ref().unwrap().decisions).clone().clone();
            for d in &_g1 {
                decisions.push(ShapingDecisionInfo::new((input.range).clone(), source_text.as_ustr(), (input.display_text).to_ustring().as_ustr(), font_key.as_ustr(), d.glyph_count, d.advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), (d.resolved_face).clone().clone(), (d.script).clone().clone(), (d.language).clone().clone(), (d.strategy).clone().clone(), (d.feature_evidence).clone().clone(), (d.capability_issue).clone().clone()));
            }
        }
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(run).clone()].to_vec(), Some((decisions).clone())));
    }
}

impl ITextShaper for RecordedEvidenceTextShaper {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.test.ShapingEvidence.RecordedEvidenceTextShaper"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(self.clone())
    }

    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        let key = ShapingEvidenceFns::shaping_evidence_fns_shaping_input_to_evidence_key((input).clone());
        let recorded = ShapingEvidenceFns::shaping_evidence_fns_shaping_result((self.evidence).clone(), (key).clone());
        if recorded.is_none() {
            return Err(TextShaperShapeFault::TextRangeErrorFault(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("No recorded shaping evidence for ")); __s += UString::from(format!("{}", key.to_string()).as_str()).as_ustr(); __s += &(UString::from(" — re-record on the JVM with ")); __s += &(UString::from("TIQIAN_RECORD_SHAPING=1 ./gradlew :engine:jvmTest --tests '*ShapingEvidenceRecorder*'")); __s }).as_str()) }));
        }
        let source_text = u_string::substring(&(input.text).to_ustring(), i32::from_ne_bytes((((input.range).clone().start) as i32).to_ne_bytes()), i32::from_ne_bytes((((input.range).clone().end) as i32).to_ne_bytes()));
        let font_key = (((input.font_decision).clone().candidate).clone().key).to_ustring();
        let cluster = Cluster::new((input.range).clone(), source_text.as_ustr(), font_key.as_ustr(), (recorded).as_ref().unwrap().cluster_advance, Some((input.display_text).to_ustring()), Some(0.0), Some(0.0), Some(0.0));
        let mut glyphs: Vec<Glyph> = vec![];
        {
            let _g1 = ((recorded).as_ref().unwrap().glyphs).clone().clone();
            for g in &_g1 {
                glyphs.push(Glyph::new(g.id, (input.range).clone(), g.advance, Some(g.x), Some(g.y), None, (g.bounds).clone(), g.halt_advance, g.halt_placement_x));
            }
        }
        let run = GlyphRun::new((input.range).clone(), font_key.as_ustr(), glyphs.to_vec(), (recorded).as_ref().unwrap().run_advance, Some(((recorded).as_ref().unwrap().run_features).clone()));
        let mut decisions: Vec<ShapingDecisionInfo> = vec![];
        {
            let _g1 = ((recorded).as_ref().unwrap().decisions).clone().clone();
            for d in &_g1 {
                decisions.push(ShapingDecisionInfo::new((input.range).clone(), source_text.as_ustr(), (input.display_text).to_ustring().as_ustr(), font_key.as_ustr(), d.glyph_count, d.advance, (d.source).to_ustring().as_ustr(), (d.reason).to_ustring().as_ustr(), Some(d.glyphs_without_ink_bounds), Some(d.missing_glyphs), (d.resolved_face).clone().clone(), (d.script).clone().clone(), (d.language).clone().clone(), (d.strategy).clone().clone(), (d.feature_evidence).clone().clone(), (d.capability_issue).clone().clone()));
            }
        }
        return Ok(ShapingResult::new(vec![(cluster).clone()].to_vec(), vec![(run).clone()].to_vec(), Some((decisions).clone())));
    }
}

#[derive(Clone, PartialEq)]
pub struct RecordedEvidenceFontMetricsResolver {
    pub(crate) evidence: ShapingEvidence,
}

impl RecordedEvidenceFontMetricsResolver {
    pub fn new(evidence: ShapingEvidence) -> Self {
        Self {
            evidence,
        }
    }

    pub fn resolve(&self, request: FontMetricsRequest) -> Result<RawFontMetrics, TextRangeError> {
        let key = MetricsEvidenceFns::metrics_evidence_fns_font_metrics_request_to_evidence_key((request).clone());
        let recorded = MetricsEvidenceFns::metrics_evidence_fns_metrics_result((self.evidence).clone(), (key).clone());
        if recorded.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("No recorded font metrics evidence for ")); __s += UString::from(format!("{}", key.to_string()).as_str()).as_ustr(); __s += &(UString::from(" — re-record on the JVM with ")); __s += &(UString::from("TIQIAN_RECORD_SHAPING=1 ./gradlew :engine:jvmTest --tests '*ShapingEvidenceRecorder*'")); __s }).as_str()) });
        }
        return Ok(recorded.as_ref().unwrap().to_raw_font_metrics()?);
    }
}

impl FontMetricsResolver for RecordedEvidenceFontMetricsResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.test.ShapingEvidence.RecordedEvidenceFontMetricsResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontMetricsResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, request: FontMetricsRequest) -> Result<RawFontMetrics, TextRangeError> {
        let key = MetricsEvidenceFns::metrics_evidence_fns_font_metrics_request_to_evidence_key((request).clone());
        let recorded = MetricsEvidenceFns::metrics_evidence_fns_metrics_result((self.evidence).clone(), (key).clone());
        if recorded.is_none() {
            return Err(TextRangeError::Message { text: UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("No recorded font metrics evidence for ")); __s += UString::from(format!("{}", key.to_string()).as_str()).as_ustr(); __s += &(UString::from(" — re-record on the JVM with ")); __s += &(UString::from("TIQIAN_RECORD_SHAPING=1 ./gradlew :engine:jvmTest --tests '*ShapingEvidenceRecorder*'")); __s }).as_str()) });
        }
        return Ok(recorded.as_ref().unwrap().to_raw_font_metrics()?);
    }
}
