use crate::org::tiqian::core::cluster::Cluster;
use crate::org::tiqian::core::text_range::TextRange;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::line_optimization::PushInAllocation;
use crate::org::tiqian::layout::line_optimization::RepairCandidate;
use crate::org::tiqian::layout::line_optimization::RepairOption;
use crate::org::tiqian::test::trace::test_trace_render::TestTraceRender;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct PushInLineWideCapacityTestSupport;

impl PushInLineWideCapacityTestSupport {
    pub fn push_in_line_wide_capacity_test_support_cluster(s: u32, e: u32, text: &UStr, a: f64) -> Result<Cluster, TextRangeError> {
        return Ok(Cluster::new(TextRange::new(s, e)?, text, &(UStr::new(&[116,101,115,116])), a, Some(text.to_ustring()), Some(0.0), Some(0.0), Some(0.0)));
    }

    pub fn push_in_line_wide_capacity_test_support_render_nullable_candidate(v: Option<RepairCandidate>) -> Result<UString, UStringFault> {
        return Ok(match &(v) { None => UString::from("null"), Some(__option) => PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_render_candidate(((*__option).clone()).clone())?.to_ustring() });
    }

    pub fn push_in_line_wide_capacity_test_support_render_candidate(v: RepairCandidate) -> Result<UString, UStringFault> {
        return Ok(TestTraceRender::test_trace_render_cap(PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_render_candidate_text((v).clone()).as_ustr())?);
    }

    pub(crate) fn push_in_line_wide_capacity_test_support_render_candidate_text(v: RepairCandidate) -> UString {
        return UString::from(format!("{}", v.to_string()).as_str());
    }

    pub(crate) fn push_in_line_wide_capacity_test_support_allocation_parts(values: &Vec<PushInAllocation>) -> Vec<UString> {
        let mut parts: Vec<UString> = vec![];
        for value in values {
            parts.push(UString::from(format!("{}", value.to_string()).as_str()));
        }
        return parts;
    }

    pub fn push_in_line_wide_capacity_test_support_push_in_string(o: RepairOption) -> UString {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4,
total_available_capacity: _p5 } => { let mut __s = UString::new(); __s += &(UString::from("PushIn(penalty=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(_p0)).as_str())); __s += &(UString::from(", reason=")); __s += _p1.as_ustr(); __s += &(UString::from(", offenderClusterIndex=")); __s += &(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(_p2)).as_str())); __s += &(UString::from(", allocations=")); __s += TestTraceRender::test_trace_render_legacy_list_text(&PushInLineWideCapacityTestSupport::push_in_line_wide_capacity_test_support_allocation_parts(&_p3)).as_ustr(); __s += &(UString::from(", totalShrink=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(_p4)); __s += &(UString::from(", totalAvailableCapacity=")); __s += &(crate::runtime::fp_helper::FPHelper::format_float(_p5)); __s += &(UString::from(")")); __s },
            RepairOption::Hang { .. } => UString::from(format!("{}", o.to_string()).as_str()),
            RepairOption::CarryPrevious { .. } => UString::from(format!("{}", o.to_string()).as_str()),
            RepairOption::CarryNext { .. } => UString::from(format!("{}", o.to_string()).as_str()),
            RepairOption::LeaveRagged { .. } => UString::from(format!("{}", o.to_string()).as_str()),
        };
    }

    pub fn push_in_line_wide_capacity_test_support_push_in_allocations(o: RepairOption) -> Vec<PushInAllocation> {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, .. } => _p3,
            RepairOption::Hang { .. } => vec![],
            RepairOption::CarryPrevious { .. } => vec![],
            RepairOption::CarryNext { .. } => vec![],
            RepairOption::LeaveRagged { .. } => vec![],
        };
    }

    pub fn push_in_line_wide_capacity_test_support_push_in_offender_cluster_index(o: RepairOption) -> u32 {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, .. } => _p2,
            RepairOption::Hang { .. } => 4294967295u32,
            RepairOption::CarryPrevious { .. } => 4294967295u32,
            RepairOption::CarryNext { .. } => 4294967295u32,
            RepairOption::LeaveRagged { .. } => 4294967295u32,
        };
    }

    pub fn push_in_line_wide_capacity_test_support_push_in_total_shrink(o: RepairOption) -> f64 {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, .. } => _p4,
            RepairOption::Hang { .. } => 0 as f64,
            RepairOption::CarryPrevious { .. } => 0 as f64,
            RepairOption::CarryNext { .. } => 0 as f64,
            RepairOption::LeaveRagged { .. } => 0 as f64,
        };
    }

    pub fn push_in_line_wide_capacity_test_support_push_in_total_available_capacity(o: RepairOption) -> f64 {
        return match o {
            RepairOption::PushIn { penalty: _p0, reason: _p1, offender_cluster_index: _p2, allocations: _p3, total_shrink: _p4, total_available_capacity: _p5 } => _p5,
            RepairOption::Hang { .. } => 0 as f64,
            RepairOption::CarryPrevious { .. } => 0 as f64,
            RepairOption::CarryNext { .. } => 0 as f64,
            RepairOption::LeaveRagged { .. } => 0 as f64,
        };
    }

    pub fn push_in_line_wide_capacity_test_support_is_push_in(o: RepairOption) -> bool {
        return match o {
            RepairOption::PushIn { .. } => true,
            RepairOption::Hang { .. } => false,
            RepairOption::CarryPrevious { .. } => false,
            RepairOption::CarryNext { .. } => false,
            RepairOption::LeaveRagged { .. } => false,
        };
    }

    pub fn push_in_line_wide_capacity_test_support_is_carry_previous(o: RepairOption) -> bool {
        return match o {
            RepairOption::PushIn { .. } => false,
            RepairOption::Hang { .. } => false,
            RepairOption::CarryPrevious { .. } => true,
            RepairOption::CarryNext { .. } => false,
            RepairOption::LeaveRagged { .. } => false,
        };
    }

    pub fn push_in_line_wide_capacity_test_support_is_leave_ragged(o: RepairOption) -> bool {
        return match o {
            RepairOption::PushIn { .. } => false,
            RepairOption::Hang { .. } => false,
            RepairOption::CarryPrevious { .. } => false,
            RepairOption::CarryNext { .. } => false,
            RepairOption::LeaveRagged { .. } => true,
        };
    }
}
