use crate::org::tiqian::core::layout_result::LayoutResult;
use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::layout::plan_lowering::PlanLowering;
use crate::org::tiqian::protocol::plan_json::PlanJson;
use crate::org::tiqian::protocol::plan_json_number::PlanJsonNumber;
use crate::org::tiqian::protocol::plan_packed::PlanPacked;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphToPreparedParagraphJsonFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    TextRangeErrorFault(crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError),
}
impl std::fmt::Display for PreparedParagraphToPreparedParagraphJsonFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphToPreparedParagraphJsonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphToPreparedParagraphJsonFault> for crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError {
    fn from(value: PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        match value {
            PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError> for PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError) -> Self {
        PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreparedParagraphToPlanWithDiagnosticsJsonFault {
    UStringFaultFault(crate::std::u_string_exception::UStringFault),
    ToPreparedParagraphJsonFault(crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault),
}
impl std::fmt::Display for PreparedParagraphToPlanWithDiagnosticsJsonFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(value) => write!(formatter, "{}", value),
            PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(value) => write!(formatter, "{}", value),
        }
    }
}

impl From<PreparedParagraphToPlanWithDiagnosticsJsonFault> for crate::std::u_string_exception::UStringFault {
    fn from(value: PreparedParagraphToPlanWithDiagnosticsJsonFault) -> Self {
        match value {
            PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<PreparedParagraphToPlanWithDiagnosticsJsonFault> for crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault {
    fn from(value: PreparedParagraphToPlanWithDiagnosticsJsonFault) -> Self {
        match value {
            PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(value) => value,
            _ => panic!("fault union converted to an unrelated fault"),
        }
    }
}

impl From<crate::std::u_string_exception::UStringFault> for PreparedParagraphToPlanWithDiagnosticsJsonFault {
    fn from(value: crate::std::u_string_exception::UStringFault) -> Self {
        PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(value)
    }
}

impl From<crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault> for PreparedParagraphToPlanWithDiagnosticsJsonFault {
    fn from(value: crate::org::tiqian::layout::prepared_paragraph::PreparedParagraphToPreparedParagraphJsonFault) -> Self {
        PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(value)
    }
}

#[derive(Clone, Copy)]
pub struct PreparedParagraphFns;

impl PreparedParagraphFns {
    pub fn prepared_paragraph_fns_to_prepared_paragraph_json(result: LayoutResult, render_evidence: bool) -> Result<UString, PreparedParagraphToPreparedParagraphJsonFault> {
        let plan = PlanLowering::plan_lowering_to_plan((result).clone(), render_evidence).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::TextRangeErrorFault(e))?;
        return Ok(PlanJson::plan_json_encode((plan).clone()).map_err(|e| PreparedParagraphToPreparedParagraphJsonFault::UStringFaultFault(e))?);
    }

    pub fn prepared_paragraph_fns_to_plan_with_diagnostics_json(result: LayoutResult, render_evidence: bool, zero_advance_epsilon_px: f64) -> Result<UString, PreparedParagraphToPlanWithDiagnosticsJsonFault> {
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("{\"plan\":").is_empty() {
                if !UString::from("{\"plan\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
        }
        out.extend(UString::from("{\"plan\":").encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(&mut out, PreparedParagraphFns::prepared_paragraph_fns_to_prepared_paragraph_json((result).clone(), render_evidence).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::ToPreparedParagraphJsonFault(e))?.as_ustr()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"diagnostics\":{\"capabilityIssues\":[").is_empty() {
                if !UString::from(",\"diagnostics\":{\"capabilityIssues\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
        }
        out.extend(UString::from(",\"diagnostics\":{\"capabilityIssues\":[").encode_utf16());
        let mut first = true;
        let shaping_diag_src = ((result.debug).clone().shaping_decisions).clone();
        for d in &shaping_diag_src {
            match &((d.capability_issue).clone()) {
                Some(__option) => {
                    if !first {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("{\"name\":").is_empty() {
                            if !UString::from("{\"name\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from("{\"name\":").encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(&mut out, (*__option).clone().as_ustr()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"reason\":").is_empty() {
                            if !UString::from(",\"reason\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from(",\"reason\":").encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(&mut out, (d.reason).to_ustring().as_ustr()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"rangeStart\":").is_empty() {
                            if !UString::from(",\"rangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from(",\"rangeStart\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().start)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().start)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"rangeEnd\":").is_empty() {
                            if !UString::from(",\"rangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from(",\"rangeEnd\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().end)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().end)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                            if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from("}").encode_utf16());
                }
                None => {
                }
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("],\"advanceSuspects\":[").is_empty() {
                if !UString::from("],\"advanceSuspects\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
        }
        out.extend(UString::from("],\"advanceSuspects\":[").encode_utf16());
        first = true;
        for d in &shaping_diag_src {
            if !((d.advance).is_finite() && (d.advance) > (zero_advance_epsilon_px)) {
                if !first {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                            }
                        }
                    }
                    out.extend(UString::from(",").encode_utf16());
                }
                first = false;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("{\"displayText\":").is_empty() {
                        if !UString::from("{\"displayText\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from("{\"displayText\":").encode_utf16());
                let _ = PlanJsonNumber::plan_json_number_append_json_string(&mut out, (d.display_text).to_ustring().as_ustr()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"advance\":\"").is_empty() {
                        if !UString::from(",\"advance\":\"").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from(",\"advance\":\"").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !if d.advance.is_finite() { PlanJsonNumber::plan_json_number_ecma_json_number(d.advance).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?.to_ustring() } else { UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(d.advance)).as_str()) }.is_empty() {
                        if !if d.advance.is_finite() { PlanJsonNumber::plan_json_number_ecma_json_number(d.advance).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?.to_ustring() } else { UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(d.advance)).as_str()) }.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(if d.advance.is_finite() { PlanJsonNumber::plan_json_number_ecma_json_number(d.advance).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?.to_ustring() } else { UString::from(format!("{}", crate::runtime::fp_helper::FPHelper::format_float(d.advance)).as_str()) }.encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("\",\"reason\":").is_empty() {
                        if !UString::from("\",\"reason\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from("\",\"reason\":").encode_utf16());
                let _ = PlanJsonNumber::plan_json_number_append_json_string(&mut out, (d.reason).to_ustring().as_ustr()).map_err(|e| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(e))?;
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"rangeStart\":").is_empty() {
                        if !UString::from(",\"rangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from(",\"rangeStart\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().start)).as_str()).is_empty() {
                        if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().start)).as_str()).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"rangeEnd\":").is_empty() {
                        if !UString::from(",\"rangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from(",\"rangeEnd\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().end)).as_str()).is_empty() {
                        if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text((d.range).clone().end)).as_str()).encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                        if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                        }
                    }
                }
                out.extend(UString::from("}").encode_utf16());
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]}}").is_empty() {
                if !UString::from("]}}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(unit) }));
                }
            }
        }
        out.extend(UString::from("]}}").encode_utf16());
        return Ok(UString::from_utf16(out.as_slice()).map_err(|_| PreparedParagraphToPlanWithDiagnosticsJsonFault::UStringFaultFault(UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) }))?);
    }

    pub fn prepared_paragraph_fns_to_packed_plan_bytes(result: LayoutResult) -> Result<Vec<u8>, TextRangeError> {
        let plan = PlanLowering::plan_lowering_to_plan((result).clone(), false)?;
        return Ok(PlanPacked::plan_packed_encode((plan).clone()));
    }
}
