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
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum LayoutDumpFormatLayoutFixtureDumpFault {
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ParagraphLayoutEngineNewFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineNewFault),
    ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(crate::org::tiqian::layout::paragraph_layout_engine::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFault),
}
impl std::fmt::Display for LayoutDumpFormatLayoutFixtureDumpFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutDumpFormatLayoutFixtureDumpFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
            LayoutDumpFormatLayoutFixtureDumpFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineNewFaultFault(value) => write!(formatter, "{}", value),
            LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(value) => write!(formatter, "{}", value),
        }
    }
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
    pub(crate) fn layout_dump_format_join_ints(a: &[u32]) -> UString {
        let mut b: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.push(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(a[usize::try_from(i).unwrap_or(0)])).as_str()));
        }
        return UString::from(format!("{}", if u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { UString::from("-") } else { UString::from(format!("{}", { let joined = b; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(","); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()) }).as_str());
    }

    pub(crate) fn layout_dump_format_join_floats(a: &[f64]) -> UString {
        let mut b: Vec<UString> = vec![];
        for i in 0..match u32::try_from(a.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.push(LayoutDumpFormat::layout_dump_format_dump_fmt(a[usize::try_from(i).unwrap_or(0)]));
        }
        return UString::from(format!("{}", if u32::try_from((b.len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 { UString::from("-") } else { UString::from(format!("{}", { let joined1 = b; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()) }).as_str());
    }

    pub fn layout_dump_format_dump_fmt(value: f64) -> UString {
        if value.is_nan() {
            return UString::from("NaN").to_ustring();
        }
        if !(value).is_finite() {
            return if value > (0 as f64) { UString::from("Infinity") } else { UString::from("-Infinity") };
        }
        let negative = u32::from_ne_bytes(((FPHelper::float_to_i32(value)) as u32).to_ne_bytes()) > 2147483647;
        let magnitude = u32::from_ne_bytes(((match f64::from(f64::floor((value).abs() * format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0) + 0.5f64)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes());
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += (if negative { UString::from("-") } else { UString::from("") }).as_ustr(); __s += UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::from_ne_bytes(((match f64::from(f64::floor(format!("{}", (i32::from_ne_bytes(((magnitude) as i32).to_ne_bytes()))).parse::<f64>().unwrap_or(0.0) / format!("{}", (10i32)).parse::<f64>().unwrap_or(0.0) as f64)) { v if v.is_nan() => 0i32, v if v >= 2147483648.0 => 2147483647i32, v if v < -2147483648.0 => -2147483648i32, v if v < 0.0 => i32::from_ne_bytes(u32::wrapping_sub(0, match ((0.0 - v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((0.0 - v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }).to_ne_bytes()), v => i32::from_ne_bytes(match ((v).to_bits() >> 52) & 2047 { e if e < 1023 => 0u32, e => u32::try_from((((v).to_bits() & 4503599627370495) | 4503599627370496) >> (52 - (e - 1023))).unwrap_or(0) }.to_ne_bytes()) }) as u32).to_ne_bytes()))).as_str()).as_ustr(); __s += &(UString::from(".")); __s += UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::from_ne_bytes(((i32::from_ne_bytes(((magnitude) as i32).to_ne_bytes()) % 10i32) as u32).to_ne_bytes()))).as_str()).as_ustr(); __s }).as_str());
    }

    pub fn layout_dump_format_escape_dump_text(value: &UStr) -> UString {
    let __units = u_string::units(&value);
    let __count = u_string::unit_count(&value);
        let mut out_b = UString::new();
        for i in 0..match u32::try_from(u_string::unit_count(&(value))) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = u_string::unit_at_from(&__units, i).unwrap_or(0);
            if c == 10 {
                out_b += &(UString::from("\\n"));
            } else {
                if c == 13 {
                    out_b += &(UString::from("\\r"));
                } else {
                    if c == 11 {
                        out_b += &(UString::from("\\v"));
                    } else {
                        if c == 12 {
                            out_b += &(UString::from("\\f"));
                        } else {
                            if c == 133 {
                                out_b += &(UString::from("\\u0085"));
                            } else {
                                if c == 8232 {
                                    out_b += &(UString::from("\\u2028"));
                                } else {
                                    if c == 8233 {
                                        out_b += &(UString::from("\\u2029"));
                                    } else {
                                        if c == 8203 {
                                            out_b += &(UString::from("\\u200B"));
                                        } else {
                                            let c = c;
                                            out_b += &(if c > 0xFFFF { u_string::from_units(&[0xD800 + (((c) - 0x10000) >> 10) as u16, 0xDC00 + (((c) - 0x10000) & 0x3FF) as u16]) } else { u_string::from_units(&[(c) as u16]) });
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

    pub fn layout_dump_format_layout_fixture_dump(f: LayoutFixture, text_shaper: Option<Arc<Mutex<dyn ITextShaper>>>, font_metrics_resolver: Option<Box<dyn FontMetricsResolver>>) -> Result<UString, LayoutDumpFormatLayoutFixtureDumpFault> {
        let mut out_b = UString::new();
        out_b += &(UString::from("fixture: "));
        {
            let x = (f.id).to_ustring().clone();
            out_b += &(x.to_string());
        }
        out_b += &(UString::from(concat!("\n",
"text: ")));
        {
            let x = LayoutDumpFormat::layout_dump_format_escape_dump_text((f.text).to_ustring().as_ustr());
            out_b += &(x.to_string());
        }
        out_b += &(UString::from(concat!("\n",
"maxWidth: ")));
        {
            let x = LayoutDumpFormat::layout_dump_format_dump_fmt((f.constraints).clone().max_width);
            out_b += &(x.to_string());
        }
        out_b += &(UString::from(concat!("\n",
"")));
        let shaper: Arc<Mutex<dyn ITextShaper>> = match &(text_shaper) { None => Arc::new(Mutex::new(ExplainableStubTextShaper::new())), Some(__option) => (*__option).clone() };
        for n in 0..3 {
            let breaker: Box<dyn LineBreaker> = if n == 0 { Box::new(GreedyLineBreaker::new(None, None, None, None)) } else { if n == 1 { Box::new(LookaheadLineBreaker::new(Some(2), Some(2), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12.0))) } else { Box::new(ParagraphDpLineBreaker::new(Some(8), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64)).map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::TextRangeErrorFault(e))?) } };
            let hyphenator: Box<dyn Hyphenator> = if f.use_english_hyphenation { EnglishHyphenation::english_hyphenation_en_us().map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::UStringFaultFault(e))? } else { Box::new(NoHyphenator::new()) };
            let resolver: Option<Box<dyn ClreqProfileResolver>> = if f.pin_basic_no_hang { Some(Box::new(BasicProfileResolver::new()) as Box<dyn ClreqProfileResolver>) } else { None };
            let mut engine = ExplainableStubParagraphLayoutEngine::new(Some(Box::new(CjkFontRoleClassifier::new())), Some(Box::new(ParagraphLayoutFallbackResolver::new(Some(UString::from("cjk-primary")), Some(UString::from("latin-primary")), Some(UString::from("symbol-fallback")))?)), resolver, match &(font_metrics_resolver) { None => Some(Box::new(StubFontMetricsResolver::new()) as Box<dyn FontMetricsResolver>), Some(__option3) => Some((*__option3).clone()) }, Some(Box::new(ScriptAwareFontMetricsNormalizer::new())), Some((PunctuationAtomBuilder::new(None, None)?).clone()), Some((PunctuationSpacingCompressor::new()?).clone()), Some(QuotePairAnalyzer::new()), Some((breaker).clone()), Some(Justifier::new(Some(0.5), Some(0.25))), Some((shaper).clone()), Some((hyphenator).clone()), Some(Arc::new(Mutex::new(LruWidthIndependentAnnotationCache::new(512))))).map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineNewFaultFault(e))?;
            let indent = match &(f.first_line_indent_em) { None => None, Some(__option11) => Some(Ic(*__option11)) };
            let input = LayoutInput::new(TiqianTextContent::new((f.text).to_ustring().as_ustr(), Some(vec![]), Some(vec![]), Some((f.line_break_spans).clone()), Some(vec![])), Some(TextStyle::new(Some(vec![]), Some(16.0), Some(UString::from("zh-Hans")), Some(400), Some(false), Some(0.0), Some(InlineAttachment::None))), Some(ParagraphStyle::new(Some(LastLineAlignment::Start), Some(WritingMode::HorizontalTb), f.line_height, (indent).clone(), Some(Ic::zero()), Some(MeasureAdaptiveFirstLineIndent::new(Some(14.0), Some(1.0), Some(2.0))), Some((f.line_length_grid).clone()), Some(f.ruby_line_height_mode), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_INLINE_OBJECT_MINIMUM_CLEARANCE_EM), Some(ParagraphStyle::PARAGRAPH_STYLE_DEFAULT_EMPHASIS_DOT_GAP_EM))), (f.constraints).clone(), Some((*crate::org::tiqian::core::built_in_layout_profiles::BUILT_IN_LAYOUT_PROFILES_CLREQ_HORIZONTAL).clone()), Some((f.decorations).clone()), Some((f.ruby_spans).clone()), Some(vec![]), Some(vec![]));
            {
                let x = LayoutDumpFormat::layout_dump_format_decision_dump(engine.layout((input).clone()).map_err(|e| LayoutDumpFormatLayoutFixtureDumpFault::ParagraphLayoutEngineLayoutWithRejectedTechnicalTiersFaultFault(e))?, if n == 0 { UString::from("greedy") } else { if n == 1 { UString::from("lookahead") } else { UString::from("paragraph-dp") }.to_ustring() }.as_ustr());
                out_b += &(x.to_string());
            }
        }
        return Ok(out_b);
    }

    pub fn layout_dump_format_decision_dump(r: LayoutResult, label: &UStr) -> UString {
        let mut o_b = UString::new();
        let d = (r.debug).clone().clone();
        o_b += &(UString::from("== "));
        o_b += &(label.to_string());
        o_b += &(UString::from(concat!(" ==\n",
"size ")));
        {
            let x = LayoutDumpFormat::layout_dump_format_dump_fmt((r.size).clone().width);
            o_b += &(x.to_string());
        }
        o_b += &(UString::from("x"));
        {
            let x = LayoutDumpFormat::layout_dump_format_dump_fmt((r.size).clone().height);
            o_b += &(x.to_string());
        }
        o_b += &(UString::from(concat!("\n",
"")));
        if match &(d.line_length_grid_decision) { Some(__option12) => __option12.enabled && (__option12.slack) > (0 as f64), None => false } {
            let g = d.line_length_grid_decision.clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("grid container=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().container_width).as_ustr(); __s += &(UString::from(" measure=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().measure).as_ustr(); __s += &(UString::from("(")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(g.as_ref().unwrap().cells)).as_str())); __s += &(UString::from("字) slack=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().slack).as_ustr(); __s += &(UString::from(" body=")); __s += (g.as_ref().unwrap().body_alignment).to_ustring().as_ustr(); __s += &(UString::from("@")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(g.as_ref().unwrap().body_offset).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        match &(d.first_line_indent_decision) {
            Some(__option13) => {
                if __option13.source.to_ustring() != UString::from("Explicit") {
                let x = (*__option13).clone();
                {
                    let x = { let mut __s = UString::new(); __s += &(UString::from("firstindent ")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.resolved_em).as_ustr(); __s += &(UString::from("字 measure=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.measure_em).as_ustr(); __s += &(UString::from("字 threshold=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.threshold_em).as_ustr(); __s += &(UString::from("字 ")); __s += (x.source).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
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
                    let x = { let mut __s = UString::new(); __s += &(UString::from("kinsoku measure=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.measure_em).as_ustr(); __s += &(UString::from("字 level=")); __s += (x.level).to_ustring().as_ustr(); __s += &(UString::from(" hang=")); __s += (x.hanging).to_ustring().as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        for i in 0..match u32::try_from(d.contextual_kinsoku_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.contextual_kinsoku_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("context-kinsoku ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" source='")); __s += LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from("' cluster=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.cluster_index)).as_str())); __s += &(UString::from(" forbid=")); __s += (x.forbidden_position).to_ustring().as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += (match &(x.impossible_measure_fallback) { None => UString::from(""), Some(__option15) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" fallback=")); __s += (*__option15).clone().as_ustr(); __s }).as_str()) }).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.break_opportunity_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.break_opportunity_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("break-opportunity ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" source='")); __s += LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from("' offsets=")); __s += LayoutDumpFormat::layout_dump_format_join_ints(&x.break_offsets).as_ustr(); __s += (match &(x.tier) { None => UString::from(""), Some(__option16) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" tier=")); __s += (*__option16).clone().as_ustr(); __s }).as_str()) }).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.emergency_tracking_eligibility_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.emergency_tracking_eligibility_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("tracking-eligibility ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" source='")); __s += LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from("' reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.inline_object_punctuation_attachment_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.inline_object_punctuation_attachment_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("inline-object-punctuation ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.object_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.object_range).clone().end)).as_str())); __s += &(UString::from(" separator=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.separator_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.separator_range).clone().end)).as_str())); __s += &(UString::from(" punctuation=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.punctuation_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.punctuation_range).clone().end)).as_str())); __s += &(UString::from(" source='")); __s += LayoutDumpFormat::layout_dump_format_escape_dump_text((x.punctuation_text).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from("' collapsed=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.collapsed_advance).as_ustr(); __s += &(UString::from(" protected=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.protected_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.protected_range).clone().end)).as_str())); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(r.lines.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let line = (r.lines[usize::try_from(i).unwrap_or(0)]).clone();
            let x = if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((d.line_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some((d.line_decisions[usize::try_from(i).unwrap_or(0)]).clone()) } else { None };
            let repair = if match &(x) { None => true,
Some(__option18) => __option18.repair_decision.is_none() } { UString::from("-") } else { UString::from(format!("{}", { let mut __s = UString::new(); __s += ((x.as_ref().unwrap().repair_decision).clone().as_ref().unwrap().kind).to_ustring().as_ustr(); __s += &(UString::from("(")); __s += ((x.as_ref().unwrap().repair_decision).clone().as_ref().unwrap().reason_code).to_ustring().as_ustr(); __s += &(UString::from(" shrink=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt((x.as_ref().unwrap().repair_decision).clone().as_ref().unwrap().shrink).as_ustr(); __s += &(UString::from(")")); __s }).as_str()) };
            let mut candidates = UString::from("-").to_ustring();
            match &(x) {
                Some(__option19) => {
                    if i32::from_ne_bytes(((u32::try_from((__option19.repair_candidates.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                    let mut a: Vec<UString> = vec![];
                    for _g_index in 0..match u32::try_from(__option19.repair_candidates.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let z = (__option19.repair_candidates[usize::try_from(_g_index).unwrap_or(0)]).clone();
                        a.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += (z.kind).to_ustring().as_ustr(); __s += (if z.accepted { UString::from("+") } else { UString::from("-") }).as_ustr(); __s }).as_str()));
                    }
                    candidates = UString::from(format!("{}", { let joined2 = a; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(","); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str());
                    }
                }
                None => {
                }
            }
            let mut justify = UString::from("-").to_ustring();
            for j in 0..match u32::try_from(d.justification_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                if d.justification_decisions[usize::try_from(j).unwrap_or(0)].clone().line_range.clone().start == (line.range).clone().start && ((d.justification_decisions[usize::try_from(j).unwrap_or(0)]).clone().line_range).clone().end == (line.range).clone().end {
                    let q = (d.justification_decisions[usize::try_from(j).unwrap_or(0)]).clone();
                    justify = UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("deficit=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(q.deficit_before).as_ustr(); __s += &(UString::from("->")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(q.deficit_after).as_ustr(); __s }).as_str());
                    let mut first_alloc = true;
                    for _g_index1 in 0..match u32::try_from(q.allocations.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                        let a = (q.allocations[usize::try_from(_g_index1).unwrap_or(0)]).clone();
                        justify += &({ let mut __s = UString::new(); __s += (if first_alloc { UString::from(" ") } else { UString::from(",") }).as_ustr(); __s += (a.kind).to_ustring().as_ustr(); __s += &(UString::from("@")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((a.cluster_range).clone().start)).as_str())); __s += &(UString::from("+")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(a.delta).as_ustr(); __s += (if a.reason.to_ustring() == (a.kind).to_ustring() { UString::from("") } else { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("(")); __s += (a.reason).to_ustring().as_ustr(); __s += &(UString::from(")")); __s }).as_str()) }).as_ustr(); __s });
                        first_alloc = false;
                    }
                    break;
                }
            }
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("line[")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(i)).as_str())); __s += &(UString::from("] ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((line.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((line.range).clone().end)).as_str())); __s += &(UString::from(" ")); __s += (if line.indent > (0 as f64) { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("indent=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(line.indent).as_ustr(); __s += &(UString::from(" ")); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += (if line.hyphen_advance > (0 as f64) { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("hyphen=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(line.hyphen_advance).as_ustr(); __s += &(UString::from(" ")); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += &(UString::from("natural=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(line.natural_width).as_ustr(); __s += &(UString::from(" adjusted=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(line.adjusted_width).as_ustr(); __s += &(UString::from(" visual=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(line.visual_width).as_ustr(); __s += &(UString::from(" repair=")); __s += repair.as_ustr(); __s += &(UString::from(" candidates=")); __s += candidates.as_ustr(); __s += &(UString::from(" justify=")); __s += justify.as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index2 in 0..match u32::try_from(r.clusters.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let c = (r.clusters[usize::try_from(_g_index2).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("cluster ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((c.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((c.range).clone().end)).as_str())); __s += &(UString::from(" '")); __s += (c.display_text).to_ustring().as_ustr(); __s += &(UString::from("' adv=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(c.advance).as_ustr(); __s += (if c.glyph_inline_shift != 0 as f64 { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" glyphShift=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(c.glyph_inline_shift).as_ustr(); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index3 in 0..match u32::try_from(d.font_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.font_decisions[usize::try_from(_g_index3).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("font ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" role=")); __s += (x.role).to_ustring().as_ustr(); __s += &(UString::from(" key=")); __s += (x.font_key).to_ustring().as_ustr(); __s += &(UString::from(" display='")); __s += (x.display_text).to_ustring().as_ustr(); __s += &(UString::from("' sub=")); __s += (x.substitution_reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index4 in 0..match u32::try_from(d.role_overrides.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.role_overrides[usize::try_from(_g_index4).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("role-override ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" source='")); __s += LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from("' ")); __s += (x.original_role).to_ustring().as_ustr(); __s += &(UString::from("->")); __s += (x.overridden_role).to_ustring().as_ustr(); __s += &(UString::from(" policy=")); __s += (x.source).to_ustring().as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index5 in 0..match u32::try_from(d.punctuation_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.punctuation_decisions[usize::try_from(_g_index5).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("punct ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" '")); __s += (x.char).to_ustring().as_ustr(); __s += &(UString::from("' class=")); __s += (x.punctuation_class).to_ustring().as_ustr(); __s += &(UString::from(" adv=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.advance).as_ustr(); __s += &(UString::from(" body=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.body_width).as_ustr(); __s += &(UString::from(" lead=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_natural).as_ustr(); __s += &(UString::from(" trail=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_natural).as_ustr(); __s += (if x.leading_glue_initially_consumed != 0 as f64 || x.trailing_glue_initially_consumed != 0 as f64 { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" initial=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_initially_consumed).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_initially_consumed).as_ustr(); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += &(UString::from(" anchor=")); __s += (x.anchor).to_ustring().as_ustr(); __s += &(UString::from(" source=")); __s += (x.geometry_source).to_ustring().as_ustr(); __s += (if x.advance_expansion != 0 as f64 { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" expand=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.advance_expansion).as_ustr(); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += (if x.glyph_inline_shift != 0 as f64 { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" glyphShift=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.glyph_inline_shift).as_ustr(); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += (match &(x.glyph_placement_reason) { None => UString::from(""), Some(__option20) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" placement=")); __s += (*__option20).clone().as_ustr(); __s }).as_str()) }).as_ustr(); __s += (match &(x.halt_advance) { None => UString::from(""), Some(__option21) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" halt=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(*__option21).as_ustr(); __s }).as_str()) }).as_ustr(); __s += (match &(x.ink_bounds_fallback) { None => UString::from(""), Some(__option22) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" fallback=")); __s += (*__option22).clone().as_ustr(); __s }).as_str()) }).as_ustr(); __s += (match &(x.halt_validation) { None => UString::from(""), Some(__option23) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" haltWarn=")); __s += (*__option23).clone().as_ustr(); __s }).as_str()) }).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index6 in 0..match u32::try_from(d.geometry_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.geometry_decisions[usize::try_from(_g_index6).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("geom ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" body=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.body_width).as_ustr(); __s += &(UString::from(" lead=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_consumed).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_glue_natural).as_ustr(); __s += &(UString::from(" trail=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_consumed).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_glue_natural).as_ustr(); __s += &(UString::from(" justify=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.justification_delta).as_ustr(); __s += (if x.ruby_spread != 0 as f64 { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" ruby=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.ruby_spread).as_ustr(); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += (if x.glyph_inline_shift != 0 as f64 { UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" glyphShift=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.glyph_inline_shift).as_ustr(); __s }).as_str()) } else { UString::from("") }).as_ustr(); __s += (match &(x.glyph_placement_reason) { None => UString::from(""), Some(__option24) => UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(" placement=")); __s += (*__option24).clone().as_ustr(); __s }).as_str()) }).as_ustr(); __s += &(UString::from(" resolved=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.resolved_advance).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index7 in 0..match u32::try_from(d.inline_box_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.inline_box_decisions[usize::try_from(_g_index7).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("inline-box ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" start=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.inline_start).as_ustr(); __s += &(UString::from(" end=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.inline_end).as_ustr(); __s += &(UString::from(" outer=")); __s += (x.outer_spacing).to_ustring().as_ustr(); __s += &(UString::from(" clusters=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.first_cluster_index)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.last_cluster_index)).as_str())); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.inline_object_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.inline_object_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("inline-object ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" advance=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.advance).as_ustr(); __s += &(UString::from(" ascent=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.ascent).as_ustr(); __s += &(UString::from(" descent=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.descent).as_ustr(); __s += &(UString::from(" cluster=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.cluster_index)).as_str())); __s += &(UString::from(" line=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.line_index)).as_str())); __s += &(UString::from(" edges=")); __s += (if x.leading_uniform_stretch { UString::from("stretch") } else { UString::from("fixed") }).as_ustr(); __s += &(UString::from("/")); __s += (match &(x.leading_preferred_stretch_kind) { None => UString::from("-"), Some(__option25) => (*__option25).clone() }).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_preferred_stretch_natural_width).as_ustr(); __s += &(UString::from("→")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_preferred_stretch_target_width).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_preferred_stretch_capacity).as_ustr(); __s += &(UString::from("/")); __s += (if x.leading_prevents_line_break { UString::from("closed") } else { UString::from("natural") }).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_shrink_capacity).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.leading_line_end_discardable_advance).as_ustr(); __s += &(UString::from("..")); __s += (if x.trailing_uniform_stretch { UString::from("stretch") } else { UString::from("fixed") }).as_ustr(); __s += &(UString::from("/")); __s += (match &(x.trailing_preferred_stretch_kind) { None => UString::from("-"), Some(__option27) => (*__option27).clone() }).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_preferred_stretch_natural_width).as_ustr(); __s += &(UString::from("→")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_preferred_stretch_target_width).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_preferred_stretch_capacity).as_ustr(); __s += &(UString::from("/")); __s += (if x.trailing_prevents_line_break { UString::from("closed") } else { UString::from("natural") }).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_shrink_capacity).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_line_end_discardable_advance).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index8 in 0..match u32::try_from(d.spacing_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.spacing_decisions[usize::try_from(_g_index8).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("spacing ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" '")); __s += (x.left_char).to_ustring().as_ustr(); __s += (x.right_char).to_ustring().as_ustr(); __s += &(UString::from("' inner=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.natural_inner_glue).as_ustr(); __s += &(UString::from("->")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.adjusted_inner_glue).as_ustr(); __s += &(UString::from(" target=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.reduction_target_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.reduction_target_range).clone().end)).as_str())); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index9 in 0..match u32::try_from(d.auto_space_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.auto_space_decisions[usize::try_from(_g_index9).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("autospace ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().end)).as_str())); __s += &(UString::from(" side=")); __s += (x.side).to_ustring().as_ustr(); __s += &(UString::from(" boundary=")); __s += (x.boundary_role).to_ustring().as_ustr(); __s += &(UString::from(" reduction=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.total_reduction).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index10 in 0..match u32::try_from(d.mandatory_break_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.mandatory_break_decisions[usize::try_from(_g_index10).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("mandatorybreak ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" afterCluster=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.break_after_cluster_index)).as_str())); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index11 in 0..match u32::try_from(d.zero_width_break_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.zero_width_break_decisions[usize::try_from(_g_index11).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("zerowidthbreak ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.range).clone().end)).as_str())); __s += &(UString::from(" source='")); __s += LayoutDumpFormat::layout_dump_format_escape_dump_text((x.source_text).to_ustring().as_ustr()).as_ustr(); __s += &(UString::from("' cluster=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.cluster_index)).as_str())); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for _g_index12 in 0..match u32::try_from(d.line_edge_trim_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.line_edge_trim_decisions[usize::try_from(_g_index12).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("edgetrim ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().end)).as_str())); __s += &(UString::from(" side=")); __s += (x.side).to_ustring().as_ustr(); __s += &(UString::from(" trim=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trim_amount).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.decoration_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.decoration_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("deco ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.cluster_range).clone().end)).as_str())); __s += &(UString::from(" '")); __s += (x.source_text).to_ustring().as_ustr(); __s += &(UString::from("' kind=")); __s += (x.kind).to_ustring().as_ustr(); __s += &(UString::from(" applied=")); __s += (if x.applied { UString::from("true") } else { UString::from("false") }).as_ustr(); __s += &(UString::from(" anchor=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.anchor_x).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.anchor_y).as_ustr(); __s += &(UString::from(" diameter=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.dot_diameter).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        match &(d.line_spacing_decision) {
            Some(__option29) => {
                let x = (*__option29).clone();
                {
                    let x = { let mut __s = UString::new(); __s += &(UString::from("linespacing natural=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.natural_height).as_ustr(); __s += &(UString::from(" requested=")); __s += (match &(x.requested_line_height) { None => UString::from("-"), Some(__option30) => LayoutDumpFormat::layout_dump_format_dump_fmt(*__option30).to_ustring() }).as_ustr(); __s += &(UString::from(" resolved=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.resolved_height).as_ustr(); __s += &(UString::from(" floor=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.spacing_floor).as_ustr(); __s += &(UString::from(" applied=")); __s += (if x.floor_applied { UString::from("true") } else { UString::from("false") }).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
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
                    let x = { let mut __s = UString::new(); __s += &(UString::from("rubylineheight mode=")); __s += (x.mode).to_ustring().as_ustr(); __s += &(UString::from(" base=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_line_height).as_ustr(); __s += &(UString::from(" face=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_face_height).as_ustr(); __s += &(UString::from(" ruby=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.ruby_extent).as_ustr(); __s += &(UString::from(" available=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.available_interline_space).as_ustr(); __s += &(UString::from(" maxExtra=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.max_extra).as_ustr(); __s += &(UString::from(" extras=")); __s += LayoutDumpFormat::layout_dump_format_join_floats(&x.line_extras).as_ustr(); __s += &(UString::from(" lines=")); __s += LayoutDumpFormat::layout_dump_format_join_ints(&x.expanded_line_indices).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        for i in 0..match u32::try_from(d.decoration_segments.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.decoration_segments[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("decobox ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.source_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.source_range).clone().end)).as_str())); __s += &(UString::from(" kind=")); __s += (x.kind).to_ustring().as_ustr(); __s += &(UString::from(" line=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.line_index)).as_str())); __s += &(UString::from(" rect=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.left).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.top).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.right).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.bottom).as_ustr(); __s += &(UString::from(" open=")); __s += (if x.open_start { UString::from("start") } else { UString::from("-") }).as_ustr(); __s += &(UString::from("/")); __s += (if x.open_end { UString::from("end") } else { UString::from("-") }).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.ruby_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.ruby_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x = { let mut __s = UString::new(); __s += &(UString::from("ruby ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.base_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.base_range).clone().end)).as_str())); __s += &(UString::from(" '")); __s += (x.text).to_ustring().as_ustr(); __s += &(UString::from("' line=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.line_index)).as_str())); __s += &(UString::from(" centerX=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.center_x).as_ustr(); __s += &(UString::from(" baselineY=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.baseline_y).as_ustr(); __s += &(UString::from(" size=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.font_size).as_ustr(); __s += &(UString::from(" box=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.ascent).as_ustr(); __s += &(UString::from("/")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.descent).as_ustr(); __s += &(UString::from(" width=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.width).as_ustr(); __s += &(UString::from(" overhang=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.overhang).as_ustr(); __s += &(UString::from(" locale=")); __s += (x.locale).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x.to_string());
            }
        }
        for i in 0..match u32::try_from(d.bopomofo_decisions.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            let x = (d.bopomofo_decisions[usize::try_from(i).unwrap_or(0)]).clone();
            {
                let x1 = { let mut __s = UString::new(); __s += &(UString::from("bopomofo ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.base_range).clone().start)).as_str())); __s += &(UString::from("-")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((x.base_range).clone().end)).as_str())); __s += &(UString::from(" '")); __s += (x.text).to_ustring().as_ustr(); __s += &(UString::from("' line=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(x.line_index)).as_str())); __s += &(UString::from(" locale=")); __s += (x.locale).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                o_b += &(x1.to_string());
            }
            for j in 0..match u32::try_from(x.placements.len()) { Ok(value) => value, Err(_) => u32::MAX } {
                let p = (x.placements[usize::try_from(j).unwrap_or(0)]).clone();
                {
                    let x = { let mut __s = UString::new(); __s += &(UString::from("  ")); __s += UString::from(p.role.name()).as_ustr(); __s += &(UString::from(" '")); __s += (p.text).to_ustring().as_ustr(); __s += &(UString::from("' rect=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.left).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.top).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.width).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.height).as_ustr(); __s += &(UString::from(" draw=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.draw_x).as_ustr(); __s += &(UString::from(",")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.baseline_y).as_ustr(); __s += &(UString::from(" size=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(p.font_size).as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                    o_b += &(x.to_string());
                }
            }
        }
        match &(d.inline_object_line_height_decision) {
            Some(__option32) => {
                let x = (*__option32).clone();
                {
                    let x = { let mut __s = UString::new(); __s += &(UString::from("inlineobjectlineheight base=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_line_height).as_ustr(); __s += &(UString::from(" face=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_face_ascent).as_ustr(); __s += &(UString::from("+")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.base_face_descent).as_ustr(); __s += &(UString::from(" available=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.available_interline_space).as_ustr(); __s += &(UString::from(" clearance=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.minimum_clearance).as_ustr(); __s += &(UString::from(" ascents=")); __s += LayoutDumpFormat::layout_dump_format_join_floats(&x.line_ascents).as_ustr(); __s += &(UString::from(" descents=")); __s += LayoutDumpFormat::layout_dump_format_join_floats(&x.line_descents).as_ustr(); __s += &(UString::from(" extras=")); __s += LayoutDumpFormat::layout_dump_format_join_floats(&x.line_extras).as_ustr(); __s += &(UString::from(" boundaries=")); __s += LayoutDumpFormat::layout_dump_format_join_floats(&x.boundary_shifts_after).as_ustr(); __s += &(UString::from(" trailing=")); __s += LayoutDumpFormat::layout_dump_format_dump_fmt(x.trailing_extra).as_ustr(); __s += &(UString::from(" lines=")); __s += LayoutDumpFormat::layout_dump_format_join_ints(&x.expanded_line_indices).as_ustr(); __s += &(UString::from(" reason=")); __s += (x.reason).to_ustring().as_ustr(); __s += &(UString::from(concat!("\n",
""))); __s };
                    o_b += &(x.to_string());
                }
            }
            None => {
            }
        }
        return o_b;
    }

    pub fn layout_dump_format_layout_dump_diff_message(id: &UStr, expected: &UStr, actual: &UStr) -> UString {
        let e = u_string::split(&expected, &UString::from(concat!("\n",
"")));
        let a = u_string::split(&actual, &UString::from(concat!("\n",
"")));
        let mut diffs: Vec<UString> = vec![];
        let n = if i32::from_ne_bytes(((u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0) } else { u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0) };
        for i in 0..n {
            if (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some((e[usize::try_from(i).unwrap_or(0)]).clone()) } else { None }) != (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { Some((a[usize::try_from(i).unwrap_or(0)]).clone()) } else { None }) {
                diffs.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("  line ")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(u32::wrapping_add(i, 1))).as_str())); __s += &(UString::from(concat!(":\n",
"    golden: "))); __s += (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((e.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { (e[usize::try_from(i).unwrap_or(0)]).clone() } else { UString::from("<missing>") }).as_ustr(); __s += &(UString::from(concat!("\n",
"    actual: "))); __s += (if ({ let v: u32 = i; i32::from_ne_bytes(v.to_ne_bytes()) }) < (i32::from_ne_bytes(((u32::try_from((a.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) { (a[usize::try_from(i).unwrap_or(0)]).clone() } else { UString::from("<missing>") }).as_ustr(); __s }).as_str()));
                if i32::from_ne_bytes(((u32::try_from((diffs.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) >= 8 {
                    diffs.push(UString::from("  …").to_ustring());
                    break;
                }
            }
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("golden mismatch for fixture '")); __s += id; __s += &(UString::from(concat!("':\n",
""))); __s += UString::from(format!("{}", { let joined3 = diffs; let mut out = String::new(); let n = joined3.len(); let mut index3 = 0usize; while index3 < n { if index3 > 0 { out.push_str(concat!("\n",
"")); } let _ = write!(out, "{}", joined3[index3]); index3 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s }).as_str());
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
        return ClreqProfile::new((p.id).to_ustring().as_ustr(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((p.auto_space).clone()), Some(p.glue_placement), (p.adjustment).clone(), KinsokuMode::Fixed { level: KinsokuLevel::Basic, hanging: HangingPunctuationStyle::Disabled }, (p.punctuation_width).clone());
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
        return ClreqProfile::new((p.id).to_ustring().as_ustr(), p.strictness, p.region, Some(p.punctuation_glyph_policy), Some(crate::org::tiqian::clreq::clreq_profile::CLREQ_PROFILE_DEFAULT_COALESCE_REPEATABLE_PUNCTUATION.to_vec()), Some((p.auto_space).clone()), Some(p.glue_placement), (p.adjustment).clone(), KinsokuMode::Fixed { level: KinsokuLevel::Basic, hanging: HangingPunctuationStyle::Disabled }, (p.punctuation_width).clone());
    }
}
