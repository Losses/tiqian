use crate::org::tiqian::protocol::plan::Plan;
use crate::org::tiqian::protocol::plan::PlanCell;
use crate::org::tiqian::protocol::plan::PlanLine;
use crate::org::tiqian::protocol::plan_end_reason::PlanEndReason;
use crate::org::tiqian::protocol::plan_json_number::PlanJsonNumber;
use crate::org::tiqian::protocol::plan_schema::PlanSchema;
use crate::org::tiqian::protocol::plan_style_delta::PlanStyleDelta;
use crate::runtime::u_string::UString;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct PlanJson;

impl PlanJson {
    pub fn plan_json_encode(plan: Plan) -> Result<UString, UStringFault> {
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("{").is_empty() {
                if !UString::from("{").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("{").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("\"schema\":").is_empty() {
                if !UString::from("\"schema\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("\"schema\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(PlanSchema::PLAN_SCHEMA_PLAN_SCHEMA)).as_str()).is_empty() {
                if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(PlanSchema::PLAN_SCHEMA_PLAN_SCHEMA)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(PlanSchema::PLAN_SCHEMA_PLAN_SCHEMA)).as_str()).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"layoutRevision\":\"").is_empty() {
                if !UString::from(",\"layoutRevision\":\"").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"layoutRevision\":\"").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanSchema::PLAN_SCHEMA_PLAN_LAYOUT_REVISION.to_ustring().is_empty() {
                if !PlanSchema::PLAN_SCHEMA_PLAN_LAYOUT_REVISION.to_ustring().encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanSchema::PLAN_SCHEMA_PLAN_LAYOUT_REVISION.to_ustring().encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("\",\"width\":").is_empty() {
                if !UString::from("\",\"width\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("\",\"width\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(plan.width)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(plan.width)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(plan.width)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"height\":").is_empty() {
                if !UString::from(",\"height\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"height\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(plan.height)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(plan.height)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(plan.height)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"lines\":[").is_empty() {
                if !UString::from(",\"lines\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"lines\":[").encode_utf16());
        let mut first_line = true;
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                if !first_line {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",").encode_utf16());
                }
                first_line = false;
                let _ = PlanJson::plan_json_append_line(&mut out, (line).clone())?;
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("]").encode_utf16());
        let _ = PlanJson::plan_json_append_paragraph_evidence(&mut out, (plan).clone())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("}").encode_utf16());
        return Ok(UString::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?);
    }

    pub(crate) fn plan_json_append_line(out: &mut Vec<u16>, line: PlanLine) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("{\"rangeStart\":").is_empty() {
                if !UString::from("{\"rangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("{\"rangeStart\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line.range_start)).as_str()).is_empty() {
                if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line.range_start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line.range_start)).as_str()).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"rangeEnd\":").is_empty() {
                if !UString::from(",\"rangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"rangeEnd\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line.range_end)).as_str()).is_empty() {
                if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line.range_end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(line.range_end)).as_str()).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"top\":").is_empty() {
                if !UString::from(",\"top\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"top\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.top)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(line.top)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.top)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"bottom\":").is_empty() {
                if !UString::from(",\"bottom\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"bottom\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.bottom)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(line.bottom)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.bottom)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"baseline\":").is_empty() {
                if !UString::from(",\"baseline\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"baseline\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.baseline)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(line.baseline)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.baseline)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"indent\":").is_empty() {
                if !UString::from(",\"indent\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"indent\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.indent)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(line.indent)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.indent)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"visualWidth\":").is_empty() {
                if !UString::from(",\"visualWidth\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"visualWidth\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.visual_width)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(line.visual_width)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.visual_width)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"hyphenAdvance\":").is_empty() {
                if !UString::from(",\"hyphenAdvance\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"hyphenAdvance\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.hyphen_advance)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(line.hyphen_advance)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.hyphen_advance)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"endReason\":").is_empty() {
                if !UString::from(",\"endReason\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"endReason\":").encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, PlanJson::plan_json_end_reason_name(line.end_reason).as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"cells\":[").is_empty() {
                if !UString::from(",\"cells\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"cells\":[").encode_utf16());
        let mut first_cell = true;
        {
            let _g1 = line.cells.clone();
            for cell in &_g1 {
                if !first_cell {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",").encode_utf16());
                }
                first_cell = false;
                let _ = PlanJson::plan_json_append_cell(out, (cell).clone())?;
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("]}").is_empty() {
                if !UString::from("]}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("]}").encode_utf16());
        Ok(())
    }

    pub(crate) fn plan_json_append_cell(out: &mut Vec<u16>, cell: PlanCell) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("{\"rangeStart\":").is_empty() {
                if !UString::from("{\"rangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("{\"rangeStart\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cell.range_start)).as_str()).is_empty() {
                if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cell.range_start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cell.range_start)).as_str()).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"rangeEnd\":").is_empty() {
                if !UString::from(",\"rangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"rangeEnd\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cell.range_end)).as_str()).is_empty() {
                if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cell.range_end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(cell.range_end)).as_str()).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"source\":").is_empty() {
                if !UString::from(",\"source\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"source\":").encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (cell.source).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"display\":").is_empty() {
                if !UString::from(",\"display\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"display\":").encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (cell.display).to_ustring().as_ustr())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"drawX\":").is_empty() {
                if !UString::from(",\"drawX\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"drawX\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(cell.draw_x)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(cell.draw_x)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(cell.draw_x)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"naturalWidth\":").is_empty() {
                if !UString::from(",\"naturalWidth\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"naturalWidth\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(cell.natural_width)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(cell.natural_width)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(cell.natural_width)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"leadingLayoutAdvance\":").is_empty() {
                if !UString::from(",\"leadingLayoutAdvance\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"leadingLayoutAdvance\":").encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(cell.leading_layout_advance)?.is_empty() {
                if !PlanJsonNumber::plan_json_number_ecma_json_number(cell.leading_layout_advance)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(cell.leading_layout_advance)?.encode_utf16());
        if cell.shaping_boundary {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"shapingBoundary\":true").is_empty() {
                    if !UString::from(",\"shapingBoundary\":true").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"shapingBoundary\":true").encode_utf16());
        }
        if i32::from_ne_bytes(((u32::try_from((cell.open_type_features.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"openTypeFeatures\":[").is_empty() {
                    if !UString::from(",\"openTypeFeatures\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"openTypeFeatures\":[").encode_utf16());
            let mut first_feature = true;
            {
                let _g1 = cell.open_type_features.clone();
                for feature in &_g1 {
                    if !first_feature {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_feature = false;
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, feature.as_ustr())?;
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        let _ = PlanJson::plan_json_append_cell_evidence(out, (cell).clone())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("}").encode_utf16());
        Ok(())
    }

    pub(crate) fn plan_json_append_cell_evidence(out: &mut Vec<u16>, cell: PlanCell) -> Result<(), UStringFault> {
        let inline_object = cell.inline_object;
        match &(inline_object) {
            Some(__option) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"inlineObject\":").is_empty() {
                        if !UString::from(",\"inlineObject\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"inlineObject\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option)?.is_empty() {
                        if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option)?.encode_utf16());
            }
            None => {
            }
        }
        let advance = cell.advance;
        match &(advance) {
            Some(__option1) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"advance\":").is_empty() {
                        if !UString::from(",\"advance\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"advance\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option1)?.is_empty() {
                        if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option1)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option1)?.encode_utf16());
            }
            None => {
            }
        }
        let render_font_family = cell.render_font_family.clone();
        match &(render_font_family) {
            Some(__option2) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"renderFontFamily\":").is_empty() {
                        if !UString::from(",\"renderFontFamily\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"renderFontFamily\":").encode_utf16());
                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option2.as_ustr())?;
            }
            None => {
            }
        }
        let dash_strategy = cell.dash_strategy.clone();
        match &(dash_strategy) {
            Some(__option3) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"dashStrategy\":").is_empty() {
                        if !UString::from(",\"dashStrategy\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"dashStrategy\":").encode_utf16());
                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option3.as_ustr())?;
                let shaping_language = cell.shaping_language.clone();
                match &(shaping_language) {
                    Some(__option4) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"shapingLanguage\":").is_empty() {
                                if !UString::from(",\"shapingLanguage\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"shapingLanguage\":").encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option4.as_ustr())?;
                    }
                    None => {
                    }
                }
                let resolved_face = cell.resolved_face.clone();
                match &(resolved_face) {
                    Some(__option5) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"resolvedFace\":").is_empty() {
                                if !UString::from(",\"resolvedFace\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"resolvedFace\":").encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option5.as_ustr())?;
                    }
                    None => {
                    }
                }
                let glyph_ids = cell.glyph_ids.clone();
                match &(glyph_ids) {
                    Some(__option6) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"glyphIds\":").is_empty() {
                                if !UString::from(",\"glyphIds\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"glyphIds\":").encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option6.as_ustr())?;
                    }
                    None => {
                    }
                }
                let shaping_evidence = cell.shaping_evidence.clone();
                match &(shaping_evidence) {
                    Some(__option7) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"shapingEvidence\":").is_empty() {
                                if !UString::from(",\"shapingEvidence\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"shapingEvidence\":").encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option7.as_ustr())?;
                    }
                    None => {
                    }
                }
            }
            None => {
            }
        }
        let punctuation_ink_floor = cell.punctuation_ink_floor;
        match &(punctuation_ink_floor) {
            Some(__option8) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"punctuationInkFloor\":").is_empty() {
                        if !UString::from(",\"punctuationInkFloor\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"punctuationInkFloor\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option8)?.is_empty() {
                        if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option8)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option8)?.encode_utf16());
                let punctuation_body_width = cell.punctuation_body_width;
                match &(punctuation_body_width) {
                    Some(__option9) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"punctuationBodyWidth\":").is_empty() {
                                if !UString::from(",\"punctuationBodyWidth\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"punctuationBodyWidth\":").encode_utf16());
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option9)?.is_empty() {
                                if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option9)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option9)?.encode_utf16());
                    }
                    None => {
                    }
                }
            }
            None => {
            }
        }
        if cell.latin {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"latin\":true").is_empty() {
                    if !UString::from(",\"latin\":true").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"latin\":true").encode_utf16());
        }
        let style_delta = cell.style_delta;
        match &(style_delta) {
            Some(__option10) => {
                let _ = PlanJson::plan_json_append_style_delta(out, (*__option10).clone())?;
            }
            None => {
            }
        }
        Ok(())
    }

    pub(crate) fn plan_json_append_style_delta(out: &mut Vec<u16>, style: PlanStyleDelta) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from(",\"style\":{").is_empty() {
                if !UString::from(",\"style\":{").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from(",\"style\":{").encode_utf16());
        let mut field_count = 0u32;
        let font_size = style.font_size;
        match &(font_size) {
            Some(__option11) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("\"fontSize\":").is_empty() {
                        if !UString::from("\"fontSize\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from("\"fontSize\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option11)?.is_empty() {
                        if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option11)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option11)?.encode_utf16());
                field_count = u32::wrapping_add(field_count, 1);
            }
            None => {
            }
        }
        let font_weight = style.font_weight;
        match &(font_weight) {
            Some(__option12) => {
                if i32::from_ne_bytes(((field_count) as i32).to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",").encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("\"fontWeight\":").is_empty() {
                        if !UString::from("\"fontWeight\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from("\"fontWeight\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(*__option12)).as_str()).is_empty() {
                        if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(*__option12)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(*__option12)).as_str()).encode_utf16());
                field_count = u32::wrapping_add(field_count, 1);
            }
            None => {
            }
        }
        let italic = style.italic;
        match &(italic) {
            Some(__option13) => {
                if i32::from_ne_bytes(((field_count) as i32).to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",").encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from("\"italic\":").is_empty() {
                        if !UString::from("\"italic\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from("\"italic\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !if *__option13.unwrap_or(false) { UString::from("true") } else { UString::from("false") }.is_empty() {
                        if !if *__option13.unwrap_or(false) { UString::from("true") } else { UString::from("false") }.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(if *__option13.unwrap_or(false) { UString::from("true") } else { UString::from("false") }.encode_utf16());
            }
            None => {
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
        }
        out.extend(UString::from("}").encode_utf16());
        Ok(())
    }

    pub(crate) fn plan_json_append_paragraph_evidence(out: &mut Vec<u16>, plan: Plan) -> Result<(), UStringFault> {
        let font_size = plan.font_size;
        match &(font_size) {
            Some(__option14) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"fontSize\":").is_empty() {
                        if !UString::from(",\"fontSize\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"fontSize\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option14)?.is_empty() {
                        if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option14)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option14)?.encode_utf16());
            }
            None => {
            }
        }
        let overlay_width = plan.overlay_width;
        match &(overlay_width) {
            Some(__option15) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !UString::from(",\"overlayWidth\":").is_empty() {
                        if !UString::from(",\"overlayWidth\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(UString::from(",\"overlayWidth\":").encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option15)?.is_empty() {
                        if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option15)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option15)?.encode_utf16());
            }
            None => {
            }
        }
        if i32::from_ne_bytes(((u32::try_from((plan.emphasis_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"emphasisRanges\":[").is_empty() {
                    if !UString::from(",\"emphasisRanges\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"emphasisRanges\":[").encode_utf16());
            let mut first_range = true;
            {
                let _g1 = plan.emphasis_ranges.clone();
                for range in &_g1 {
                    if !first_range {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_range = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("[").is_empty() {
                            if !UString::from("[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("[").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(range.start)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(range.start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(range.start)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(range.end)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(range.end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(range.end)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                            if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("]").encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        if i32::from_ne_bytes(((u32::try_from((plan.inline_edges.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"inlineEdges\":[").is_empty() {
                    if !UString::from(",\"inlineEdges\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"inlineEdges\":[").encode_utf16());
            let mut first_edge = true;
            {
                let _g1 = plan.inline_edges.clone();
                for edge in &_g1 {
                    if !first_edge {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_edge = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("{\"offset\":").is_empty() {
                            if !UString::from("{\"offset\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("{\"offset\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(edge.offset)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(edge.offset)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(edge.offset)).as_str()).encode_utf16());
                    let inline_start = edge.inline_start;
                    match &(inline_start) {
                        Some(__option16) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"inlineStart\":").is_empty() {
                                    if !UString::from(",\"inlineStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"inlineStart\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option16)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option16)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option16)?.encode_utf16());
                        }
                        None => {
                        }
                    }
                    let inline_end = edge.inline_end;
                    match &(inline_end) {
                        Some(__option17) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"inlineEnd\":").is_empty() {
                                    if !UString::from(",\"inlineEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"inlineEnd\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option17)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option17)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option17)?.encode_utf16());
                        }
                        None => {
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                            if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("}").encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        if i32::from_ne_bytes(((u32::try_from((plan.ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"rubyDecisions\":[").is_empty() {
                    if !UString::from(",\"rubyDecisions\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"rubyDecisions\":[").encode_utf16());
            let mut first_ruby = true;
            {
                let _g1 = plan.ruby_decisions.clone();
                for ruby in &_g1 {
                    if !first_ruby {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_ruby = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("{\"baseRangeStart\":").is_empty() {
                            if !UString::from("{\"baseRangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("{\"baseRangeStart\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.base_range_start)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.base_range_start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.base_range_start)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"baseRangeEnd\":").is_empty() {
                            if !UString::from(",\"baseRangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"baseRangeEnd\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.base_range_end)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.base_range_end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.base_range_end)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"text\":").is_empty() {
                            if !UString::from(",\"text\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"text\":").encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (ruby.text).to_ustring().as_ustr())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"centerX\":").is_empty() {
                            if !UString::from(",\"centerX\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"centerX\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.center_x)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.center_x)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(ruby.center_x)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"baselineY\":").is_empty() {
                            if !UString::from(",\"baselineY\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"baselineY\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.baseline_y)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.baseline_y)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(ruby.baseline_y)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"fontSize\":").is_empty() {
                            if !UString::from(",\"fontSize\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"fontSize\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.font_size)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.font_size)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(ruby.font_size)?.encode_utf16());
                    let ascent = ruby.ascent;
                    match &(ascent) {
                        Some(__option18) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"ascent\":").is_empty() {
                                    if !UString::from(",\"ascent\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"ascent\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option18)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option18)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option18)?.encode_utf16());
                        }
                        None => {
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"fontWeight\":").is_empty() {
                            if !UString::from(",\"fontWeight\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"fontWeight\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.font_weight)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.font_weight)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(ruby.font_weight)).as_str()).encode_utf16());
                    if i32::from_ne_bytes(((u32::try_from(((ruby.font_families).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"fontFamilies\":[").is_empty() {
                                if !UString::from(",\"fontFamilies\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"fontFamilies\":[").encode_utf16());
                        let mut first_family = true;
                        {
                            let mut _g = 0u32;
                            let _g1 = (ruby.font_families).clone().clone();
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let family = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if !first_family {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                            }
                                        }
                                    }
                                    out.extend(UString::from(",").encode_utf16());
                                }
                                first_family = false;
                                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, family.as_ustr())?;
                            }
                        }
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from("]").encode_utf16());
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                            if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("}").encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        if i32::from_ne_bytes(((u32::try_from((plan.bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"bopomofoDecisions\":[").is_empty() {
                    if !UString::from(",\"bopomofoDecisions\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"bopomofoDecisions\":[").encode_utf16());
            let mut first_bopomofo = true;
            {
                let _g1 = plan.bopomofo_decisions.clone();
                for bopomofo in &_g1 {
                    if !first_bopomofo {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_bopomofo = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("{\"baseRangeStart\":").is_empty() {
                            if !UString::from("{\"baseRangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("{\"baseRangeStart\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.base_range_start)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.base_range_start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.base_range_start)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"baseRangeEnd\":").is_empty() {
                            if !UString::from(",\"baseRangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"baseRangeEnd\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.base_range_end)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.base_range_end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.base_range_end)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"text\":").is_empty() {
                            if !UString::from(",\"text\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"text\":").encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (bopomofo.text).to_ustring().as_ustr())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"fontWeight\":").is_empty() {
                            if !UString::from(",\"fontWeight\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"fontWeight\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.font_weight)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.font_weight)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(bopomofo.font_weight)).as_str()).encode_utf16());
                    if i32::from_ne_bytes(((u32::try_from(((bopomofo.font_families).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",\"fontFamilies\":[").is_empty() {
                                if !UString::from(",\"fontFamilies\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",\"fontFamilies\":[").encode_utf16());
                        let mut first_family = true;
                        {
                            let mut _g = 0u32;
                            let _g1 = (bopomofo.font_families).clone().clone();
                            while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                                let family = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if !first_family {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                            if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                            }
                                        }
                                    }
                                    out.extend(UString::from(",").encode_utf16());
                                }
                                first_family = false;
                                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, family.as_ustr())?;
                            }
                        }
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                                if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from("]").encode_utf16());
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"placements\":[").is_empty() {
                            if !UString::from(",\"placements\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"placements\":[").encode_utf16());
                    let mut first_placement = true;
                    {
                        let mut _g = 0u32;
                        let _g1 = (bopomofo.placements).clone().clone();
                        while (i32::from_ne_bytes(((_g) as i32).to_ne_bytes())) < (i32::from_ne_bytes(((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes())) {
                            let placement = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                            _g = u32::wrapping_add(_g, 1);
                            if !first_placement {
                                if let Some(&unit) = out.last() {
                                    if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                        if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                        }
                                    }
                                }
                                out.extend(UString::from(",").encode_utf16());
                            }
                            first_placement = false;
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from("{\"text\":").is_empty() {
                                    if !UString::from("{\"text\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from("{\"text\":").encode_utf16());
                            let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (placement.text).to_ustring().as_ustr())?;
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"left\":").is_empty() {
                                    if !UString::from(",\"left\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"left\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.left)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(placement.left)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.left)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"top\":").is_empty() {
                                    if !UString::from(",\"top\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"top\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.top)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(placement.top)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.top)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"width\":").is_empty() {
                                    if !UString::from(",\"width\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"width\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.width)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(placement.width)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.width)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"height\":").is_empty() {
                                    if !UString::from(",\"height\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"height\":").encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.height)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(placement.height)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.height)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from(",\"role\":").is_empty() {
                                    if !UString::from(",\"role\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from(",\"role\":").encode_utf16());
                            let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (placement.role).to_ustring().as_ustr())?;
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                                    if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from("}").encode_utf16());
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("]}").is_empty() {
                            if !UString::from("]}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("]}").encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        if i32::from_ne_bytes(((u32::try_from((plan.decoration_segments.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"decorationSegments\":[").is_empty() {
                    if !UString::from(",\"decorationSegments\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"decorationSegments\":[").encode_utf16());
            let mut first_segment = true;
            {
                let _g1 = plan.decoration_segments.clone();
                for seg in &_g1 {
                    if !first_segment {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_segment = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("{\"kind\":").is_empty() {
                            if !UString::from("{\"kind\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("{\"kind\":").encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (seg.kind).to_ustring().as_ustr())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"left\":").is_empty() {
                            if !UString::from(",\"left\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"left\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(seg.left)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(seg.left)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(seg.left)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"top\":").is_empty() {
                            if !UString::from(",\"top\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"top\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(seg.top)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(seg.top)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(seg.top)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"right\":").is_empty() {
                            if !UString::from(",\"right\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"right\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(seg.right)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(seg.right)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(seg.right)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"sourceRangeStart\":").is_empty() {
                            if !UString::from(",\"sourceRangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"sourceRangeStart\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(seg.source_range_start)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(seg.source_range_start)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(seg.source_range_start)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"sourceRangeEnd\":").is_empty() {
                            if !UString::from(",\"sourceRangeEnd\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"sourceRangeEnd\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(seg.source_range_end)).as_str()).is_empty() {
                            if !UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(seg.source_range_end)).as_str()).encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(format!("{}", crate::runtime::int_text::IntText::int_text(seg.source_range_end)).as_str()).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                            if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("}").encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        if i32::from_ne_bytes(((u32::try_from((plan.emphasis_dots.len()) & 0xFFFF_FFFF).unwrap_or(0)) as i32).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from(",\"emphasisDots\":[").is_empty() {
                    if !UString::from(",\"emphasisDots\":[").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from(",\"emphasisDots\":[").encode_utf16());
            let mut first_dot = true;
            {
                let _g1 = plan.emphasis_dots.clone();
                for dot in &_g1 {
                    if !first_dot {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !UString::from(",").is_empty() {
                                if !UString::from(",").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                        }
                        out.extend(UString::from(",").encode_utf16());
                    }
                    first_dot = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("{\"clusterRangeStart\":").is_empty() {
                            if !UString::from("{\"clusterRangeStart\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("{\"clusterRangeStart\":").encode_utf16());
                    let cluster_range_start = dot.cluster_range_start;
                    match &(cluster_range_start) {
                        Some(__option19) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option19)?.is_empty() {
                                    if !PlanJsonNumber::plan_json_number_ecma_json_number(*__option19)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option19)?.encode_utf16());
                        }
                        None => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !UString::from("null").is_empty() {
                                    if !UString::from("null").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                            }
                            out.extend(UString::from("null").encode_utf16());
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"anchorX\":").is_empty() {
                            if !UString::from(",\"anchorX\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"anchorX\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_x)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_x)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_x)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"anchorY\":").is_empty() {
                            if !UString::from(",\"anchorY\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"anchorY\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_y)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_y)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_y)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from(",\"dotDiameter\":").is_empty() {
                            if !UString::from(",\"dotDiameter\":").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from(",\"dotDiameter\":").encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(dot.dot_diameter)?.is_empty() {
                            if !PlanJsonNumber::plan_json_number_ecma_json_number(dot.dot_diameter)?.encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(dot.dot_diameter)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !UString::from("}").is_empty() {
                            if !UString::from("}").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                    }
                    out.extend(UString::from("}").encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !UString::from("]").is_empty() {
                    if !UString::from("]").encode_utf16().next().map_or(false, |head| head >= 56320 && head <= 57343) {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
            }
            out.extend(UString::from("]").encode_utf16());
        }
        Ok(())
    }

    pub(crate) fn plan_json_end_reason_name(reason: PlanEndReason) -> UString {
        return match reason {
            PlanEndReason::AutoWrap => UString::from("AutoWrap").to_ustring().to_ustring(),
            PlanEndReason::MandatoryBreak => UString::from("MandatoryBreak").to_ustring().to_ustring(),
            PlanEndReason::ParagraphEnd => UString::from("ParagraphEnd").to_ustring().to_ustring(),
        };
    }
}
