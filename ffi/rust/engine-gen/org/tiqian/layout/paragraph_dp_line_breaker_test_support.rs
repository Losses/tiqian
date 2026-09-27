use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::kinsoku_rule::ClreqKinsokuRule;
use crate::org::tiqian::layout::line_optimization::LineSolution;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::layout::line_optimization::RepairOptions;
use crate::org::tiqian::layout::paragraph_dp_line_breaker::ParagraphDpLineBreaker;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::ProgressiveBreakTier;
use crate::org::tiqian::layout::progressive_break_decisions::ShrinkOpportunity;
use crate::org::tiqian::layout::progressive_break_decisions::UnbreakableRanges;
use crate::org::tiqian::test::trace::test_trace_recorder::TestTraceRecorder;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertions;
use crate::org::tiqian::test::trace::traced_assertions::TracedAssertionsFailFault;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedMapTableBuilder;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedSetTableBuilder;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use std::fmt::Write;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct ParagraphDpLineBreakerTestSupport;

impl ParagraphDpLineBreakerTestSupport {
    pub fn paragraph_dp_line_breaker_test_support_cluster(i: u32, text: Option<UString>, advance: Option<f64>) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(i, u32::wrapping_add(i, 1))?, match &(text) { None => UString::from("中"), Some(__option) => __option.to_ustring() }.as_ustr(), &(UStr::new(&[116,101,115,116])), match &(advance) { None => 16.0f64, Some(__option1) => *__option1 }, Some(match &(text) { None => UString::from("中"), Some(__option2) => __option2.to_ustring() }), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn paragraph_dp_line_breaker_test_support_han(n: u32, advance: Option<f64>) -> Result<Vec<Cluster>, TextRangeError> {
        let mut a: Vec<Cluster> = vec![];
        for i in 0..n {
            a.push(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(i, Some(UString::from("中")), advance)?);
        }
        return Ok(a);
    }

    pub fn paragraph_dp_line_breaker_test_support_latin() -> Result<Vec<Cluster>, TextRangeError> {
        return Ok(vec![
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(0, Some(UString::from("a")), Some(30 as f64 as f64))?).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(1, Some(UString::from("/")), Some(30 as f64 as f64))?).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(2, Some(UString::from("b")), Some(25 as f64 as f64))?).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(3, Some(UString::from("c")), Some(30 as f64 as f64))?).clone(),
    (ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(4, Some(UString::from("d")), Some(30 as f64 as f64))?).clone(),
]);
    }

    pub fn paragraph_dp_line_breaker_test_support_ints(v: &Vec<u32>) -> SortedSetTable<u32> {
        let mut b: SortedSetTableBuilder<u32> = SortedTable::sorted_table_set_builder::<u32>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for &x in v {
            b.put(&(x));
        }
        return b.clone().build();
    }

    pub fn paragraph_dp_line_breaker_test_support_opportunities(v: &Vec<u32>, spans: &Vec<TextRange>, tiers: &Vec<ProgressiveBreakTier>) -> SortedMapTable<u32, ProgressiveBreakOpportunity> {
        let mut b: SortedMapTableBuilder<u32, ProgressiveBreakOpportunity> = SortedTable::sorted_table_map_builder::<u32,
ProgressiveBreakOpportunity>(Arc::new(|a, b| SortedTable::sorted_table_compare_ints(i32::from_ne_bytes(((*a) as i32).to_ne_bytes()), i32::from_ne_bytes(((*b) as i32).to_ne_bytes()))));
        for i in 0..match u32::try_from(v.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            b.put(&(v[usize::try_from(i).unwrap_or(0)]), &(ProgressiveBreakOpportunity::new(tiers[usize::try_from(i).unwrap_or(0)], (spans[usize::try_from(i).unwrap_or(0)]).clone(), Some(0.0))));
        }
        return b.clone().build();
    }

    pub fn paragraph_dp_line_breaker_test_support_solve(c: &Vec<Cluster>, width: f64, shrink: Option<Vec<ShrinkOpportunity>>, hard: Option<Vec<u32>>, push: Option<bool>, ranges: Option<UnbreakableRanges>, progressive: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>, window: Option<u32>, cjk: Option<Vec<u32>>, max_stretch: Option<f64>, forbid_start: Option<Vec<u32>>) -> Result<LineSolution, TextRangeError> {
        let x = ParagraphDpLineBreaker::new(Some(match &(window) { None => 8, Some(__option10) => *__option10 }), Some(0.5), Some(Box::new(ClreqKinsokuRule::new(Some(KinsokuLevel::Basic)))), Some(2), Some(10), Some(20), Some(12 as f64), Some(12 as f64), Some(3 as f64), Some(1 as f64))?;
        return Ok(x.break_lines(&c, &c, width, (shrink).clone(), (ranges).clone(), None, None, None, match &(forbid_start) { None => None, Some(__option26) => Some(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_ints(__option26)) }, None, None, Some(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_ints(&match &(cjk) { None => vec![], Some(__option29) => (*__option29).clone() })), Some(match &(max_stretch) { None => f64::INFINITY, Some(__option30) => *__option30 }), None, None, push, None, Some(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_ints(&match &(hard) { None => vec![], Some(__option32) => (*__option32).clone() })), None, (progressive).clone())?);
    }

    pub fn paragraph_dp_line_breaker_test_support_solve_test_defaults(c: &Vec<Cluster>, width: f64, shrink: Option<Vec<ShrinkOpportunity>>, hard: Option<Vec<u32>>, push: Option<bool>, ranges: Option<UnbreakableRanges>, progressive: Option<SortedMapTable<u32, ProgressiveBreakOpportunity>>, forbid_start: Option<Vec<u32>>) -> Result<LineSolution, TextRangeError> {
        let mut boundaries: Vec<u32> = vec![];
        for i in 1..match u32::try_from(c.len()) { Ok(value) => value, Err(_) => u32::MAX } {
            boundaries.push(i);
        }
        return Ok(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_solve(&c, width, (shrink).clone(), (hard).clone(), push, (ranges).clone(), (progressive).clone(), None, Some((boundaries).clone()), Some(8.0f64), (forbid_start).clone())?);
    }

    pub fn paragraph_dp_line_breaker_test_support_tiles(s: LineSolution, n: u32) -> Result<(), TracedAssertionsFailFault> {
        let mut e = 0u32;
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                if i32::from_ne_bytes((((l.cluster_range).clone().start) as i32).to_ne_bytes()) <= i32::from_ne_bytes((((l.cluster_range).clone().end) as i32).to_ne_bytes()) {
                    let _ = TracedAssertions::traced_assertions_assert_equals_int(e, (l.cluster_range).clone().start, Some(UString::from("lines must tile clusters in order")))?;
                    e = u32::wrapping_add((l.cluster_range).clone().end, 1);
                }
            }
        }
        let _ = TracedAssertions::traced_assertions_assert_equals_int(n, e, Some(UString::from("lines must cover every cluster")))?;
        Ok(())
    }

    pub fn paragraph_dp_line_breaker_test_support_repairs_string(s: LineSolution) -> UString {
        let mut parts: Vec<UString> = vec![];
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                parts.push(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_render_repair((l.repair).clone()));
            }
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined = parts; let mut out = String::new(); let n = joined.len(); let mut index = 0usize; while index < n { if index > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined[index]); index += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn paragraph_dp_line_breaker_test_support_render_repair(r: Option<RepairOption>) -> UString {
        return match &(r) { None => UString::from("null"), Some(__option33) => ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_render_repair_option((*__option33).clone()).to_ustring() };
    }

    pub(crate) fn paragraph_dp_line_breaker_test_support_render_repair_option(r: RepairOption) -> UString {
        return UString::from(format!("{}", r.to_string()).as_str());
    }

    pub fn paragraph_dp_line_breaker_test_support_lines_string(s: LineSolution) -> UString {
        let mut parts: Vec<UString> = vec![];
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                parts.push(UString::from(format!("{}", l.to_string()).as_str()));
            }
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined1 = parts; let mut out = String::new(); let n = joined1.len(); let mut index1 = 0usize; while index1 < n { if index1 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined1[index1]); index1 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn paragraph_dp_line_breaker_test_support_push_in_reason(r: Option<RepairOption>) -> Option<UString> {
        return match &(r) { None => None, Some(__option34) => RepairOptions::repair_options_push_in_reason_of((*__option34).clone()) };
    }

    pub fn paragraph_dp_line_breaker_test_support_ranges_string(s: LineSolution) -> UString {
        let mut parts: Vec<UString> = vec![];
        {
            let _g1 = s.lines.clone();
            for l in &_g1 {
                parts.push(UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((l.cluster_range).clone().start)).as_str())); __s += &(UString::from("..")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((l.cluster_range).clone().end)).as_str())); __s }).as_str()));
            }
        }
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("[")); __s += UString::from(format!("{}", { let joined2 = parts; let mut out = String::new(); let n = joined2.len(); let mut index2 = 0usize; while index2 < n { if index2 > 0 { out.push_str(", "); } let _ = write!(out, "{}", joined2[index2]); index2 += 1; } UString::from(out.as_str()) }).as_str()).as_ustr(); __s += &(UString::from("]")); __s }).as_str());
    }

    pub fn paragraph_dp_line_breaker_test_support_rec(n: &UStr) {
        TestTraceRecorder::new(&(UStr::new(&[80,97,114,97,103,114,97,112,104,68,112,76,105,110,101,66,114,101,97,107,101,114,84,101,115,116]))).section(n);
    }

    pub fn paragraph_dp_line_breaker_test_support_c(i: u32, t: &UStr, a: f64) -> Result<Cluster, TextRangeError> {
        return Ok(ParagraphDpLineBreakerTestSupport::paragraph_dp_line_breaker_test_support_cluster(i, Some((t).to_ustring()), Some(a))?);
    }
}
