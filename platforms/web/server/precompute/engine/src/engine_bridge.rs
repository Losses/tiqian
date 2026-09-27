//! Session-backed host components for the generated Rust engine (ADR 0050
//! Slice C, cutover to the boring engine-rust bundle).
//!
//! The generated `ExplainableStubParagraphLayoutEngine` receives shaping and
//! font metrics through the generated trait objects (`ITextShaper`,
//! `FontMetricsResolver`). This module presents a [`FontSession`] and its
//! capture window as those components, the direct-Rust successor of the
//! former C ABI vtable bridge, and runs one paragraph request through
//! `tiqian::engine::Engine`. The engine `LayoutResult` goes through the
//! generated prepared-paragraph JSON emitter, and the existing plan reader
//! parses that JSON, so callers keep receiving the same [`Plan`] the packed
//! ABI path produced.
//!
//! The component mapping mirrors the Kotlin `NativeFontBackendTextShaper`
//! and `NativeFontBackendFontMetricsResolver` of the nativeMain shaper: one
//! cluster, one glyph run and one shaping decision per backend shape call,
//! and the five metric doubles mapped onto `RawFontMetrics`.

use tiqian::engine::{Engine, EngineComponents};
use tiqian::NamedError;
use tiqian_engine_gen::org::tiqian::core::cluster::Cluster;
use tiqian_engine_gen::org::tiqian::core::glyph::Glyph;
use tiqian_engine_gen::org::tiqian::core::glyph_run::GlyphRun;
use tiqian_engine_gen::org::tiqian::core::ic::Ic;
use tiqian_engine_gen::org::tiqian::core::illegal_state_exception::IllegalStateException;
use tiqian_engine_gen::org::tiqian::core::inline_box_outer_spacing::InlineBoxOuterSpacing;
use tiqian_engine_gen::org::tiqian::core::inline_box_span::InlineBoxSpan;
use tiqian_engine_gen::org::tiqian::core::layout_constraints::LayoutConstraints;
use tiqian_engine_gen::org::tiqian::core::layout_input::LayoutInput;
use tiqian_engine_gen::org::tiqian::core::line_break_policy::LineBreakPolicy;
use tiqian_engine_gen::org::tiqian::core::line_break_span::LineBreakSpan;
use tiqian_engine_gen::org::tiqian::core::line_length_grid::LineLengthGrid;
use tiqian_engine_gen::org::tiqian::core::paragraph_style::ParagraphStyle;
use tiqian_engine_gen::org::tiqian::core::rect::Rect;
use tiqian_engine_gen::org::tiqian::core::shaping_decision_info::ShapingDecisionInfo;
use tiqian_engine_gen::org::tiqian::core::text_range::TextRange;
use tiqian_engine_gen::org::tiqian::core::text_span::TextSpan;
use tiqian_engine_gen::org::tiqian::core::text_style::TextStyle;
use tiqian_engine_gen::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use tiqian_engine_gen::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use tiqian_engine_gen::org::tiqian::font::font_metric_source::FontMetricSource;
use tiqian_engine_gen::org::tiqian::font::font_metrics::{FontMetricsRequest, FontMetricsResolver};
use tiqian_engine_gen::org::tiqian::font::raw_font_metrics::RawFontMetrics;
use tiqian_engine_gen::org::tiqian::layout::prepared_paragraph::PreparedParagraphFns;
use tiqian_engine_gen::org::tiqian::shaping::text_shaper::{
    ITextShaper, ShapingInput, ShapingResult, ShapingSource, TextShaperShapeFault,
};

use crate::paragraph::{InlineBoxOuterSpacingCode, ParagraphRequest};
use crate::session::{CaptureEvidence, FontSession, MetricsInput, ShapeInput};

/// Presents one font session and its capture window to the engine as the
/// `ITextShaper` and `FontMetricsResolver` implementations.
///
/// The struct erases the borrows into raw pointers because the generated
/// trait objects require a `_static` type. The obligations that keep this
/// sound:
///
/// - `precompute_paragraph` holds the `&FontSession` and the
///   `&mut CaptureEvidence` borrows for the whole engine call, so both
///   pointers stay valid while the engine can reach them.
/// - The engine runs its layout synchronously on the calling thread, so all
///   access through these pointers happens on one thread, sequentially:
///   never concurrently, never re-entrantly.
///
/// The invariants are the direct-Rust form of the former thread-local lend
/// slot; obligations are recorded in docs/rust-unsafe-inventory.md, section
/// "engine_bridge.rs".
struct SessionBackend {
    session: *const FontSession,
    evidence: *mut CaptureEvidence,
}

// SAFETY: the raw pointers target data that outlives the engine call (see
// the obligation list above); the engine drives every access on the calling
// thread, so the `Send`/`Sync` impls only satisfy the generated trait
// bounds, they never enable cross-thread use.
unsafe impl Send for SessionBackend {}
unsafe impl Sync for SessionBackend {}

impl SessionBackend {
    /// # Safety
    /// Both pointers must stay valid, and the pointees exclusively owned by
    /// this call, for as long as the built engine lives.
    /// `precompute_paragraph` guarantees both.
    unsafe fn new(session: &FontSession, evidence: &mut CaptureEvidence) -> Self {
        Self {
            session: std::ptr::from_ref(session),
            evidence: std::ptr::from_mut(evidence),
        }
    }

    fn with_access<T>(
        &self,
        call: impl FnOnce(&FontSession, &mut CaptureEvidence) -> T,
    ) -> T {
        // SAFETY: obligation list on `SessionBackend`: the borrows held by
        // `precompute_paragraph` cover the engine call, and the engine
        // invokes the components sequentially on its calling thread.
        call(unsafe { &*self.session }, unsafe { &mut *self.evidence })
    }

    /// The `shape` body, kept free of the fault mapping so the error strings
    /// read the same as the former vtable error slot contents.
    fn shape_record(&self, input: &ShapingInput) -> Result<ShapingResult, String> {
        let shaped = self.with_access(|session, evidence| {
            let source_text = utf16_substring(&input.text, input.range.start, input.range.end);
            let input = ShapeInput {
                display_text: &input.display_text,
                font_families: &input.style.font_families,
                font_size: input.style.font_size,
                font_weight: f64::from(input.style.font_weight),
                italic: input.style.italic,
                locale: &input.style.locale,
                role: Some(input.font_decision.role.name()),
                source_text: Some(&source_text),
            };
            session
                .shape_into(evidence, &input)
                .map(|record| (record, source_text))
        })?;
        let (record, source_text) = shaped;
        let range = input.range.clone();
        let glyphs: Vec<Glyph> = record
            .glyphs
            .iter()
            .map(|glyph| {
                Glyph::new(
                    glyph.id,
                    range.clone(),
                    glyph.advance,
                    Some(glyph.x),
                    Some(glyph.y),
                    None,
                    glyph
                        .bounds
                        .map(|bounds| Rect::new(bounds[0], bounds[1], bounds[2], bounds[3])),
                    None,
                    None,
                )
            })
            .collect();
        let advance = record.advance;
        let key = input.font_decision.candidate.key.clone();
        let features = record.features.clone();
        let cluster = Cluster::new(
            range.clone(),
            &source_text,
            &key,
            advance,
            Some(input.display_text.clone()),
            None,
            None,
            None,
        );
        let run = GlyphRun::new(range, &key, glyphs, advance, Some(features.clone()));
        // The reason string stays byte-identical to the former native backend
        // decision so engine dumps remain diffable across the cutover.
        let reason = format!(
            "SharedHarfBuzzSession:face={}; instance={}; current-segment-context; features={}; unsafeToBreakGlyphs={}",
            record.face_id,
            record.font_instance_id,
            if features.is_empty() {
                "default".to_string()
            } else {
                features.join(",")
            },
            record.unsafe_break_count,
        );
        let glyph_count = u32::try_from(run.glyphs.len()).unwrap_or(u32::MAX);
        let without_ink_bounds = u32::try_from(
            run.glyphs.iter().filter(|glyph| glyph.bounds.is_none()).count(),
        )
        .unwrap_or(u32::MAX);
        let missing_glyphs =
            u32::try_from(run.glyphs.iter().filter(|glyph| glyph.id == 0).count())
                .unwrap_or(u32::MAX);
        let decision = ShapingDecisionInfo::new(
            input.range.clone(),
            &source_text,
            &input.display_text,
            &key,
            glyph_count,
            advance,
            ShapingSource::HarfBuzz.name(),
            &reason,
            Some(without_ink_bounds),
            Some(missing_glyphs),
            Some(record.face_id.clone()),
            Some(record.script.clone()),
            Some(input.style.locale.clone()),
            None,
            (!features.is_empty()).then(|| features.join(",")),
            None,
        );
        Ok(ShapingResult::new(
            vec![cluster],
            vec![run],
            Some(vec![decision]),
        ))
    }

    fn resolve_metrics(&self, request: &FontMetricsRequest) -> Result<[f64; 5], String> {
        self.with_access(|session, evidence| {
            let input = MetricsInput {
                font_families: &request.font_families,
                font_size: request.font_size,
                font_weight: f64::from(request.font_weight),
                italic: request.italic,
                role: Some(request.role.name()),
                face_selection_text: Some(&request.face_selection_text),
            };
            session.metrics_into(evidence, &input)
        })
    }
}

impl ITextShaper for SessionBackend {
    fn __haxe_type_name(&self) -> &'static str {
        "tiqian_precompute.engine_bridge.SessionBackend"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ITextShaper> {
        Box::new(SessionBackend {
            session: self.session,
            evidence: self.evidence,
        })
    }
    fn shape(&mut self, input: ShapingInput) -> Result<ShapingResult, TextShaperShapeFault> {
        self.shape_record(&input).map_err(|message| {
            TextShaperShapeFault::IllegalStateExceptionFault(IllegalStateException::new(&message))
        })
    }
}

impl FontMetricsResolver for SessionBackend {
    fn __haxe_type_name(&self) -> &'static str {
        "tiqian_precompute.engine_bridge.SessionBackend"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn FontMetricsResolver> {
        Box::new(SessionBackend {
            session: self.session,
            evidence: self.evidence,
        })
    }
    fn resolve(&self, request: FontMetricsRequest) -> Result<RawFontMetrics, TextRangeError> {
        let values = self
            .resolve_metrics(&request)
            .map_err(|message| TextRangeError::Message { text: message })?;
        let [ascent, descent, leading, typo_ascent, typo_descent] = values;
        Ok(RawFontMetrics::new(
            ascent,
            descent,
            Some(leading),
            Some(FontMetricSource::RawTables),
            typo_ascent.is_finite().then_some(typo_ascent),
            typo_descent.is_finite().then_some(typo_descent),
        ))
    }
}
/// Adapts the lane request into the generated engine input. The generated
/// engine is the f64 bundle, so the values keep full f64 precision here; the
/// i32 to u32 range casts keep the check outcomes of the validation, which
/// ran on the i32 values the caller passed.
fn layout_input(request: &ParagraphRequest) -> Result<LayoutInput, NamedError> {
    fn text_range(start: i32, end: i32) -> Result<TextRange, NamedError> {
        TextRange::new(start as u32, end as u32).map_err(|error| NamedError(error.to_string()))
    }
    let text_style = TextStyle::new(
        Some(request.font_families.clone()),
        Some(request.font_size_px),
        Some(request.locale.clone()),
        Some(request.font_weight as u32),
        Some(request.italic),
        Some(0.0),
        None,
    );
    let spans = request
        .text_spans
        .iter()
        .map(|span| {
            Ok(TextSpan::new(
                text_range(span.start, span.end)?,
                TextStyle::new(
                    Some(span.families.clone()),
                    Some(span.font_size_px),
                    None,
                    Some(span.font_weight as u32),
                    Some(span.italic),
                    Some(span.baseline_shift),
                    None,
                ),
            ))
        })
        .collect::<Result<Vec<_>, NamedError>>()?;
    let boundaries = request
        .source_boundaries
        .iter()
        .map(|value| *value as u32)
        .collect();
    let line_break_spans = request
        .line_break_spans
        .iter()
        .map(|span| {
            Ok(LineBreakSpan::new(
                text_range(span.start, span.end)?,
                LineBreakPolicy::ProgressiveTechnical,
            ))
        })
        .collect::<Result<Vec<_>, NamedError>>()?;
    let content = TiqianTextContent::new(
        &request.text,
        Some(spans),
        Some(boundaries),
        Some(line_break_spans),
        None,
    );
    let constraints = LayoutConstraints::new(request.max_width_px, None, None)
        .map_err(|error| NamedError(error.to_string()))?;
    let paragraph_style = ParagraphStyle::new(
        None,
        None,
        Some(request.line_height_px),
        Some(Ic::new(request.first_line_indent_ic)),
        None,
        None,
        Some(LineLengthGrid::new(Some(request.line_length_grid_enabled), None)),
        None,
        None,
        None,
    );
    let inline_boxes = request
        .inline_boxes
        .iter()
        .map(|inline_box| {
            Ok(InlineBoxSpan::new(
                text_range(inline_box.start, inline_box.end)?,
                Some(inline_box.inline_start),
                Some(inline_box.inline_end),
                Some(match inline_box.outer_spacing {
                    InlineBoxOuterSpacingCode::Narrow => InlineBoxOuterSpacing::Narrow,
                    InlineBoxOuterSpacingCode::Source => InlineBoxOuterSpacing::Source,
                }),
            ))
        })
        .collect::<Result<Vec<_>, NamedError>>()?;
    Ok(LayoutInput::new(
        content,
        Some(text_style),
        Some(paragraph_style),
        constraints,
        None,
        None,
        None,
        Some(inline_boxes),
        None,
    ))
}

/// Runs one paragraph request through the generated engine with `session`
/// and its capture window as the shaping and metrics components. Both stay
/// borrowed for the duration of the call. Errors are the named validation
/// issues of the request and the engine; the production path returns the
/// prepared-paragraph JSON read back into [`crate::plan::Plan`].
pub fn precompute_paragraph(
    session: &FontSession,
    evidence: &mut CaptureEvidence,
    request: &ParagraphRequest,
) -> Result<crate::plan::Plan, String> {
    request.validate().map_err(|error| error.0)?;
    // SAFETY: `precompute_paragraph` holds both borrows across the engine
    // call; see the obligation list on `SessionBackend`.
    let mut engine = Engine::new(EngineComponents {
        text_shaper: Some(Box::new(unsafe { SessionBackend::new(session, evidence) })),
        font_metrics_resolver: Some(Box::new(unsafe { SessionBackend::new(session, evidence) })),
        ..EngineComponents::default()
    })
    .map_err(|error| error.0)?;
    let input = layout_input(request).map_err(|error| error.0)?;
    let result = engine.layout(input).map_err(|error| error.0)?;
    let json =
        PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json(result, true)
            .map_err(|fault| format!("{fault:?}"))?;
    crate::plan::Plan::from_json_str(&json).map_err(|error| error.0)
}

/// Kotlin `String.substring(start, end)`: UTF-16 code-unit offsets. A char
/// is kept when its UTF-16 units fall fully inside the range; the char that
/// a surrogate-split boundary cuts joins when its first unit sits inside.
fn utf16_substring(text: &str, start: u32, end: u32) -> String {
    let mut out = String::new();
    let mut position = 0u32;
    for character in text.chars() {
        let units = if character as u32 <= 0xffff { 1 } else { 2 };
        if position + units > start && position < end {
            out.push(character);
        }
        position += units;
    }
    out
}