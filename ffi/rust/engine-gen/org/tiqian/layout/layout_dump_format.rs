use crate::org::tiqian::clreq::clreq_profile::ClreqProfile;
use crate::org::tiqian::clreq::clreq_profile_resolver::ClreqProfileResolver;
use crate::org::tiqian::clreq::hanging_punctuation_style::HangingPunctuationStyle;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::clreq::kinsoku_mode::KinsokuMode;
use crate::org::tiqian::core::ic::Ic;
use crate::org::tiqian::core::inline_attachment::InlineAttachment;
use crate::org::tiqian::core::last_line_alignment::LastLineAlignment;
use crate::org::tiqian::core::layout_input::LayoutInput;
use crate::org::tiqian::core::layout_profile_id::LayoutProfileId;
use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::measure_adaptive_first_line_indent::MeasureAdaptiveFirstLineIndent;
use crate::org::tiqian::core::paragraph_style::ParagraphStyle;
use crate::org::tiqian::core::text_style::TextStyle;
use crate::org::tiqian::core::tiqian_text_content::TiqianTextContent;
use crate::org::tiqian::core::writing_mode::WritingMode;
use crate::org::tiqian::font::cjk_font_role_classifier::CjkFontRoleClassifier;
use crate::org::tiqian::font::font_metrics::FontMetricsResolver;
use crate::org::tiqian::font::font_metrics::ScriptAwareFontMetricsNormalizer;
use crate::org::tiqian::font::font_metrics::StubFontMetricsResolver;
use crate::org::tiqian::layout::justifier::Justifier;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_breaker::GreedyLineBreaker;
use crate::org::tiqian::layout::line_breaker::LineBreaker;
use crate::org::tiqian::layout::line_breaker::LookaheadLineBreaker;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::layout::paragraph_layout_engine::ExplainableStubParagraphLayoutEngine;
use crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutFallbackResolver;
use crate::org::tiqian::layout::punctuation_model::PunctuationAtomBuilder;
use crate::org::tiqian::layout::punctuation_model::PunctuationSpacingCompressor;
use crate::org::tiqian::layout::quote_pair_analyzer::QuotePairAnalyzer;
use crate::org::tiqian::layout::width_independent_annotation_cache::LruWidthIndependentAnnotationCache;
use crate::org::tiqian::linebreak::english_hyphenation::EnglishHyphenation;
use crate::org::tiqian::linebreak::hyphenator::Hyphenator;
use crate::org::tiqian::linebreak::hyphenator::NoHyphenator;
use crate::org::tiqian::shaping::text_shaper::ExplainableStubTextShaper;
use crate::org::tiqian::shaping::text_shaper::ITextShaper;
use crate::org::tiqian::test::layout_fixtures::LayoutFixture;
use crate::runtime::fp_helper::FPHelper;
use crate::runtime::u_string;
use std::fmt::Write;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutDumpFormatLayoutFixtureDumpFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}

impl From<LayoutDumpFormatLayoutFixtureDumpFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: LayoutDumpFormatLayoutFixtureDumpFault) -> Self {
        match value {
            LayoutDumpFormatLayoutFixtureDumpFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutDumpFormatLayoutFixtureDumpFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: LayoutDumpFormatLayoutFixtureDumpFault) -> Self {
        match value {
            LayoutDumpFormatLayoutFixtureDumpFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutDumpFormatLayoutFixtureDumpFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault {
    fn from(value: LayoutDumpFormatLayoutFixtureDumpFault) -> Self {
        match value {
            LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineNewFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<LayoutDumpFormatLayoutFixtureDumpFault> for crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault {
    fn from(value: LayoutDumpFormatLayoutFixtureDumpFault) -> Self {
        match value {
            LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for LayoutDumpFormatLayoutFixtureDumpFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        LayoutDumpFormatLayoutFixtureDumpFault::TextRangeErrorFault(value)
    }
}

impl From<crate::std::u_string_exception::UStringFault> for LayoutDumpFormatLayoutFixtureDumpFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        LayoutDumpFormatLayoutFixtureDumpFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault> for LayoutDumpFormatLayoutFixtureDumpFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault) -> Self {
        LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineNewFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault> for LayoutDumpFormatLayoutFixtureDumpFault {
    fn from(value: crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault) -> Self {
        LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct LayoutDumpFormat;

impl LayoutDumpFormat {
    pub(crate) fn layout_dump_format_join_ints(a: &[u32]) -> String {
        let mut b: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.push(crate::runtime::int_text::IntText::int_text(a[usize::try_from(i).unwrap_or(0)]));
        }
        return if u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { "-".to_string() } else { { let joined = b; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(&(",")); } let _ = write!(out, "{}",
joined[index]); index += 1; } out }.to_string() };
    }

    pub(crate) fn layout_dump_format_join_floats(a: &[f64]) -> String {
        let mut b: Vec<String> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.push(LayoutDumpFormat::layout_dump_format_dump_fmt(a[usize::try_from(i).unwrap_or(0)]));
        }
        return if u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { "-".to_string() } else { { let joined1 = b; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(&(",")); } let _ = write!(out,
"{}", joined1[index1]); index1 += 1; } out }.to_string() };
    }

    pub fn layout_dump_format_dump_fmt(value: f64) -> String {
        if value.is_nan() {
            return "NaN".to_string();
        }
        if !(value).is_finite() {
            return if value > (0 as f64) { "Infinity".to_string() } else { "-Infinity".to_string() };
        }
        let negative = u32::from_ne_bytes((FPHelper::float_to_i32(value)).to_ne_bytes()) > 2147483647;
        let magnitude = u32::from_ne_bytes((match f64::from(f64::floor((value).abs() * format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0) + 0.5f64)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 =>
i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match
((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }).to_ne_bytes());
        return format!("{}{}{}{}",
            (if negative { "-".to_string() } else { "".to_string() }),
            crate::runtime::int_text::IntText::int_text(u32::from_ne_bytes((match f64::from(f64::floor(format!("{}", (i32::from_ne_bytes((magnitude).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) / format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0) as f64)) { v if v.is_nan() =>
0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) |
4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0)
}.to_ne_bytes()) }).to_ne_bytes())),
            ".",
            crate::runtime::int_text::IntText::int_text(u32::from_ne_bytes((i32::from_ne_bytes((magnitude).to_ne_bytes()) % 10i32).to_ne_bytes()))
        );
    }

    pub fn layout_dump_format_escape_dump_text(value: &str) -> String {
    let __units = u_string::units(&value);
    let __count = u_string::unit_count(&value);
        let mut out_b = String::new();
        for i in 0..match u32::try_from(u_string::unit_count(&(value))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units, i).unwrap_or(0);
            if c == 10 {
                out_b += &("\\n");
            } else {
                if c == 13 {
                    out_b += &("\\r");
                } else {
                    if c == 11 {
                        out_b += &("\\v");
                    } else {
                        if c == 12 {
                            out_b += &("\\f");
                        } else {
                            if c == 133 {
                                out_b += &("\\u0085");
                            } else {
                                if c == 8232 {
                                    out_b += &("\\u2028");
                                } else {
                                    if c == 8233 {
                                        out_b += &("\\u2029");
                                    } else {
                                        if c == 8203 {
                                            out_b += &("\\u200B");
                                        } else {
                                            let c = c;
                                            out_b += &(if c > 0xFFFF { String::from_utf16(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]).unwrap() } else { String::from_utf16_lossy(&[(c) as u16]) });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return out_b;
    }

    pub fn layout_dump_format_layout_fixture_dump(f: LayoutFixture, text_shaper: Option<Box<dyn ITextShaper>>, font_metrics_resolver: Option<Box<dyn FontMetricsResolver>>) -> Result<String, LayoutDumpFormatLayoutFixtureDumpFault> {
        let mut out_b = String::new();
        out_b += &("fixture: ");
        {
            let x = (f.id).to_string().clone();
            out_b += &(x.to_string());
        }
        out_b += &(concat!("\n",
"text: "));
        {
            let x = LayoutDumpFormat::layout_dump_format_escape_dump_text((f.text).to_string().as_str());
            out_b += &(x.to_string());
        }
        out_b += &(concat!("\n",
"maxWidth: "));
        {
            let x = LayoutDumpFormat::layout_dump_format_dump_fmt((f.constraints).clone().max_width);
            out_b += &(x.to_string());
        }
        out_b += &(concat!("\n",
""));
        let shaper: Box<dyn ITextShaper> = match &(text_shaper) { None => Box::new(ExplainableStubTextShaper::new()), Some(__option) => (*__option).clone() };
        for n in 0..3 {
            let breaker: Box<dyn LineBreaker> = if n == 0 { Box::new(GreedyLineBreaker::new(None, None, None, None)) } else { if n == 1 { Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2),
Some(10), Some(20), Some(12.0))) } else { Box::new(ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).map_err(|e|
LayoutDumpFormatLayoutFixtureDumpFault::TextRangeErrorFault(e))?) } };
            let hyphenator: Box<dyn Hyphenator> = if f.use_english_hyphenation { EnglishHyphenation::english_hyphenation_en_us().map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::UStringFaultFault(e))? } else { Box::new(NoHyphenator::new()) };
            let resolver: Option<Box<dyn ClreqProfileResolver>> = if f.pin_basic_no_hang { Some(Box::new(BasicProfileResolver::new()) as Box<dyn ClreqProfileResolver>) } else { None };
            let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some("cjk-primary".to_string()), Some("latin-primary".to_string()), Some("symbol-fallback".to_string()))?)),
resolver, match &(font_metrics_resolver) { None => Some(Box::new(StubFontMetricsResolver::new()) as Box<dyn FontMetricsResolver>), Some(__option3) => Some((*__option3).clone()) }, Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None,
None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some((breaker).clone()), Some(Justifier::new(Some(0.5), Some(0.25))), Some((shaper).clone()), Some((hyphenator).clone()),
Some(Box::new(LruWidthIndependentAnnotationCache::new(512)))).map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineNewFaultFault(e))?;
            let indent = match &(f.first_line_indent_em) { None => None, Some(__option11) => Some(Ic(*__option11)) };
            let input = LayoutInput::new(TiqianTextContent::new((f.text).to_string().as_str(), Some(vec![]), Some(vec![]), Some((f.line_break_spans).clone()), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some("zh-Hans".to_string()), Some(400), Some(false),
Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), f.line_height, (indent).clone(), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))),
Some((f.line_length_grid).clone()), Some(f.ruby_line_height_mode), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), (f.constraints).clone(),
Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some((f.decorations).clone()), Some((f.ruby_spans).clone()), Some(vec![]), Some(vec![]));
            {
                let x = LayoutDumpFormat::layout_dump_format_decision_dump(engine.layout((input).clone()).map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?, if n == 0 { "greedy".to_string() } else { if n == 1
{ "lookahead".to_string() } else { "paragraph-dp".to_string() }.to_string() }.as_str());
                out_b += &(x.to_string());
            }
        }
        return Ok(out_b);
    }

    pub fn layout_dump_format_decision_dump(r: LayoutResult, label: &str) -> String {
        let mut o_b = String::new();
        let d = (r.debug).clone().clone();
        o_b += &("== ");
        o_b += &(label.to_string());
        o_b += &(concat!(" ==\n",
"size "));
        {
            let x = LayoutDumpFormat::layout_dump_format_dump_fmt((r.size).clone().width);
            o_b += &(x.to_string());
        }
        o_b += &("x");
        {
            let x = LayoutDumpFormat::layout_dump_format_dump_fmt((r.size).clone().height);
            o_b += &(x.to_string());
        }
        o_b += &(concat!("\n",
""));
        if match &(d.line_length_grid_decision) { Some(__option12) => __option12.enabled && (__option12.slack) > (0 as f64), None => false } {
            let g = d.line_length_grid_decision.clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "grid container=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().container_width),
            " measure=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().measure),
            "(",
            crate::runtime::int_text::IntText::int_text(g.as_ref().unwrap().cells),
            "字) slack=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().slack),
            " body=",
            (g.as_ref().unwrap().body_alignment).to_string(),
            "@",
            LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().body_offset),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        match &(d.first_line_indent_decision) {
            Some(__option13) => {
                if __option13.source.to_string() != "Explicit" {
                let x = (*__option13).clone();
                {
                    let x = format!("{}{}{}{}{}{}{}{}{}",
            "firstindent ",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.resolved_em),
            "字 measure=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.measure_em),
            "字 threshold=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.threshold_em),
            "字 ",
            (x.source).to_string(),
            concat!("\n",
"")
        );
                    o_b += &(x.to_string());
                }
                }
            }
            None => {
            }
        }
        match &(d.kinsoku_decision) {
            Some(__option14) => {
                let x = (*__option14).clone();
                {
                    let x = format!("{}{}{}{}{}{}{}{}{}",
            "kinsoku measure=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.measure_em),
            "字 level=",
            (x.level).to_string(),
            " hang=",
            (x.hanging).to_string(),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        for i in 0..match u32::try_from(d.contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "context-kinsoku ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " source='",
            LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_string().as_str()),
            "' cluster=",
            crate::runtime::int_text::IntText::int_text(x.cluster_index),
            " forbid=",
            (x.forbidden_position).to_string(),
            " reason=",
            (x.reason).to_string(),
            (match &(x.impossible_measure_fallback) { None => "".to_string(), Some(__option15) => format!("{}{}",
            " fallback=",
            (*__option15).clone()
        ).to_string() }),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.break_opportunity_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.break_opportunity_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}",
            "break-opportunity ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " source='",
            LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_string().as_str()),
            "' offsets=",
            LayoutDumpFormat::layout_dump_format_join_ints(&x.break_offsets),
            (match &(x.tier) { None => "".to_string(), Some(__option16) => format!("{}{}",
            " tier=",
            (*__option16).clone()
        ).to_string() }),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}",
            "tracking-eligibility ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " source='",
            LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_string().as_str()),
            "' reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.inline_object_punctuation_attachment_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.inline_object_punctuation_attachment_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "inline-object-punctuation ",
            crate::runtime::int_text::IntText::int_text((x.object_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.object_range).clone().end),
            " separator=",
            crate::runtime::int_text::IntText::int_text((x.separator_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.separator_range).clone().end),
            " punctuation=",
            crate::runtime::int_text::IntText::int_text((x.punctuation_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.punctuation_range).clone().end),
            " source='",
            LayoutDumpFormat::layout_dump_format_escape_dump_text((x.punctuation_text).to_string().as_str()),
            "' collapsed=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.collapsed_advance),
            " protected=",
            crate::runtime::int_text::IntText::int_text((x.protected_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.protected_range).clone().end),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let x = if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u32::try_from((d.line_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some((d.line_decisions[usize::try_from(i).unwrap_or(0)]).clone()) } else { None };
            let repair = if match &(x) { None => true, Some(__option18) => __option18.repair_decision.is_none() } { "-".to_string() } else { format!("{}{}{}{}{}{}",
            ((x.as_ref().unwrap().repair_decision).clone().as_ref().unwrap().kind).to_string(),
            "(",
            ((x.as_ref().unwrap().repair_decision).clone().as_ref().unwrap().reason_code).to_string(),
            " shrink=",
            LayoutDumpFormat::layout_dump_format_dump_fmt((x.as_ref().unwrap().repair_decision).clone().as_ref().unwrap().shrink),
            ")"
        ).to_string() };
            let mut candidates = "-".to_string();
            match &(x) {
                Some(__option19) => {
                    if i32::from_ne_bytes((u32::try_from((__option19.repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                    let mut a: Vec<String> = vec![];
                    for _g_index in 0..match u32::try_from(__option19.repair_candidates.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let z = (__option19.repair_candidates[usize::try_from(_g_index).unwrap_or(0)]).clone();
                        a.push(format!("{}{}",
            (z.kind).to_string(),
            (if z.accepted { "+".to_string() } else { "-".to_string() })
        ));
                    }
                    candidates = { let joined2 = a; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(&(",")); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } out };
                    }
                }
                None => {
                }
            }
            let mut justify = "-".to_string();
            for j in 0..match u32::try_from(d.justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if d.justification_decisions[usize::try_from(j).unwrap_or(0)].clone().line_range.clone().start == (line.range).clone().start && ((d.justification_decisions[usize::try_from(j).unwrap_or(0)]).clone().line_range).clone().end == (line.range).clone().end {
                    let q = (d.justification_decisions[usize::try_from(j).unwrap_or(0)]).clone();
                    justify = format!("{}{}{}{}",
            "deficit=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(q.deficit_before),
            "->",
            LayoutDumpFormat::layout_dump_format_dump_fmt(q.deficit_after)
        );
                    let mut first_alloc = true;
                    for _g_index1 in 0..match u32::try_from(q.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let a = (q.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
                        justify += &(format!("{}{}{}{}{}{}{}",
            (if first_alloc { " ".to_string() } else { ",".to_string() }),
            (a.kind).to_string(),
            "@",
            crate::runtime::int_text::IntText::int_text((a.cluster_range).clone().start),
            "+",
            LayoutDumpFormat::layout_dump_format_dump_fmt(a.delta),
            (if a.reason.to_string() == (a.kind).to_string() { "".to_string() } else { format!("{}{}{}",
            "(",
            (a.reason).to_string(),
            ")"
        ).to_string() })
        ));
                        first_alloc = false;
                    }
                    break;
                }
            }
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "line[",
            crate::runtime::int_text::IntText::int_text(i),
            "] ",
            crate::runtime::int_text::IntText::int_text((line.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((line.range).clone().end),
            " ",
            (if line.indent > (0 as f64) { format!("{}{}{}",
            "indent=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(line.indent),
            " "
        ).to_string() } else { "".to_string() }),
            (if line.hyphen_advance > (0 as f64) { format!("{}{}{}",
            "hyphen=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(line.hyphen_advance),
            " "
        ).to_string() } else { "".to_string() }),
            "natural=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(line.natural_width),
            " adjusted=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(line.adjusted_width),
            " visual=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(line.visual_width),
            " repair=",
            repair,
            " candidates=",
            candidates,
            " justify=",
            justify,
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index2 in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}",
            "cluster ",
            crate::runtime::int_text::IntText::int_text((c.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((c.range).clone().end),
            " '",
            (c.display_text).to_string(),
            "' adv=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(c.advance),
            (if c.glyph_inline_shift != 0 as f64 { format!("{}{}",
            " glyphShift=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(c.glyph_inline_shift)
        ).to_string() } else { "".to_string() }),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index3 in 0..match u32::try_from(d.font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.font_decisions[usize::try_from(_g_index3).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "font ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " role=",
            (x.role).to_string(),
            " key=",
            (x.font_key).to_string(),
            " display='",
            (x.display_text).to_string(),
            "' sub=",
            (x.substitution_reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index4 in 0..match u32::try_from(d.role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.role_overrides[usize::try_from(_g_index4).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "role-override ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " source='",
            LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_string().as_str()),
            "' ",
            (x.original_role).to_string(),
            "->",
            (x.overridden_role).to_string(),
            " policy=",
            (x.source).to_string(),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index5 in 0..match u32::try_from(d.punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.punctuation_decisions[usize::try_from(_g_index5).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "punct ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " '",
            (x.char).to_string(),
            "' class=",
            (x.punctuation_class).to_string(),
            " adv=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.advance),
            " body=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.body_width),
            " lead=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_natural),
            " trail=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_natural),
            (if x.leading_glue_initially_consumed != 0 as f64 || x.trailing_glue_initially_consumed != 0 as f64 { format!("{}{}{}{}",
            " initial=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_initially_consumed),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_initially_consumed)
        ).to_string() } else { "".to_string() }),
            " anchor=",
            (x.anchor).to_string(),
            " source=",
            (x.geometry_source).to_string(),
            (if x.advance_expansion != 0 as f64 { format!("{}{}",
            " expand=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.advance_expansion)
        ).to_string() } else { "".to_string() }),
            (if x.glyph_inline_shift != 0 as f64 { format!("{}{}",
            " glyphShift=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.glyph_inline_shift)
        ).to_string() } else { "".to_string() }),
            (match &(x.glyph_placement_reason) { None => "".to_string(), Some(__option20) => format!("{}{}",
            " placement=",
            (*__option20).clone()
        ).to_string() }),
            (match &(x.halt_advance) { None => "".to_string(), Some(__option21) => format!("{}{}",
            " halt=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(*__option21)
        ).to_string() }),
            (match &(x.ink_bounds_fallback) { None => "".to_string(), Some(__option22) => format!("{}{}",
            " fallback=",
            (*__option22).clone()
        ).to_string() }),
            (match &(x.halt_validation) { None => "".to_string(), Some(__option23) => format!("{}{}",
            " haltWarn=",
            (*__option23).clone()
        ).to_string() }),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index6 in 0..match u32::try_from(d.geometry_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.geometry_decisions[usize::try_from(_g_index6).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "geom ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " body=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.body_width),
            " lead=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_consumed),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_natural),
            " trail=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_consumed),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_natural),
            " justify=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.justification_delta),
            (if x.ruby_spread != 0 as f64 { format!("{}{}",
            " ruby=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.ruby_spread)
        ).to_string() } else { "".to_string() }),
            (if x.glyph_inline_shift != 0 as f64 { format!("{}{}",
            " glyphShift=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.glyph_inline_shift)
        ).to_string() } else { "".to_string() }),
            (match &(x.glyph_placement_reason) { None => "".to_string(), Some(__option24) => format!("{}{}",
            " placement=",
            (*__option24).clone()
        ).to_string() }),
            " resolved=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.resolved_advance),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index7 in 0..match u32::try_from(d.inline_box_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.inline_box_decisions[usize::try_from(_g_index7).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "inline-box ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " start=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.inline_start),
            " end=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.inline_end),
            " outer=",
            (x.outer_spacing).to_string(),
            " clusters=",
            crate::runtime::int_text::IntText::int_text(x.first_cluster_index),
            "-",
            crate::runtime::int_text::IntText::int_text(x.last_cluster_index),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.inline_object_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.inline_object_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "inline-object ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " advance=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.advance),
            " ascent=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.ascent),
            " descent=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.descent),
            " cluster=",
            crate::runtime::int_text::IntText::int_text(x.cluster_index),
            " line=",
            crate::runtime::int_text::IntText::int_text(x.line_index),
            " edges=",
            (if x.leading_uniform_stretch { "stretch".to_string() } else { "fixed".to_string() }),
            "/",
            (match &(x.leading_preferred_stretch_kind) { None => "-".to_string(), Some(__option25) => (*__option25).clone() }),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_preferred_stretch_natural_width),
            "→",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_preferred_stretch_target_width),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_preferred_stretch_capacity),
            "/",
            (if x.leading_prevents_line_break { "closed".to_string() } else { "natural".to_string() }),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_shrink_capacity),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_line_end_discardable_advance),
            "..",
            (if x.trailing_uniform_stretch { "stretch".to_string() } else { "fixed".to_string() }),
            "/",
            (match &(x.trailing_preferred_stretch_kind) { None => "-".to_string(), Some(__option27) => (*__option27).clone() }),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_preferred_stretch_natural_width),
            "→",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_preferred_stretch_target_width),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_preferred_stretch_capacity),
            "/",
            (if x.trailing_prevents_line_break { "closed".to_string() } else { "natural".to_string() }),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_shrink_capacity),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_line_end_discardable_advance),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index8 in 0..match u32::try_from(d.spacing_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.spacing_decisions[usize::try_from(_g_index8).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "spacing ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " '",
            (x.left_char).to_string(),
            (x.right_char).to_string(),
            "' inner=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.natural_inner_glue),
            "->",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.adjusted_inner_glue),
            " target=",
            crate::runtime::int_text::IntText::int_text((x.reduction_target_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.reduction_target_range).clone().end),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index9 in 0..match u32::try_from(d.auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.auto_space_decisions[usize::try_from(_g_index9).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}",
            "autospace ",
            crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().end),
            " side=",
            (x.side).to_string(),
            " boundary=",
            (x.boundary_role).to_string(),
            " reduction=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.total_reduction),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index10 in 0..match u32::try_from(d.mandatory_break_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.mandatory_break_decisions[usize::try_from(_g_index10).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}",
            "mandatorybreak ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " afterCluster=",
            crate::runtime::int_text::IntText::int_text(x.break_after_cluster_index),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index11 in 0..match u32::try_from(d.zero_width_break_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.zero_width_break_decisions[usize::try_from(_g_index11).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}",
            "zerowidthbreak ",
            crate::runtime::int_text::IntText::int_text((x.range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.range).clone().end),
            " source='",
            LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_string().as_str()),
            "' cluster=",
            crate::runtime::int_text::IntText::int_text(x.cluster_index),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for _g_index12 in 0..match u32::try_from(d.line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.line_edge_trim_decisions[usize::try_from(_g_index12).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}",
            "edgetrim ",
            crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().end),
            " side=",
            (x.side).to_string(),
            " trim=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trim_amount),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.decoration_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.decoration_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "deco ",
            crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().end),
            " '",
            (x.source_text).to_string(),
            "' kind=",
            (x.kind).to_string(),
            " applied=",
            (if x.applied { "true".to_string() } else { "false".to_string() }),
            " anchor=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.anchor_x),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.anchor_y),
            " diameter=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.dot_diameter),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        match &(d.line_spacing_decision) {
            Some(__option29) => {
                let x = (*__option29).clone();
                {
                    let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "linespacing natural=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.natural_height),
            " requested=",
            (match &(x.requested_line_height) { None => "-".to_string(), Some(__option30) => LayoutDumpFormat::layout_dump_format_dump_fmt(*__option30).to_string() }),
            " resolved=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.resolved_height),
            " floor=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.spacing_floor),
            " applied=",
            (if x.floor_applied { "true".to_string() } else { "false".to_string() }),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        match &(d.ruby_line_height_decision) {
            Some(__option31) => {
                let x = (*__option31).clone();
                {
                    let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "rubylineheight mode=",
            (x.mode).to_string(),
            " base=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_line_height),
            " face=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_face_height),
            " ruby=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.ruby_extent),
            " available=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.available_interline_space),
            " maxExtra=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.max_extra),
            " extras=",
            LayoutDumpFormat::layout_dump_format_join_floats(&x.line_extras),
            " lines=",
            LayoutDumpFormat::layout_dump_format_join_ints(&x.expanded_line_indices),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        for i in 0..match u32::try_from(d.decoration_segments.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.decoration_segments[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "decobox ",
            crate::runtime::int_text::IntText::int_text((x.source_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.source_range).clone().end),
            " kind=",
            (x.kind).to_string(),
            " line=",
            crate::runtime::int_text::IntText::int_text(x.line_index),
            " rect=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.left),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.top),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.right),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.bottom),
            " open=",
            (if x.open_start { "start".to_string() } else { "-".to_string() }),
            "/",
            (if x.open_end { "end".to_string() } else { "-".to_string() }),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.ruby_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.ruby_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "ruby ",
            crate::runtime::int_text::IntText::int_text((x.base_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.base_range).clone().end),
            " '",
            (x.text).to_string(),
            "' line=",
            crate::runtime::int_text::IntText::int_text(x.line_index),
            " centerX=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.center_x),
            " baselineY=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.baseline_y),
            " size=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.font_size),
            " box=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.ascent),
            "/",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.descent),
            " width=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.width),
            " overhang=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.overhang),
            " locale=",
            (x.locale).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.bopomofo_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.bopomofo_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x1 = format!("{}{}{}{}{}{}{}{}{}{}{}",
            "bopomofo ",
            crate::runtime::int_text::IntText::int_text((x.base_range).clone().start),
            "-",
            crate::runtime::int_text::IntText::int_text((x.base_range).clone().end),
            " '",
            (x.text).to_string(),
            "' line=",
            crate::runtime::int_text::IntText::int_text(x.line_index),
            " locale=",
            (x.locale).to_string(),
            concat!("\n",
"")
        );
                o_b += &(x1.to_string());
            }
            for j in 0..match u32::try_from(x.placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let p = (x.placements[usize::try_from(j).unwrap_or(0)]).clone();
                {
                    let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "  ",
            p.role.name(),
            " '",
            (p.text).to_string(),
            "' rect=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.left),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.top),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.width),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.height),
            " draw=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.draw_x),
            ",",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.baseline_y),
            " size=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(p.font_size),
            concat!("\n",
"")
        );
                    o_b += &(x.to_string());
                }
            }
        }
        match &(d.inline_object_line_height_decision) {
            Some(__option32) => {
                let x = (*__option32).clone();
                {
                    let x = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "inlineobjectlineheight base=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_line_height),
            " face=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_face_ascent),
            "+",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_face_descent),
            " available=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.available_interline_space),
            " clearance=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.minimum_clearance),
            " ascents=",
            LayoutDumpFormat::layout_dump_format_join_floats(&x.line_ascents),
            " descents=",
            LayoutDumpFormat::layout_dump_format_join_floats(&x.line_descents),
            " extras=",
            LayoutDumpFormat::layout_dump_format_join_floats(&x.line_extras),
            " boundaries=",
            LayoutDumpFormat::layout_dump_format_join_floats(&x.boundary_shifts_after),
            " trailing=",
            LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_extra),
            " lines=",
            LayoutDumpFormat::layout_dump_format_join_ints(&x.expanded_line_indices),
            " reason=",
            (x.reason).to_string(),
            concat!("\n",
"")
        );
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        return o_b;
    }

    pub fn layout_dump_format_layout_dump_diff_message(id: &str, expected: &str, actual: &str) -> String {
        let e = u_string::split(&expected, &concat!("\n",
""));
        let a = u_string::split(&actual, &concat!("\n",
""));
        let mut diffs: Vec<String> = vec![];
        let n = if i32::from_ne_bytes((u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0) } else {
u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) };
        for i in 0..n {
            if (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some((e[usize::try_from(i).unwrap_or(0)]).clone()) } else { None }) != (if ({ let v: u32 = i;
i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { Some((a[usize::try_from(i).unwrap_or(0)]).clone()) } else { None }) {
                diffs.push(format!("{}{}{}{}{}{}",
            "  line ",
            crate::runtime::int_text::IntText::int_text(u32::wrapping_add(i, 1)),
            concat!(":\n",
"    golden: "),
            (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { (e[usize::try_from(i).unwrap_or(0)]).clone() } else { "<missing>".to_string() }),
            concat!("\n",
"    actual: "),
            (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) { (a[usize::try_from(i).unwrap_or(0)]).clone() } else { "<missing>".to_string() })
        ));
                if i32::from_ne_bytes((u32::try_from((diffs.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) >= 8 {
                    diffs.push("  …".to_string());
                    break;
                }
            }
        }
        return format!("{}{}{}{}",
            "golden mismatch for fixture '",
            id,
            concat!("':\n",
""),
            { let joined3 = diffs; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(&(concat!("\n",
""))); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } out }
        );
    }
}

#[derive(Clone, PartialEq)]
pub struct BasicProfileResolver {
}

impl BasicProfileResolver {
    pub const fn new() -> Self {
        Self {
        }
    }

    pub fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        let p = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((p.id).to_string().as_str(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((p.auto_space).clone()),
Some(p.glue_placement), (p.adjustment).clone(), KinsokuMode::Fixed { level: KinsokuLevel::Basic, hanging: HangingPunctuationStyle::Disabled }, (p.punctuation_width).clone());
    }
}

impl ClreqProfileResolver for BasicProfileResolver {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.LayoutDumpFormat.BasicProfileResolver"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ClreqProfileResolver> {
        Box::new(self.clone())
    }

    fn resolve(&self, _id: LayoutProfileId) -> ClreqProfile {
        let p = (*crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_MAINLAND_HORIZONTAL).clone();
        return ClreqProfile::new((p.id).to_string().as_str(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((p.auto_space).clone()),
Some(p.glue_placement), (p.adjustment).clone(), KinsokuMode::Fixed { level: KinsokuLevel::Basic, hanging: HangingPunctuationStyle::Disabled }, (p.punctuation_width).clone());
    }
}
