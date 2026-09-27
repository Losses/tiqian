use crate::org::tiqian::protocol::plan::Plan;
use crate::org::tiqian::protocol::plan::PlanCell;
use crate::org::tiqian::protocol::plan::PlanLine;
use crate::org::tiqian::protocol::plan_end_reason::PlanEndReason;
use crate::org::tiqian::protocol::plan_json_number::PlanJsonNumber;
use crate::org::tiqian::protocol::plan_schema::PlanSchema;
use crate::org::tiqian::protocol::plan_style_delta::PlanStyleDelta;
use crate::std::u_string_exception::UStringFault;


#[derive(Clone, Copy)]
pub struct PlanJson;

impl PlanJson {
    pub fn plan_json_encode(plan: Plan) -> Result<String, UStringFault> {
        let mut out = Vec::<u16>::new();
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"{".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("{".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"\"schema\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("\"schema\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(PlanSchema::PLAN_SCHEMA_PLAN_SCHEMA).is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(crate::runtime::int_text::IntText::int_text(PlanSchema::PLAN_SCHEMA_PLAN_SCHEMA).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"layoutRevision\":\"".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"layoutRevision\":\"".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanSchema::PLAN_SCHEMA_PLAN_LAYOUT_REVISION.to_string().is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanSchema::PLAN_SCHEMA_PLAN_LAYOUT_REVISION.to_string().encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"\",\"width\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("\",\"width\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(plan.width)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(plan.width)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"height\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"height\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(plan.height)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(plan.height)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"lines\":[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"lines\":[".encode_utf16());
        let mut first_line = true;
        {
            let _g1 = plan.lines.clone();
            for line in &_g1 {
                if !first_line {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                first_line = false;
                let _ = PlanJson::plan_json_append_line(&mut out, (line).clone())?;
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("]".encode_utf16());
        let _ = PlanJson::plan_json_append_paragraph_evidence(&mut out, (plan).clone())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("}".encode_utf16());
        return Ok(String::from_utf16(out.as_slice()).map_err(|_| UStringFault::UnpairedSurrogate { unit: u32::from(out[out.len() - 1]) })?);
    }

    pub(crate) fn plan_json_append_line(out: &mut Vec<u16>, line: PlanLine) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"{\"rangeStart\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("{\"rangeStart\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(line.range_start).is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(crate::runtime::int_text::IntText::int_text(line.range_start).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"rangeEnd\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"rangeEnd\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(line.range_end).is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(crate::runtime::int_text::IntText::int_text(line.range_end).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"top\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"top\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.top)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.top)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"bottom\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"bottom\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.bottom)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.bottom)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"baseline\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"baseline\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.baseline)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.baseline)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"indent\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"indent\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.indent)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.indent)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"visualWidth\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"visualWidth\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.visual_width)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.visual_width)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"hyphenAdvance\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"hyphenAdvance\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(line.hyphen_advance)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(line.hyphen_advance)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"endReason\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"endReason\":".encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, PlanJson::plan_json_end_reason_name(line.end_reason).as_str())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"cells\":[".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"cells\":[".encode_utf16());
        let mut first_cell = true;
        {
            let _g1 = line.cells.clone();
            for cell in &_g1 {
                if !first_cell {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                first_cell = false;
                let _ = PlanJson::plan_json_append_cell(out, (cell).clone())?;
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"]}".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("]}".encode_utf16());
        Ok(())
    }

    pub(crate) fn plan_json_append_cell(out: &mut Vec<u16>, cell: PlanCell) -> Result<(), UStringFault> {
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"{\"rangeStart\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("{\"rangeStart\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(cell.range_start).is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(crate::runtime::int_text::IntText::int_text(cell.range_start).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"rangeEnd\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"rangeEnd\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(cell.range_end).is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(crate::runtime::int_text::IntText::int_text(cell.range_end).encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"source\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"source\":".encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (cell.source).to_string().as_str())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"display\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"display\":".encode_utf16());
        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (cell.display).to_string().as_str())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"drawX\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"drawX\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(cell.draw_x)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(cell.draw_x)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"naturalWidth\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"naturalWidth\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(cell.natural_width)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(cell.natural_width)?.encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !",\"leadingLayoutAdvance\":".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"leadingLayoutAdvance\":".encode_utf16());
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(cell.leading_layout_advance)?.is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(cell.leading_layout_advance)?.encode_utf16());
        if cell.shaping_boundary {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"shapingBoundary\":true".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"shapingBoundary\":true".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from((cell.open_type_features.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"openTypeFeatures\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"openTypeFeatures\":[".encode_utf16());
            let mut first_feature = true;
            {
                let _g1 = cell.open_type_features.clone();
                for feature in &_g1 {
                    if !first_feature {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_feature = false;
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, feature.as_str())?;
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        let _ = PlanJson::plan_json_append_cell_evidence(out, (cell).clone())?;
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("}".encode_utf16());
        Ok(())
    }

    pub(crate) fn plan_json_append_cell_evidence(out: &mut Vec<u16>, cell: PlanCell) -> Result<(), UStringFault> {
        let inline_object = cell.inline_object;
        match &(inline_object) {
            Some(__option) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"inlineObject\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"inlineObject\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
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
                    if unit >= 55296 && unit <= 56319 && !",\"advance\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"advance\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option1)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
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
                    if unit >= 55296 && unit <= 56319 && !",\"renderFontFamily\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"renderFontFamily\":".encode_utf16());
                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option2.as_str())?;
            }
            None => {
            }
        }
        let dash_strategy = cell.dash_strategy.clone();
        match &(dash_strategy) {
            Some(__option3) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"dashStrategy\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"dashStrategy\":".encode_utf16());
                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option3.as_str())?;
                let shaping_language = cell.shaping_language.clone();
                match &(shaping_language) {
                    Some(__option4) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"shapingLanguage\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"shapingLanguage\":".encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option4.as_str())?;
                    }
                    None => {
                    }
                }
                let resolved_face = cell.resolved_face.clone();
                match &(resolved_face) {
                    Some(__option5) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"resolvedFace\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"resolvedFace\":".encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option5.as_str())?;
                    }
                    None => {
                    }
                }
                let glyph_ids = cell.glyph_ids.clone();
                match &(glyph_ids) {
                    Some(__option6) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"glyphIds\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"glyphIds\":".encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option6.as_str())?;
                    }
                    None => {
                    }
                }
                let shaping_evidence = cell.shaping_evidence.clone();
                match &(shaping_evidence) {
                    Some(__option7) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"shapingEvidence\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"shapingEvidence\":".encode_utf16());
                        let _ = PlanJsonNumber::plan_json_number_append_json_string(out, __option7.as_str())?;
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
                    if unit >= 55296 && unit <= 56319 && !",\"punctuationInkFloor\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"punctuationInkFloor\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option8)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option8)?.encode_utf16());
                let punctuation_body_width = cell.punctuation_body_width;
                match &(punctuation_body_width) {
                    Some(__option9) => {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"punctuationBodyWidth\":".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"punctuationBodyWidth\":".encode_utf16());
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option9)?.is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
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
                if unit >= 55296 && unit <= 56319 && !",\"latin\":true".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"latin\":true".encode_utf16());
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
            if unit >= 55296 && unit <= 56319 && !",\"style\":{".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend(",\"style\":{".encode_utf16());
        let mut field_count = 0u32;
        let font_size = style.font_size;
        match &(font_size) {
            Some(__option11) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\"fontSize\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\"fontSize\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option11)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
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
                if i32::from_ne_bytes((field_count).to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\"fontWeight\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\"fontWeight\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(*__option12).is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(crate::runtime::int_text::IntText::int_text(*__option12).encode_utf16());
                field_count = u32::wrapping_add(field_count, 1);
            }
            None => {
            }
        }
        let italic = style.italic;
        match &(italic) {
            Some(__option13) => {
                if i32::from_ne_bytes((field_count).to_ne_bytes()) > (0) {
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                }
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !"\"italic\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend("\"italic\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !if *__option13.unwrap_or(false) { "true".to_string() } else { "false".to_string() }.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(if *__option13.unwrap_or(false) { "true".to_string() } else { "false".to_string() }.encode_utf16());
            }
            None => {
            }
        }
        if let Some(&unit) = out.last() {
            if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
            }
        }
        out.extend("}".encode_utf16());
        Ok(())
    }

    pub(crate) fn plan_json_append_paragraph_evidence(out: &mut Vec<u16>, plan: Plan) -> Result<(), UStringFault> {
        let font_size = plan.font_size;
        match &(font_size) {
            Some(__option14) => {
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !",\"fontSize\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"fontSize\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option14)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
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
                    if unit >= 55296 && unit <= 56319 && !",\"overlayWidth\":".is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(",\"overlayWidth\":".encode_utf16());
                if let Some(&unit) = out.last() {
                    if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option15)?.is_empty() {
                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                    }
                }
                out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option15)?.encode_utf16());
            }
            None => {
            }
        }
        if i32::from_ne_bytes((u32::try_from((plan.emphasis_ranges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"emphasisRanges\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"emphasisRanges\":[".encode_utf16());
            let mut first_range = true;
            {
                let _g1 = plan.emphasis_ranges.clone();
                for range in &_g1 {
                    if !first_range {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_range = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"[".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("[".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(range.start).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(range.start).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(range.end).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(range.end).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("]".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from((plan.inline_edges.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"inlineEdges\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"inlineEdges\":[".encode_utf16());
            let mut first_edge = true;
            {
                let _g1 = plan.inline_edges.clone();
                for edge in &_g1 {
                    if !first_edge {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_edge = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"offset\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("{\"offset\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(edge.offset).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(edge.offset).encode_utf16());
                    let inline_start = edge.inline_start;
                    match &(inline_start) {
                        Some(__option16) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"inlineStart\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"inlineStart\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option16)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
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
                                if unit >= 55296 && unit <= 56319 && !",\"inlineEnd\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"inlineEnd\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option17)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option17)?.encode_utf16());
                        }
                        None => {
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("}".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from((plan.ruby_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"rubyDecisions\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"rubyDecisions\":[".encode_utf16());
            let mut first_ruby = true;
            {
                let _g1 = plan.ruby_decisions.clone();
                for ruby in &_g1 {
                    if !first_ruby {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_ruby = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"baseRangeStart\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("{\"baseRangeStart\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(ruby.base_range_start).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(ruby.base_range_start).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"baseRangeEnd\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"baseRangeEnd\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(ruby.base_range_end).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(ruby.base_range_end).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"text\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"text\":".encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (ruby.text).to_string().as_str())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"centerX\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"centerX\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.center_x)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(ruby.center_x)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"baselineY\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"baselineY\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.baseline_y)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(ruby.baseline_y)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"fontSize\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"fontSize\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(ruby.font_size)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(ruby.font_size)?.encode_utf16());
                    let ascent = ruby.ascent;
                    match &(ascent) {
                        Some(__option18) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"ascent\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"ascent\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option18)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option18)?.encode_utf16());
                        }
                        None => {
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"fontWeight\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"fontWeight\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(ruby.font_weight).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(ruby.font_weight).encode_utf16());
                    if i32::from_ne_bytes((u32::try_from(((ruby.font_families).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"fontFamilies\":[".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"fontFamilies\":[".encode_utf16());
                        let mut first_family = true;
                        {
                            let mut _g = 0u32;
                            let _g1 = (ruby.font_families).clone().clone();
                            while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                let family = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if !first_family {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                        }
                                    }
                                    out.extend(",".encode_utf16());
                                }
                                first_family = false;
                                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, family.as_str())?;
                            }
                        }
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend("]".encode_utf16());
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("}".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from((plan.bopomofo_decisions.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"bopomofoDecisions\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"bopomofoDecisions\":[".encode_utf16());
            let mut first_bopomofo = true;
            {
                let _g1 = plan.bopomofo_decisions.clone();
                for bopomofo in &_g1 {
                    if !first_bopomofo {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_bopomofo = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"baseRangeStart\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("{\"baseRangeStart\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(bopomofo.base_range_start).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(bopomofo.base_range_start).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"baseRangeEnd\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"baseRangeEnd\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(bopomofo.base_range_end).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(bopomofo.base_range_end).encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"text\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"text\":".encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (bopomofo.text).to_string().as_str())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"fontWeight\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"fontWeight\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !crate::runtime::int_text::IntText::int_text(bopomofo.font_weight).is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(crate::runtime::int_text::IntText::int_text(bopomofo.font_weight).encode_utf16());
                    if i32::from_ne_bytes((u32::try_from(((bopomofo.font_families).clone().len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",\"fontFamilies\":[".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",\"fontFamilies\":[".encode_utf16());
                        let mut first_family = true;
                        {
                            let mut _g = 0u32;
                            let _g1 = (bopomofo.font_families).clone().clone();
                            while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                                let family = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                                _g = u32::wrapping_add(_g, 1);
                                if !first_family {
                                    if let Some(&unit) = out.last() {
                                        if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                        }
                                    }
                                    out.extend(",".encode_utf16());
                                }
                                first_family = false;
                                let _ = PlanJsonNumber::plan_json_number_append_json_string(out, family.as_str())?;
                            }
                        }
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend("]".encode_utf16());
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"placements\":[".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"placements\":[".encode_utf16());
                    let mut first_placement = true;
                    {
                        let mut _g = 0u32;
                        let _g1 = (bopomofo.placements).clone().clone();
                        while (i32::from_ne_bytes((_g).to_ne_bytes())) < (i32::from_ne_bytes((u32::try_from((_g1.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes())) {
                            let placement = (_g1[usize::try_from(_g).unwrap_or(0)]).clone();
                            _g = u32::wrapping_add(_g, 1);
                            if !first_placement {
                                if let Some(&unit) = out.last() {
                                    if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                        return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                    }
                                }
                                out.extend(",".encode_utf16());
                            }
                            first_placement = false;
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !"{\"text\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend("{\"text\":".encode_utf16());
                            let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (placement.text).to_string().as_str())?;
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"left\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"left\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.left)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.left)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"top\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"top\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.top)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.top)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"width\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"width\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.width)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.width)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"height\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"height\":".encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(placement.height)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(placement.height)?.encode_utf16());
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !",\"role\":".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(",\"role\":".encode_utf16());
                            let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (placement.role).to_string().as_str())?;
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend("}".encode_utf16());
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"]}".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("]}".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from((plan.decoration_segments.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"decorationSegments\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"decorationSegments\":[".encode_utf16());
            let mut first_segment = true;
            {
                let _g1 = plan.decoration_segments.clone();
                for seg in &_g1 {
                    if !first_segment {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_segment = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"kind\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("{\"kind\":".encode_utf16());
                    let _ = PlanJsonNumber::plan_json_number_append_json_string(out, (seg.kind).to_string().as_str())?;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"left\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"left\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(seg.left)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(seg.left)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"top\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"top\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(seg.top)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(seg.top)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"right\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"right\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(seg.right)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(seg.right)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("}".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        if i32::from_ne_bytes((u32::try_from((plan.emphasis_dots.len()) & 0xFFFF_FFFF).unwrap_or(0)).to_ne_bytes()) > (0) {
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !",\"emphasisDots\":[".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend(",\"emphasisDots\":[".encode_utf16());
            let mut first_dot = true;
            {
                let _g1 = plan.emphasis_dots.clone();
                for dot in &_g1 {
                    if !first_dot {
                        if let Some(&unit) = out.last() {
                            if unit >= 55296 && unit <= 56319 && !",".is_empty() {
                                return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                            }
                        }
                        out.extend(",".encode_utf16());
                    }
                    first_dot = false;
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"{\"clusterRangeStart\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("{\"clusterRangeStart\":".encode_utf16());
                    let cluster_range_start = dot.cluster_range_start;
                    match &(cluster_range_start) {
                        Some(__option19) => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(*__option19)?.is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(*__option19)?.encode_utf16());
                        }
                        None => {
                            if let Some(&unit) = out.last() {
                                if unit >= 55296 && unit <= 56319 && !"null".is_empty() {
                                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                                }
                            }
                            out.extend("null".encode_utf16());
                        }
                    }
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"anchorX\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"anchorX\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_x)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_x)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"anchorY\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"anchorY\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_y)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(dot.anchor_y)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !",\"dotDiameter\":".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(",\"dotDiameter\":".encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !PlanJsonNumber::plan_json_number_ecma_json_number(dot.dot_diameter)?.is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend(PlanJsonNumber::plan_json_number_ecma_json_number(dot.dot_diameter)?.encode_utf16());
                    if let Some(&unit) = out.last() {
                        if unit >= 55296 && unit <= 56319 && !"}".is_empty() {
                            return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                        }
                    }
                    out.extend("}".encode_utf16());
                }
            }
            if let Some(&unit) = out.last() {
                if unit >= 55296 && unit <= 56319 && !"]".is_empty() {
                    return Err(UStringFault::UnpairedSurrogate { unit: u32::from(unit) });
                }
            }
            out.extend("]".encode_utf16());
        }
        Ok(())
    }

    pub(crate) fn plan_json_end_reason_name(reason: PlanEndReason) -> String {
        return match reason {
            PlanEndReason::AutoWrap => "AutoWrap".to_string().to_string(),
            PlanEndReason::MandatoryBreak => "MandatoryBreak".to_string().to_string(),
            PlanEndReason::ParagraphEnd => "ParagraphEnd".to_string().to_string(),
        };
    }
}
