use crate::org::tiqian::core::decoration_kind::DecorationKind;
use crate::org::tiqian::core::decoration_span::DecorationSpan;
use crate::org::tiqian::core::layout_constraints::LayoutConstraints;
use crate::org::tiqian::core::line_break_policy::LineBreakPolicy;
use crate::org::tiqian::core::line_break_span::LineBreakSpan;
use crate::org::tiqian::core::line_length_grid::LineLengthGrid;
use crate::org::tiqian::core::ruby_kind::RubyKind;
use crate::org::tiqian::core::ruby_line_height_mode::RubyLineHeightMode;
use crate::org::tiqian::core::ruby_span::RubySpan;
use crate::org::tiqian::core::text_range::TextRange;
use std::fmt::Write;
use std::sync::LazyLock;


#[derive(Debug, Clone, PartialEq)]
pub struct LayoutFixture {
    pub id: String,
    pub text: String,
    pub constraints: LayoutConstraints,
    pub notes: String,
    pub line_height: Option<f64>,
    pub decorations: Vec<DecorationSpan>,
    pub ruby_spans: Vec<RubySpan>,
    pub ruby_line_height_mode: RubyLineHeightMode,
    pub first_line_indent_em: Option<f64>,
    pub pin_basic_no_hang: bool,
    pub use_english_hyphenation: bool,
    pub line_length_grid: LineLengthGrid,
    pub line_break_spans: Vec<LineBreakSpan>,
}

impl LayoutFixture {
    pub fn new(id: &str, text: &str, constraints: LayoutConstraints, notes: &str, line_height: Option<f64>, decorations: Option<Vec<DecorationSpan>>, ruby_spans: Option<Vec<RubySpan>>, ruby_line_height_mode: Option<RubyLineHeightMode>, first_line_indent_em: Option<f64>,
pin_basic_no_hang: Option<bool>, use_english_hyphenation: Option<bool>, line_length_grid: Option<LineLengthGrid>, line_break_spans: Option<Vec<LineBreakSpan>>) -> Self {
        let decorations = decorations.unwrap_or_else(|| vec![]);
        let ruby_spans = ruby_spans.unwrap_or_else(|| vec![]);
        let ruby_line_height_mode = ruby_line_height_mode.unwrap_or_else(|| RubyLineHeightMode::PerLine);
        let pin_basic_no_hang = pin_basic_no_hang.unwrap_or_else(|| false);
        let use_english_hyphenation = use_english_hyphenation.unwrap_or_else(|| false);
        let line_length_grid = line_length_grid.unwrap_or_else(|| LineLengthGrid::new(Some(true), None));
        let line_break_spans = line_break_spans.unwrap_or_else(|| vec![]);
        Self {
            id: id.to_string(),
            text: text.to_string(),
            constraints,
            notes: notes.to_string(),
            line_height,
            decorations: decorations,
            ruby_spans: ruby_spans,
            ruby_line_height_mode: ruby_line_height_mode,
            first_line_indent_em,
            pin_basic_no_hang: pin_basic_no_hang,
            use_english_hyphenation: use_english_hyphenation,
            line_length_grid: line_length_grid,
            line_break_spans: line_break_spans,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "LayoutFixture(",
            "id=",
            (self.id).to_string(),
            ", ",
            "text=",
            (self.text).to_string(),
            ", ",
            "constraints=",
            (self.constraints).clone().to_string(),
            ", ",
            "notes=",
            (self.notes).to_string(),
            ", ",
            "lineHeight=",
            match self.line_height { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "decorations=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.decorations).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "rubySpans=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.ruby_spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ", ",
            "rubyLineHeightMode=",
            self.ruby_line_height_mode.name(),
            ", ",
            "firstLineIndentEm=",
            match self.first_line_indent_em { Some(v) => crate::runtime::fp_helper::FPHelper::format_float(v), None => "null".to_string() },
            ", ",
            "pinBasicNoHang=",
            self.pin_basic_no_hang,
            ", ",
            "useEnglishHyphenation=",
            self.use_english_hyphenation,
            ", ",
            "lineLengthGrid=",
            (self.line_length_grid).clone().to_string(),
            ", ",
            "lineBreakSpans=",
            {
        let mut out = String::new();
        out.push('[');
        let arr = (self.line_break_spans).clone();
        let n = arr.len();
        let mut i = 0usize;
        while i < n {
            if i > 0 { out.push_str(", "); }
            let _ = write!(out, "{}", arr[i].to_string());
            i += 1;
        }
        out.push(']');
        out
    },
            ")"
        );
    }
}

pub static EARLY_LAYOUT_FIXTURES_ALL: LazyLock<Vec<LayoutFixture>> = LazyLock::new(|| vec![
    (LayoutFixture::new("basic-pause-stop", "中文，中文。", LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Covers pause/stop punctuation glue.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false),
Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("ellipsis-and-dash", "中文……English——中文。", LayoutConstraints::new(220 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Covers CJK ellipsis and dash fallback decisions.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("nested-quotes", "他说：“你好，世界。”", LayoutConstraints::new(180 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Covers opening/closing punctuation and repair planning.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("adjacent-punctuation-spacing", "他说：“你好，世界。”！！", LayoutConstraints::new(220 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Shows punctuation atoms and adjacent punctuation spacing compression.", None, Some(vec![]), Some(vec![]),
Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("contextual-curly-quotes", "中‘that’s’中’，‘", LayoutConstraints::new(192 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"NonCjkInWordApostrophe keeps the apostrophe in that’s on the Western run while the surrounding single quotes retain CJK punctuation geometry; the trailing ’，‘ sequence exercises both adjacent-punctuation compression boundaries.", None, Some(vec![]), Some(vec![]),
Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mixed-script-quote-paragraph-language", "“Json是谁？”", LayoutConstraints::new(192 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"A quote-only Chinese paragraph begins with a Latin identifier; full content evidence is mixed, so ParagraphLanguageQuoteContext keeps the pair on CJK punctuation geometry.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false),
Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("adjacent-curly-quote-list-context", "中文“对A”“波霸”；中文“欧派”“double”“double may”呢", LayoutConstraints::new(320 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"PairedPunctuationOuterScriptContext evaluates ordinary text at the enclosing level and excludes every quoted sibling, so Latin content in one item cannot switch a following CJK-context quote pair to proportional Latin geometry.", None, Some(vec![]), Some(vec![]),
Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mi10s-adjacent-curly-quote-wrap", "所以这个和 “骑ji” “说shui”“斜xiá”不一样，港台是从众的，大陆读音大多数源自韵书。", LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Mi 10s dogfood regression: PairedPunctuationOuterScriptContext gives the adjacent ‘斜xiá’ pair its enclosing Chinese prose context, while Uax14WesternPunctuationBoundary keeps Western closing/opening punctuation attached even when a shared code point uses a Latin face.", None,
Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mi10s-western-bracket-citation-wrap", "史力军,姚晨,杨国玉,等.常见有机化合物中文词汇的读音详解[J].化学教育(中英文), 2023, 44(10):21-38.", LayoutConstraints::new(272 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Mi 10s dogfood regression: WesternBracketCjkInterChar lets proportional ASCII parentheses touching Chinese share tier-3 equal expansion without changing their Latin face; BibliographicNumericLocatorBreak exposes clean volume(issue):page-range boundaries while each digit run and the page range remain intact.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(false), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("bibliographic-numeric-locator-break", "中文中文中文44(10):21-38.", LayoutConstraints::new(224 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"BibliographicNumericLocatorBreak lets the preceding Chinese line take 44(10): and wraps before the intact page range 21-38.; the volume and issue digit runs remain cohesive and no synthetic hyphen is added.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(false), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("unmatched-curly-quotes", "’90s James’； “truncated；中文“未闭", LayoutConstraints::new(240 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"UnmatchedQuoteSurroundingScriptContext keeps leading elisions, trailing possessives, and spaced truncated Latin quotations proportional while an unspaced truncated quote in Chinese remains CJK punctuation.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("fallback-roles", "提椠……Hello——世界。", LayoutConstraints::new(240 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Covers cluster font role classification for CJK text, CJK punctuation, and Latin words.", None, Some(vec![]), Some(vec![]),
Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("greedy-multi-line", "咖啡馆比咖啡更早地改变了城里人的作息与谈吐。", LayoutConstraints::new(144 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Exercises greedy multi-line breaking with width tight enough to trigger several breaks.", None, Some(vec![]),
Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("kinsoku-carry-previous", "提椠中文中文中文。", LayoutConstraints::new(64 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Forces a kinsoku CarryPrevious repair: greedy break would put 。 at line start, so the engine pulls the preceding character down.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(true), Some(false), Some(LineLengthGrid::new(Some(true),
None)), Some(vec![]))).clone(),
    (LayoutFixture::new("kinsoku-push-in", "中文中。", LayoutConstraints::new(60 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Forces PushIn: greedy would put 。 at line start, then line-end punctuation glue shrinks enough to keep it on the previous line.", None,
Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(true), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("lookahead-future-push-in", "中文中文中文。", LayoutConstraints::new(60 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Forces a PushIn repair inside lookahead's future lines; lookahead should score that cheap repair instead of adding an earlier break.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(true), Some(false),
Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("lookahead-avoids-repair", "中文中文中文。", LayoutConstraints::new(48 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"At width 48 greedy ends up with a CarryPrevious repair on the last line; lookahead shifts the first break earlier to avoid the conflict entirely.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(true), Some(false),
Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("justify-cjk-paragraph", "中文中文中文中文中文中文", LayoutConstraints::new(100 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "Justification fills the small deficit on the first line by adding CjkInterChar glue between adjacent CJK clusters.", None,
Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("justify-mixed-paragraph", "中文Hello中文，世界。", LayoutConstraints::new(144 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Justification uses CjkLatinSpace at the CJK↔Latin boundary plus PunctuationGlue if a spacing reduction landed on the line.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)),
Some(vec![]))).clone(),
    (LayoutFixture::new("justify-unbreakable-number-symbol", "中文50℃中文中文中文Example", LayoutConstraints::new(128 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"CLREQ stretch prohibition: the inseparable 50|℃ boundary stays closed while other legal gaps justify the line.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(false), None)),
Some(vec![]))).clone(),
    (LayoutFixture::new("ascii-brackets-in-cjk", "中文段落(English)和[mixed]说明。", LayoutConstraints::new(240 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ASCII (, ), [, ] do not share code points with the CJK fullwidth forms （）【】, so they always classify as Latin. (English) and [mixed] cluster as Latin runs and render in latin-primary even when surrounded by CJK content.", None, Some(vec![]), Some(vec![]),
Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("ascii-point-mark-in-cjk", "中文中文,中文", LayoutConstraints::new(64 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"AttachedAsciiPointMarkKinsoku: a directly attached ASCII comma keeps its Latin face and proportional advance, but cannot start a wrapped Chinese line. The structured contextual-kinsoku decision explains the exception without creating CJK punctuation glue.", None, Some(vec![]),
Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(true), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("ascii-point-mark-impossible-measure", "中,文", LayoutConstraints::new(15 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"AttachedAsciiPointMarkImpossibleMeasureHang: when even the preceding character plus its attached Latin comma cannot fit, the applied contextual fallback hangs the comma instead of accepting it at line start. The ordinary profile hanging policy remains disabled.", None,
Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(true), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("real-paragraph-1", "咖啡（coffee）在十七世纪经威尼斯传入欧洲。最初它被当作药物出售，价格高得吓人，真正让它流行起来的是随后遍地开花的咖啡馆——读报、辩论、下棋、写作——城市生活忽然多出一个公共客厅。意大利人做出了 espresso，维也纳人往杯里加奶油，土耳其人坚持连渣同煮……每座城市都相信自己手里那一杯才是正统。有人说：「先有咖啡馆，后有启蒙运动」。这话说得夸张，但也不算太离谱。", LayoutConstraints::new(320 as f64 as f64,
Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Real-text stress test: ~200 chars of authentic Chinese with Latin words, fullwidth/halfwidth brackets, em-dash pair, ellipsis, Chinese quotes, and multiple comma-stop sequences. Triggers multi-line greedy + justification + adjacent punctuation compression simultaneously. Uses the standard 2em 段首缩进 like real body text.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(2 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("latin-word-wrap", "他引用了一句话：The quick brown fox jumps over the lazy dog，然后继续讲。", LayoutConstraints::new(240 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"LatinWordSegmentation: the English sentence wraps at word boundaries instead of overflowing as one unbreakable cluster; spaces collapse at line edges; word spaces stretch under justify.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64),
Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("emphasis-marks", "他强调：豆子新鲜最要紧，烘焙其次。", LayoutConstraints::new(128 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"CLREQ emphasis dots (着重号): span covers 豆子新鲜最要紧，烘焙其次 including the comma — Han text gets a dot anchor, punctuation is skipped per CLREQ. Narrow measure wraps the span across lines; lineHeight 25.6px (1.6×16) leaves room for the dots below the em box.", Some(25.6f64),
Some(vec![(DecorationSpan::new(TextRange::new(4u32, 16u32).unwrap(), DecorationKind::Emphasis)).clone()]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("ruby-line-height", "甲乙丙丁戊己庚辛壬癸子丑", LayoutConstraints::new(64 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ConditionalRubyLineHeight: an 18px line leaves only 2px above the 16px base face, so the 8px pinyin box adds the 6px deficit only before its annotated line in the default PerLine mode.", Some(18 as f64), Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(4u32, 5u32).unwrap(), "wù", Some(vec![]), RubyKind::Pinyin, None)).clone(),
]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("bopomofo-tone-em-box", "好", LayoutConstraints::new(64 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"BopomofoToneSharedAnnotationEmSizing: the ordinary tone mark shares the 0.3em annotation size; its 5×5 slot only positions it, and glyph ink does not rescale it.", None, Some(vec![]), Some(vec![
    (RubySpan::new(TextRange::new(0u32, 1u32).unwrap(), "ㄏㄠˇ", Some(vec![]), RubyKind::Bopomofo, Some("zh-TW".to_string()))).clone(),
]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("first-line-indent", "咖啡的风味因产地而各异，烘焙的深浅同样会改变口感与香气。", LayoutConstraints::new(200 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"段首缩进: first line indents 2em (CLREQ standard) — its usable measure shrinks to maxWidth-2em and the LineBox carries the indent; later lines use the full measure. Justify targets the indented measure on line 0.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(2 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("latin-camelcase", "用PowerPoint做", LayoutConstraints::new(128 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"CamelCaseBreak: a camelCase token wraps at its hump (Power|Point) with NO hyphen — the capital signals the break. All-caps abbreviations (NASA) and single Title-case words are NOT treated this way.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as
f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("latin-existing-hyphen", "out-of-the-way", LayoutConstraints::new(128 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ExistingHyphenBreak (CY/T 154-2017 §9.3): a hyphenated compound wraps AT its existing '-' (no new hyphen, no synthetic 短横线 atom). Keeps ≥2 letters each side (§9.4).", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false),
Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("latin-hard-break", "中Network", LayoutConstraints::new(64 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"LatinForcedHyphenBreak (ADR 0029): with NO hyphenator (default), an over-long Latin word still hard-breaks at character boundaries with a hanging hyphen, keeping 前二后三 — 'Ne' head, 'ork' tail.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64),
Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("latin-opaque-url-token", "链接 https://example.com/path/to/abc123def456ghi789", LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"LatinOpaqueTokenBreak (ADR 0029): URL / identifier-like Latin runs break at clean separator or character boundaries without adding a synthetic hyphen.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false),
Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("zero-width-space-soft-break", "A.​.​.Complete？AaFont？", LayoutConstraints::new(96 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"UAX #14 ZW: U+200B is a source-faithful zero-width soft break control. It is not shaped, never creates a blank visual line, and does not weaken the visible-zero-advance capability guard.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64),
Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("western-hyphenation", "请运行 internationalization 命令", LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"LineEndHangingHyphen (ADR 0029): the long English word is split at en-US syllable points so it wraps inside the measure; a hyphen is reserved inside the line when possible, and only hangs when it cannot fit. The 'hyphen=' line tag marks where. Needs the injected English hyphenator.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(true), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("progressive-technical-inline", "中文 internationalization 命令", LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ProgressiveTechnicalBreak: semantic link/code text uses structural, letter-digit, camel, then syllable boundaries without a displayed hyphen. Ordinary paragraph opportunities remain first; only an otherwise unfillable auto-wrapped technical line uses explicit grapheme tracking.",
None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(true), Some(LineLengthGrid::new(Some(true), None)), Some(vec![
    (LineBreakSpan::new(TextRange::new(3u32, 23u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]))).clone(),
    (LayoutFixture::new("progressive-technical-hash-fill", "deadbeefcafebabefeedfaceabcdefabcdef", LayoutConstraints::new(173 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ExplicitEmergencyGraphemeTracking: a standalone technical hash skips syllable classification, hard-breaks at source graphemes, and exactly fills every non-last line. The final line remains naturally aligned.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(0 as f64), Some(false), Some(true), Some(LineLengthGrid::new(Some(false), None)), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 36u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]))).clone(),
    (LayoutFixture::new("progressive-technical-alpha-numeric", "Machine2Machine", LayoutConstraints::new(76 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"TechnicalAlphaNumericTransitionBreak: the real-font report selects Machine2|Machine at the letter-digit structural boundary; the cut stays clean and adds no hyphen.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(true),
Some(LineLengthGrid::new(Some(false), None)), Some(vec![
    (LineBreakSpan::new(TextRange::new(0u32, 15u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]))).clone(),
    (LayoutFixture::new("progressive-technical-current-line-emergency", "Swift 这边是我最有体感的。JSONDecoder 慢是个老问题，SR-6252[36] 那个 issue 里挖出的根因是底层走 NSJSONSerialization 再桥接回 Objective-C，swift_dynamicCast 吃掉大量时间。", LayoutConstraints::new(579 as f64 as f64, Some(f64::INFINITY),
Some(2147483647)).unwrap(),
"CurrentLineTechnicalTierRejection: a technical token is reconsidered against the current line's stretch, regardless of whether the complete token fits a full measure. A clean tier that needs unbounded tracking is rejected; the hierarchy continues to a rightmost Emergency cut before terminal technical tracking is allowed.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(false), None)), Some(vec![
    (LineBreakSpan::new(TextRange::new(16u32, 27u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(67u32, 86u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
    (LineBreakSpan::new(TextRange::new(104u32, 121u32).unwrap(), LineBreakPolicy::ProgressiveTechnical)).clone(),
]))).clone(),
    (LayoutFixture::new("adaptive-short-line-indent", "提椠是一个面向中文正文的排版引擎", LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"MeasureAdaptiveFirstLineIndent: with no explicit indent and a short measure (10 字 < 14), the段首缩进 default narrows to 1 字 (not 2). The firstindent decision line records measure/threshold/source.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), None,
Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mandatory-single-newline", concat!("第一行\n",
"第二行"), LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "ADR 0037: a single source newline is a mandatory break, zero-width and unshaped.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false),
Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mandatory-blank-lines", concat!("甲\n",
"\n",
"乙\n",
""), LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "ADR 0037: consecutive and trailing mandatory breaks preserve blank lines.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false),
Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mandatory-leading-trailing-newline", concat!("\n",
"开头和结尾\n",
""), LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "ADR 0037: leading and trailing mandatory breaks produce visible empty lines.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false),
Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mandatory-crlf", concat!("甲\r\n",
"乙"), LayoutConstraints::new(160 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "ADR 0037: CRLF is one mandatory break cluster, not two blank lines.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false),
Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mandatory-wraps-long-line", concat!("中文中文中文中文中文\n",
"尾行"), LayoutConstraints::new(64 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(), "ADR 0037: long source lines still auto-wrap before the mandatory break; mandatory-break lines are not justified.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine),
Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("indent-opening-quote", "“好咖啡要趁热喝。”他说完便把杯子推了过来，让大家依次尝一口。", LayoutConstraints::new(192 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"段首缩进 composed with an opening quote at paragraph start: the additive model's line-start leading-glue trim halves the quote (CLREQ 缩减该符号始侧二分之一个汉字大小的空白) — visual blank before the quote ink is exactly the 2em indent.", None, Some(vec![]), Some(vec![]),
Some(RubyLineHeightMode::PerLine), Some(2 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("line-end-kinsoku", "中文中文（中文）中文", LayoutConstraints::new(80 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"CLREQ 行尾禁则 (Basic): 开括号不得居行尾. maxWidth 80 (5字) would end line 0 on （ — the break retreats so （ starts line 1 (CarryNext, cascade-free). Pinned Fixed(Basic) so the measure doesn't auto-escalate.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64),
Some(true), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("interlinear-lines", "屈原写下离骚，顾炎武王夫之并称。", LayoutConstraints::new(224 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"行间线 (ADR 0024): 专名号 underlines 屈原/顾炎武/王夫之, 书名号甲式 wavy line under 离骚. 顾炎武 and 王夫之 are adjacent — AdjacentInterlinearLineShortening pulls each adjacent edge back 1/16em so the two marks read separately. No explicit lineHeight: InterlinearMarkLineSpacingFloor raises the line height to 1.5em.", None, Some(vec![
    (DecorationSpan::new(TextRange::new(0u32, 2u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(4u32, 6u32).unwrap(), DecorationKind::BookTitle)).clone(),
    (DecorationSpan::new(TextRange::new(7u32, 10u32).unwrap(), DecorationKind::ProperNoun)).clone(),
    (DecorationSpan::new(TextRange::new(10u32, 13u32).unwrap(), DecorationKind::ProperNoun)).clone(),
]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("mourning-frame", "悼念：王小明同志、张大同同志。", LayoutConstraints::new(72 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"示亡号 (mourning frames) around 王小明 and 张大同. maxWidth 72 would naturally break inside 王小明 — MourningSpanKeptUnbroken moves the break to the span start instead. Frame rects hug the font-declared character face; the InterlinearMarkLineSpacingFloor (0.5em) keeps frames clear of neighbouring lines without an explicit lineHeight.", None, Some(vec![
    (DecorationSpan::new(TextRange::new(3u32, 6u32).unwrap(), DecorationKind::Mourning)).clone(),
    (DecorationSpan::new(TextRange::new(9u32, 12u32).unwrap(), DecorationKind::Mourning)).clone(),
]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("contextual-dash-ellipsis", concat!("中文—下句；等…真。 English — next; ellipsis… / slash. A——B; Wait……what? 中文—English\n",
"——中文\n",
"……"), LayoutConstraints::new(1024 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ContextualDashEllipsisRoleResolution uses surrounding strong script, not mark count: single CJK marks retain CJK geometry while repeated Western marks stay on the Latin face in their own clusters (ContextualDashEllipsisRunSegmentation). The tail exercises conflicting-surrounding-script (中文—English), mandatory-break truncation with only-right evidence (——中文), and the no-context paragraph-language fallback (……).", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("parenthetical-dash-pairs", "他彻夜想Jessica——Jessica是他的前女友——睡不着觉。地点——北京，时间——明天。", LayoutConstraints::new(1024 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"ParentheticalDashPairContext: the Jessica insertion pair resolves jointly from the outer context (conflict -> paragraph language -> CJK two-em dashes) even though the first run sits between Latin words; the second sentence's runs are separated by a comma, stay independent, and resolve from their own surroundings.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
    (LayoutFixture::new("quote-digit-boundaries", "中文 le“t”ters 中1“1”2文；中Ａ“Ｂ”Ｃ文。尾号是“1‘2’3”，用时1’30”。", LayoutConstraints::new(1024 as f64 as f64, Some(f64::INFINITY), Some(2147483647)).unwrap(),
"Non-CJK word-internal quote boundaries: le“t”ters keeps NonCjkWordInternalQuotePair on the Latin face; digit (中1“1”2文) and fullwidth (中Ａ“Ｂ”Ｃ文) boundaries stay excluded and resolve CJK; the digit-bounded single pair in “1‘2’3” inherits the enclosing CJK quotation; the unmatched marks in 1’30” resolve as NumericPrimeUnmatchedQuote on the Latin face.", None, Some(vec![]), Some(vec![]), Some(RubyLineHeightMode::PerLine), Some(0 as f64), Some(false), Some(false), Some(LineLengthGrid::new(Some(true), None)), Some(vec![]))).clone(),
]);

#[derive(Clone, Copy)]
pub struct EarlyLayoutFixtures;

impl EarlyLayoutFixtures {
}
