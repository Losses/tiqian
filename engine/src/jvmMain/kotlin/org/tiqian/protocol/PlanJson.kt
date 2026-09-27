package org.tiqian.protocol

import std.UStringException

object PlanJson {
    fun encode(plan: Plan): String {
        val out = StringBuilder()
        val tail = out.lastOrNull()?.code ?: -1
        if (tail >= 55296 && tail <= 56319 && "{".length > 0 && !("{"[0].code >= 56320 && "{"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail)
        }
        out.append("{")
        val tail2 = out.lastOrNull()?.code ?: -1
        if (tail2 >= 55296 && tail2 <= 56319 && "\"schema\":".length > 0 && !("\"schema\":"[0].code >= 56320 && "\"schema\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail2)
        }
        out.append("\"schema\":")
        val tail3 = out.lastOrNull()?.code ?: -1
        if (tail3 >= 55296 && tail3 <= 56319 && ((PlanSchema.PLAN_SCHEMA).toString()).length > 0 && !(((PlanSchema.PLAN_SCHEMA).toString())[0].code >= 56320 && ((PlanSchema.PLAN_SCHEMA).toString())[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail3)
        }
        out.append(((PlanSchema.PLAN_SCHEMA).toString()))
        val tail4 = out.lastOrNull()?.code ?: -1
        if (tail4 >= 55296 && tail4 <= 56319 && ",\"layoutRevision\":\"".length > 0 && !(",\"layoutRevision\":\""[0].code >= 56320 && ",\"layoutRevision\":\""[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail4)
        }
        out.append(",\"layoutRevision\":\"")
        val tail5 = out.lastOrNull()?.code ?: -1
        if (tail5 >= 55296 && tail5 <= 56319 && (PlanSchema.PLAN_LAYOUT_REVISION).length > 0 && !((PlanSchema.PLAN_LAYOUT_REVISION)[0].code >= 56320 && (PlanSchema.PLAN_LAYOUT_REVISION)[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail5)
        }
        out.append((PlanSchema.PLAN_LAYOUT_REVISION))
        val tail6 = out.lastOrNull()?.code ?: -1
        if (tail6 >= 55296 && tail6 <= 56319 && "\",\"width\":".length > 0 && !("\",\"width\":"[0].code >= 56320 && "\",\"width\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail6)
        }
        out.append("\",\"width\":")
        val tail7 = out.lastOrNull()?.code ?: -1
        if (tail7 >= 55296 && tail7 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(plan.width)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(plan.width))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(plan.width))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail7)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(plan.width)))
        val tail8 = out.lastOrNull()?.code ?: -1
        if (tail8 >= 55296 && tail8 <= 56319 && ",\"height\":".length > 0 && !(",\"height\":"[0].code >= 56320 && ",\"height\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail8)
        }
        out.append(",\"height\":")
        val tail9 = out.lastOrNull()?.code ?: -1
        if (tail9 >= 55296 && tail9 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(plan.height)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(plan.height))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(plan.height))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail9)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(plan.height)))
        val tail10 = out.lastOrNull()?.code ?: -1
        if (tail10 >= 55296 && tail10 <= 56319 && ",\"lines\":[".length > 0 && !(",\"lines\":["[0].code >= 56320 && ",\"lines\":["[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail10)
        }
        out.append(",\"lines\":[")
        var firstLine = true
        run {
            val _g1 = plan.lines
            for (line in _g1) {
                if ((!firstLine)) {
                    val tail11 = out.lastOrNull()?.code ?: -1
                    if (tail11 >= 55296 && tail11 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail11)
                    }
                    out.append(",")
                }
                firstLine = false
                PlanJson.appendLine(out, line)
            }
        }
        val tail12 = out.lastOrNull()?.code ?: -1
        if (tail12 >= 55296 && tail12 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail12)
        }
        out.append("]")
        PlanJson.appendParagraphEvidence(out, plan)
        val tail13 = out.lastOrNull()?.code ?: -1
        if (tail13 >= 55296 && tail13 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail13)
        }
        out.append("}")
        val tail14 = out.lastOrNull()?.code ?: -1
        if (tail14 >= 55296 && tail14 <= 56319) {
            throw UStringException.UnpairedSurrogate(tail14)
        }
        return out.toString()
    }

    private fun appendLine(out: StringBuilder, line: PlanLine) {
        val tail15 = out.lastOrNull()?.code ?: -1
        if (tail15 >= 55296 && tail15 <= 56319 && "{\"rangeStart\":".length > 0 && !("{\"rangeStart\":"[0].code >= 56320 && "{\"rangeStart\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail15)
        }
        out.append("{\"rangeStart\":")
        val tail16 = out.lastOrNull()?.code ?: -1
        if (tail16 >= 55296 && tail16 <= 56319 && ((line.rangeStart).toString()).length > 0 && !(((line.rangeStart).toString())[0].code >= 56320 && ((line.rangeStart).toString())[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail16)
        }
        out.append(((line.rangeStart).toString()))
        val tail17 = out.lastOrNull()?.code ?: -1
        if (tail17 >= 55296 && tail17 <= 56319 && ",\"rangeEnd\":".length > 0 && !(",\"rangeEnd\":"[0].code >= 56320 && ",\"rangeEnd\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail17)
        }
        out.append(",\"rangeEnd\":")
        val tail18 = out.lastOrNull()?.code ?: -1
        if (tail18 >= 55296 && tail18 <= 56319 && ((line.rangeEnd).toString()).length > 0 && !(((line.rangeEnd).toString())[0].code >= 56320 && ((line.rangeEnd).toString())[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail18)
        }
        out.append(((line.rangeEnd).toString()))
        val tail19 = out.lastOrNull()?.code ?: -1
        if (tail19 >= 55296 && tail19 <= 56319 && ",\"top\":".length > 0 && !(",\"top\":"[0].code >= 56320 && ",\"top\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail19)
        }
        out.append(",\"top\":")
        val tail20 = out.lastOrNull()?.code ?: -1
        if (tail20 >= 55296 && tail20 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(line.top)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(line.top))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(line.top))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail20)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(line.top)))
        val tail21 = out.lastOrNull()?.code ?: -1
        if (tail21 >= 55296 && tail21 <= 56319 && ",\"bottom\":".length > 0 && !(",\"bottom\":"[0].code >= 56320 && ",\"bottom\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail21)
        }
        out.append(",\"bottom\":")
        val tail22 = out.lastOrNull()?.code ?: -1
        if (tail22 >= 55296 && tail22 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(line.bottom)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(line.bottom))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(line.bottom))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail22)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(line.bottom)))
        val tail23 = out.lastOrNull()?.code ?: -1
        if (tail23 >= 55296 && tail23 <= 56319 && ",\"baseline\":".length > 0 && !(",\"baseline\":"[0].code >= 56320 && ",\"baseline\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail23)
        }
        out.append(",\"baseline\":")
        val tail24 = out.lastOrNull()?.code ?: -1
        if (tail24 >= 55296 && tail24 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(line.baseline)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(line.baseline))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(line.baseline))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail24)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(line.baseline)))
        val tail25 = out.lastOrNull()?.code ?: -1
        if (tail25 >= 55296 && tail25 <= 56319 && ",\"indent\":".length > 0 && !(",\"indent\":"[0].code >= 56320 && ",\"indent\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail25)
        }
        out.append(",\"indent\":")
        val tail26 = out.lastOrNull()?.code ?: -1
        if (tail26 >= 55296 && tail26 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(line.indent)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(line.indent))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(line.indent))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail26)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(line.indent)))
        val tail27 = out.lastOrNull()?.code ?: -1
        if (tail27 >= 55296 && tail27 <= 56319 && ",\"visualWidth\":".length > 0 && !(",\"visualWidth\":"[0].code >= 56320 && ",\"visualWidth\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail27)
        }
        out.append(",\"visualWidth\":")
        val tail28 = out.lastOrNull()?.code ?: -1
        if (tail28 >= 55296 && tail28 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(line.visualWidth)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(line.visualWidth))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(line.visualWidth))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail28)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(line.visualWidth)))
        val tail29 = out.lastOrNull()?.code ?: -1
        if (tail29 >= 55296 && tail29 <= 56319 && ",\"hyphenAdvance\":".length > 0 && !(",\"hyphenAdvance\":"[0].code >= 56320 && ",\"hyphenAdvance\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail29)
        }
        out.append(",\"hyphenAdvance\":")
        val tail30 = out.lastOrNull()?.code ?: -1
        if (tail30 >= 55296 && tail30 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(line.hyphenAdvance)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(line.hyphenAdvance))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(line.hyphenAdvance))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail30)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(line.hyphenAdvance)))
        val tail31 = out.lastOrNull()?.code ?: -1
        if (tail31 >= 55296 && tail31 <= 56319 && ",\"endReason\":".length > 0 && !(",\"endReason\":"[0].code >= 56320 && ",\"endReason\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail31)
        }
        out.append(",\"endReason\":")
        PlanJsonNumber.appendJsonString(out, PlanJson.endReasonName(line.endReason))
        val tail32 = out.lastOrNull()?.code ?: -1
        if (tail32 >= 55296 && tail32 <= 56319 && ",\"cells\":[".length > 0 && !(",\"cells\":["[0].code >= 56320 && ",\"cells\":["[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail32)
        }
        out.append(",\"cells\":[")
        var firstCell = true
        run {
            val _g1 = line.cells
            for (cell in _g1) {
                if ((!firstCell)) {
                    val tail33 = out.lastOrNull()?.code ?: -1
                    if (tail33 >= 55296 && tail33 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail33)
                    }
                    out.append(",")
                }
                firstCell = false
                PlanJson.appendCell(out, cell)
            }
        }
        val tail34 = out.lastOrNull()?.code ?: -1
        if (tail34 >= 55296 && tail34 <= 56319 && "]}".length > 0 && !("]}"[0].code >= 56320 && "]}"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail34)
        }
        out.append("]}")
    }

    private fun appendCell(out: StringBuilder, cell: PlanCell) {
        val tail35 = out.lastOrNull()?.code ?: -1
        if (tail35 >= 55296 && tail35 <= 56319 && "{\"rangeStart\":".length > 0 && !("{\"rangeStart\":"[0].code >= 56320 && "{\"rangeStart\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail35)
        }
        out.append("{\"rangeStart\":")
        val tail36 = out.lastOrNull()?.code ?: -1
        if (tail36 >= 55296 && tail36 <= 56319 && ((cell.rangeStart).toString()).length > 0 && !(((cell.rangeStart).toString())[0].code >= 56320 && ((cell.rangeStart).toString())[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail36)
        }
        out.append(((cell.rangeStart).toString()))
        val tail37 = out.lastOrNull()?.code ?: -1
        if (tail37 >= 55296 && tail37 <= 56319 && ",\"rangeEnd\":".length > 0 && !(",\"rangeEnd\":"[0].code >= 56320 && ",\"rangeEnd\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail37)
        }
        out.append(",\"rangeEnd\":")
        val tail38 = out.lastOrNull()?.code ?: -1
        if (tail38 >= 55296 && tail38 <= 56319 && ((cell.rangeEnd).toString()).length > 0 && !(((cell.rangeEnd).toString())[0].code >= 56320 && ((cell.rangeEnd).toString())[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail38)
        }
        out.append(((cell.rangeEnd).toString()))
        val tail39 = out.lastOrNull()?.code ?: -1
        if (tail39 >= 55296 && tail39 <= 56319 && ",\"source\":".length > 0 && !(",\"source\":"[0].code >= 56320 && ",\"source\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail39)
        }
        out.append(",\"source\":")
        PlanJsonNumber.appendJsonString(out, cell.source)
        val tail40 = out.lastOrNull()?.code ?: -1
        if (tail40 >= 55296 && tail40 <= 56319 && ",\"display\":".length > 0 && !(",\"display\":"[0].code >= 56320 && ",\"display\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail40)
        }
        out.append(",\"display\":")
        PlanJsonNumber.appendJsonString(out, cell.display)
        val tail41 = out.lastOrNull()?.code ?: -1
        if (tail41 >= 55296 && tail41 <= 56319 && ",\"drawX\":".length > 0 && !(",\"drawX\":"[0].code >= 56320 && ",\"drawX\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail41)
        }
        out.append(",\"drawX\":")
        val tail42 = out.lastOrNull()?.code ?: -1
        if (tail42 >= 55296 && tail42 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(cell.drawX)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(cell.drawX))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(cell.drawX))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail42)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(cell.drawX)))
        val tail43 = out.lastOrNull()?.code ?: -1
        if (tail43 >= 55296 && tail43 <= 56319 && ",\"naturalWidth\":".length > 0 && !(",\"naturalWidth\":"[0].code >= 56320 && ",\"naturalWidth\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail43)
        }
        out.append(",\"naturalWidth\":")
        val tail44 = out.lastOrNull()?.code ?: -1
        if (tail44 >= 55296 && tail44 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(cell.naturalWidth)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(cell.naturalWidth))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(cell.naturalWidth))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail44)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(cell.naturalWidth)))
        val tail45 = out.lastOrNull()?.code ?: -1
        if (tail45 >= 55296 && tail45 <= 56319 && ",\"leadingLayoutAdvance\":".length > 0 && !(",\"leadingLayoutAdvance\":"[0].code >= 56320 && ",\"leadingLayoutAdvance\":"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail45)
        }
        out.append(",\"leadingLayoutAdvance\":")
        val tail46 = out.lastOrNull()?.code ?: -1
        if (tail46 >= 55296 && tail46 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(cell.leadingLayoutAdvance)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(cell.leadingLayoutAdvance))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(cell.leadingLayoutAdvance))[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail46)
        }
        out.append((PlanJsonNumber.ecmaJsonNumber(cell.leadingLayoutAdvance)))
        if ((cell.shapingBoundary)) {
            val tail47 = out.lastOrNull()?.code ?: -1
            if (tail47 >= 55296 && tail47 <= 56319 && ",\"shapingBoundary\":true".length > 0 && !(",\"shapingBoundary\":true"[0].code >= 56320 && ",\"shapingBoundary\":true"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail47)
            }
            out.append(",\"shapingBoundary\":true")
        }
        if ((cell.openTypeFeatures.size > 0)) {
            val tail48 = out.lastOrNull()?.code ?: -1
            if (tail48 >= 55296 && tail48 <= 56319 && ",\"openTypeFeatures\":[".length > 0 && !(",\"openTypeFeatures\":["[0].code >= 56320 && ",\"openTypeFeatures\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail48)
            }
            out.append(",\"openTypeFeatures\":[")
            var firstFeature = true
            run {
                val _g1 = cell.openTypeFeatures
                for (feature in _g1) {
                    if ((!firstFeature)) {
                        val tail49 = out.lastOrNull()?.code ?: -1
                        if (tail49 >= 55296 && tail49 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail49)
                        }
                        out.append(",")
                    }
                    firstFeature = false
                    PlanJsonNumber.appendJsonString(out, feature)
                }
            }
            val tail50 = out.lastOrNull()?.code ?: -1
            if (tail50 >= 55296 && tail50 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail50)
            }
            out.append("]")
        }
        PlanJson.appendCellEvidence(out, cell)
        val tail51 = out.lastOrNull()?.code ?: -1
        if (tail51 >= 55296 && tail51 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail51)
        }
        out.append("}")
    }

    private fun appendCellEvidence(out: StringBuilder, cell: PlanCell) {
        val inlineObject = cell.inlineObject
        if ((inlineObject != null)) {
            val tail52 = out.lastOrNull()?.code ?: -1
            if (tail52 >= 55296 && tail52 <= 56319 && ",\"inlineObject\":".length > 0 && !(",\"inlineObject\":"[0].code >= 56320 && ",\"inlineObject\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail52)
            }
            out.append(",\"inlineObject\":")
            val tail53 = out.lastOrNull()?.code ?: -1
            if (tail53 >= 55296 && tail53 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(inlineObject)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(inlineObject))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(inlineObject))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail53)
            }
            out.append((PlanJsonNumber.ecmaJsonNumber(inlineObject)))
        }
        val advance = cell.advance
        if ((advance != null)) {
            val tail54 = out.lastOrNull()?.code ?: -1
            if (tail54 >= 55296 && tail54 <= 56319 && ",\"advance\":".length > 0 && !(",\"advance\":"[0].code >= 56320 && ",\"advance\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail54)
            }
            out.append(",\"advance\":")
            val tail55 = out.lastOrNull()?.code ?: -1
            if (tail55 >= 55296 && tail55 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(advance)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(advance))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(advance))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail55)
            }
            out.append((PlanJsonNumber.ecmaJsonNumber(advance)))
        }
        val renderFontFamily = cell.renderFontFamily
        if ((renderFontFamily != null)) {
            val tail56 = out.lastOrNull()?.code ?: -1
            if (tail56 >= 55296 && tail56 <= 56319 && ",\"renderFontFamily\":".length > 0 && !(",\"renderFontFamily\":"[0].code >= 56320 && ",\"renderFontFamily\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail56)
            }
            out.append(",\"renderFontFamily\":")
            PlanJsonNumber.appendJsonString(out, renderFontFamily)
        }
        val dashStrategy = cell.dashStrategy
        if ((dashStrategy != null)) {
            val tail57 = out.lastOrNull()?.code ?: -1
            if (tail57 >= 55296 && tail57 <= 56319 && ",\"dashStrategy\":".length > 0 && !(",\"dashStrategy\":"[0].code >= 56320 && ",\"dashStrategy\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail57)
            }
            out.append(",\"dashStrategy\":")
            PlanJsonNumber.appendJsonString(out, dashStrategy)
            val shapingLanguage = cell.shapingLanguage
            if ((shapingLanguage != null)) {
                val tail58 = out.lastOrNull()?.code ?: -1
                if (tail58 >= 55296 && tail58 <= 56319 && ",\"shapingLanguage\":".length > 0 && !(",\"shapingLanguage\":"[0].code >= 56320 && ",\"shapingLanguage\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail58)
                }
                out.append(",\"shapingLanguage\":")
                PlanJsonNumber.appendJsonString(out, shapingLanguage)
            }
            val resolvedFace = cell.resolvedFace
            if ((resolvedFace != null)) {
                val tail59 = out.lastOrNull()?.code ?: -1
                if (tail59 >= 55296 && tail59 <= 56319 && ",\"resolvedFace\":".length > 0 && !(",\"resolvedFace\":"[0].code >= 56320 && ",\"resolvedFace\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail59)
                }
                out.append(",\"resolvedFace\":")
                PlanJsonNumber.appendJsonString(out, resolvedFace)
            }
            val glyphIds = cell.glyphIds
            if ((glyphIds != null)) {
                val tail60 = out.lastOrNull()?.code ?: -1
                if (tail60 >= 55296 && tail60 <= 56319 && ",\"glyphIds\":".length > 0 && !(",\"glyphIds\":"[0].code >= 56320 && ",\"glyphIds\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail60)
                }
                out.append(",\"glyphIds\":")
                PlanJsonNumber.appendJsonString(out, glyphIds)
            }
            val shapingEvidence = cell.shapingEvidence
            if ((shapingEvidence != null)) {
                val tail61 = out.lastOrNull()?.code ?: -1
                if (tail61 >= 55296 && tail61 <= 56319 && ",\"shapingEvidence\":".length > 0 && !(",\"shapingEvidence\":"[0].code >= 56320 && ",\"shapingEvidence\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail61)
                }
                out.append(",\"shapingEvidence\":")
                PlanJsonNumber.appendJsonString(out, shapingEvidence)
            }
        }
        val punctuationInkFloor = cell.punctuationInkFloor
        if ((punctuationInkFloor != null)) {
            val tail62 = out.lastOrNull()?.code ?: -1
            if (tail62 >= 55296 && tail62 <= 56319 && ",\"punctuationInkFloor\":".length > 0 && !(",\"punctuationInkFloor\":"[0].code >= 56320 && ",\"punctuationInkFloor\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail62)
            }
            out.append(",\"punctuationInkFloor\":")
            val tail63 = out.lastOrNull()?.code ?: -1
            if (tail63 >= 55296 && tail63 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(punctuationInkFloor)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(punctuationInkFloor))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(punctuationInkFloor))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail63)
            }
            out.append((PlanJsonNumber.ecmaJsonNumber(punctuationInkFloor)))
            val punctuationBodyWidth = cell.punctuationBodyWidth
            if ((punctuationBodyWidth != null)) {
                val tail64 = out.lastOrNull()?.code ?: -1
                if (tail64 >= 55296 && tail64 <= 56319 && ",\"punctuationBodyWidth\":".length > 0 && !(",\"punctuationBodyWidth\":"[0].code >= 56320 && ",\"punctuationBodyWidth\":"[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail64)
                }
                out.append(",\"punctuationBodyWidth\":")
                val tail65 = out.lastOrNull()?.code ?: -1
                if (tail65 >= 55296 && tail65 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(punctuationBodyWidth)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(punctuationBodyWidth))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(punctuationBodyWidth))[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail65)
                }
                out.append((PlanJsonNumber.ecmaJsonNumber(punctuationBodyWidth)))
            }
        }
        if ((cell.latin)) {
            val tail66 = out.lastOrNull()?.code ?: -1
            if (tail66 >= 55296 && tail66 <= 56319 && ",\"latin\":true".length > 0 && !(",\"latin\":true"[0].code >= 56320 && ",\"latin\":true"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail66)
            }
            out.append(",\"latin\":true")
        }
        val styleDelta = cell.styleDelta
        if ((styleDelta != null)) {
            PlanJson.appendStyleDelta(out, styleDelta)
        }
    }

    private fun appendStyleDelta(out: StringBuilder, style: PlanStyleDelta) {
        val tail67 = out.lastOrNull()?.code ?: -1
        if (tail67 >= 55296 && tail67 <= 56319 && ",\"style\":{".length > 0 && !(",\"style\":{"[0].code >= 56320 && ",\"style\":{"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail67)
        }
        out.append(",\"style\":{")
        var fieldCount = 0
        val fontSize = style.fontSize
        if ((fontSize != null)) {
            val tail68 = out.lastOrNull()?.code ?: -1
            if (tail68 >= 55296 && tail68 <= 56319 && "\"fontSize\":".length > 0 && !("\"fontSize\":"[0].code >= 56320 && "\"fontSize\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail68)
            }
            out.append("\"fontSize\":")
            val tail69 = out.lastOrNull()?.code ?: -1
            if (tail69 >= 55296 && tail69 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(fontSize)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(fontSize))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(fontSize))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail69)
            }
            out.append((PlanJsonNumber.ecmaJsonNumber(fontSize)))
            fieldCount++
        }
        val fontWeight = style.fontWeight
        if ((fontWeight != null)) {
            if ((fieldCount > 0)) {
                val tail70 = out.lastOrNull()?.code ?: -1
                if (tail70 >= 55296 && tail70 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail70)
                }
                out.append(",")
            }
            val tail71 = out.lastOrNull()?.code ?: -1
            if (tail71 >= 55296 && tail71 <= 56319 && "\"fontWeight\":".length > 0 && !("\"fontWeight\":"[0].code >= 56320 && "\"fontWeight\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail71)
            }
            out.append("\"fontWeight\":")
            val tail72 = out.lastOrNull()?.code ?: -1
            if (tail72 >= 55296 && tail72 <= 56319 && ((fontWeight).toString()).length > 0 && !(((fontWeight).toString())[0].code >= 56320 && ((fontWeight).toString())[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail72)
            }
            out.append(((fontWeight).toString()))
            fieldCount++
        }
        val italic = style.italic
        if ((italic != null)) {
            if ((fieldCount > 0)) {
                val tail73 = out.lastOrNull()?.code ?: -1
                if (tail73 >= 55296 && tail73 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                    throw UStringException.UnpairedSurrogate(tail73)
                }
                out.append(",")
            }
            val tail74 = out.lastOrNull()?.code ?: -1
            if (tail74 >= 55296 && tail74 <= 56319 && "\"italic\":".length > 0 && !("\"italic\":"[0].code >= 56320 && "\"italic\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail74)
            }
            out.append("\"italic\":")
            val tail75 = out.lastOrNull()?.code ?: -1
            if (tail75 >= 55296 && tail75 <= 56319 && ((if ((italic)) "true" else "false")).length > 0 && !(((if ((italic)) "true" else "false"))[0].code >= 56320 && ((if ((italic)) "true" else "false"))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail75)
            }
            out.append(((if ((italic)) "true" else "false")))
        }
        val tail76 = out.lastOrNull()?.code ?: -1
        if (tail76 >= 55296 && tail76 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
            throw UStringException.UnpairedSurrogate(tail76)
        }
        out.append("}")
    }

    private fun appendParagraphEvidence(out: StringBuilder, plan: Plan) {
        val fontSize = plan.fontSize
        if ((fontSize != null)) {
            val tail77 = out.lastOrNull()?.code ?: -1
            if (tail77 >= 55296 && tail77 <= 56319 && ",\"fontSize\":".length > 0 && !(",\"fontSize\":"[0].code >= 56320 && ",\"fontSize\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail77)
            }
            out.append(",\"fontSize\":")
            val tail78 = out.lastOrNull()?.code ?: -1
            if (tail78 >= 55296 && tail78 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(fontSize)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(fontSize))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(fontSize))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail78)
            }
            out.append((PlanJsonNumber.ecmaJsonNumber(fontSize)))
        }
        val overlayWidth = plan.overlayWidth
        if ((overlayWidth != null)) {
            val tail79 = out.lastOrNull()?.code ?: -1
            if (tail79 >= 55296 && tail79 <= 56319 && ",\"overlayWidth\":".length > 0 && !(",\"overlayWidth\":"[0].code >= 56320 && ",\"overlayWidth\":"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail79)
            }
            out.append(",\"overlayWidth\":")
            val tail80 = out.lastOrNull()?.code ?: -1
            if (tail80 >= 55296 && tail80 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(overlayWidth)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(overlayWidth))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(overlayWidth))[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail80)
            }
            out.append((PlanJsonNumber.ecmaJsonNumber(overlayWidth)))
        }
        if ((plan.emphasisRanges.size > 0)) {
            val tail81 = out.lastOrNull()?.code ?: -1
            if (tail81 >= 55296 && tail81 <= 56319 && ",\"emphasisRanges\":[".length > 0 && !(",\"emphasisRanges\":["[0].code >= 56320 && ",\"emphasisRanges\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail81)
            }
            out.append(",\"emphasisRanges\":[")
            var firstRange = true
            run {
                val _g1 = plan.emphasisRanges
                for (range in _g1) {
                    if ((!firstRange)) {
                        val tail82 = out.lastOrNull()?.code ?: -1
                        if (tail82 >= 55296 && tail82 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail82)
                        }
                        out.append(",")
                    }
                    firstRange = false
                    val tail83 = out.lastOrNull()?.code ?: -1
                    if (tail83 >= 55296 && tail83 <= 56319 && "[".length > 0 && !("["[0].code >= 56320 && "["[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail83)
                    }
                    out.append("[")
                    val tail84 = out.lastOrNull()?.code ?: -1
                    if (tail84 >= 55296 && tail84 <= 56319 && ((range.start).toString()).length > 0 && !(((range.start).toString())[0].code >= 56320 && ((range.start).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail84)
                    }
                    out.append(((range.start).toString()))
                    val tail85 = out.lastOrNull()?.code ?: -1
                    if (tail85 >= 55296 && tail85 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail85)
                    }
                    out.append(",")
                    val tail86 = out.lastOrNull()?.code ?: -1
                    if (tail86 >= 55296 && tail86 <= 56319 && ((range.end).toString()).length > 0 && !(((range.end).toString())[0].code >= 56320 && ((range.end).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail86)
                    }
                    out.append(((range.end).toString()))
                    val tail87 = out.lastOrNull()?.code ?: -1
                    if (tail87 >= 55296 && tail87 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail87)
                    }
                    out.append("]")
                }
            }
            val tail88 = out.lastOrNull()?.code ?: -1
            if (tail88 >= 55296 && tail88 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail88)
            }
            out.append("]")
        }
        if ((plan.inlineEdges.size > 0)) {
            val tail89 = out.lastOrNull()?.code ?: -1
            if (tail89 >= 55296 && tail89 <= 56319 && ",\"inlineEdges\":[".length > 0 && !(",\"inlineEdges\":["[0].code >= 56320 && ",\"inlineEdges\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail89)
            }
            out.append(",\"inlineEdges\":[")
            var firstEdge = true
            run {
                val _g1_2 = plan.inlineEdges
                for (edge in _g1_2) {
                    if ((!firstEdge)) {
                        val tail90 = out.lastOrNull()?.code ?: -1
                        if (tail90 >= 55296 && tail90 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail90)
                        }
                        out.append(",")
                    }
                    firstEdge = false
                    val tail91 = out.lastOrNull()?.code ?: -1
                    if (tail91 >= 55296 && tail91 <= 56319 && "{\"offset\":".length > 0 && !("{\"offset\":"[0].code >= 56320 && "{\"offset\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail91)
                    }
                    out.append("{\"offset\":")
                    val tail92 = out.lastOrNull()?.code ?: -1
                    if (tail92 >= 55296 && tail92 <= 56319 && ((edge.offset).toString()).length > 0 && !(((edge.offset).toString())[0].code >= 56320 && ((edge.offset).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail92)
                    }
                    out.append(((edge.offset).toString()))
                    val inlineStart = edge.inlineStart
                    if ((inlineStart != null)) {
                        val tail93 = out.lastOrNull()?.code ?: -1
                        if (tail93 >= 55296 && tail93 <= 56319 && ",\"inlineStart\":".length > 0 && !(",\"inlineStart\":"[0].code >= 56320 && ",\"inlineStart\":"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail93)
                        }
                        out.append(",\"inlineStart\":")
                        val tail94 = out.lastOrNull()?.code ?: -1
                        if (tail94 >= 55296 && tail94 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(inlineStart)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(inlineStart))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(inlineStart))[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail94)
                        }
                        out.append((PlanJsonNumber.ecmaJsonNumber(inlineStart)))
                    }
                    val inlineEnd = edge.inlineEnd
                    if ((inlineEnd != null)) {
                        val tail95 = out.lastOrNull()?.code ?: -1
                        if (tail95 >= 55296 && tail95 <= 56319 && ",\"inlineEnd\":".length > 0 && !(",\"inlineEnd\":"[0].code >= 56320 && ",\"inlineEnd\":"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail95)
                        }
                        out.append(",\"inlineEnd\":")
                        val tail96 = out.lastOrNull()?.code ?: -1
                        if (tail96 >= 55296 && tail96 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(inlineEnd)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(inlineEnd))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(inlineEnd))[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail96)
                        }
                        out.append((PlanJsonNumber.ecmaJsonNumber(inlineEnd)))
                    }
                    val tail97 = out.lastOrNull()?.code ?: -1
                    if (tail97 >= 55296 && tail97 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail97)
                    }
                    out.append("}")
                }
            }
            val tail98 = out.lastOrNull()?.code ?: -1
            if (tail98 >= 55296 && tail98 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail98)
            }
            out.append("]")
        }
        if ((plan.rubyDecisions.size > 0)) {
            val tail99 = out.lastOrNull()?.code ?: -1
            if (tail99 >= 55296 && tail99 <= 56319 && ",\"rubyDecisions\":[".length > 0 && !(",\"rubyDecisions\":["[0].code >= 56320 && ",\"rubyDecisions\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail99)
            }
            out.append(",\"rubyDecisions\":[")
            var firstRuby = true
            run {
                val _g1_3 = plan.rubyDecisions
                for (ruby in _g1_3) {
                    if ((!firstRuby)) {
                        val tail100 = out.lastOrNull()?.code ?: -1
                        if (tail100 >= 55296 && tail100 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail100)
                        }
                        out.append(",")
                    }
                    firstRuby = false
                    val tail101 = out.lastOrNull()?.code ?: -1
                    if (tail101 >= 55296 && tail101 <= 56319 && "{\"baseRangeStart\":".length > 0 && !("{\"baseRangeStart\":"[0].code >= 56320 && "{\"baseRangeStart\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail101)
                    }
                    out.append("{\"baseRangeStart\":")
                    val tail102 = out.lastOrNull()?.code ?: -1
                    if (tail102 >= 55296 && tail102 <= 56319 && ((ruby.baseRangeStart).toString()).length > 0 && !(((ruby.baseRangeStart).toString())[0].code >= 56320 && ((ruby.baseRangeStart).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail102)
                    }
                    out.append(((ruby.baseRangeStart).toString()))
                    val tail103 = out.lastOrNull()?.code ?: -1
                    if (tail103 >= 55296 && tail103 <= 56319 && ",\"baseRangeEnd\":".length > 0 && !(",\"baseRangeEnd\":"[0].code >= 56320 && ",\"baseRangeEnd\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail103)
                    }
                    out.append(",\"baseRangeEnd\":")
                    val tail104 = out.lastOrNull()?.code ?: -1
                    if (tail104 >= 55296 && tail104 <= 56319 && ((ruby.baseRangeEnd).toString()).length > 0 && !(((ruby.baseRangeEnd).toString())[0].code >= 56320 && ((ruby.baseRangeEnd).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail104)
                    }
                    out.append(((ruby.baseRangeEnd).toString()))
                    val tail105 = out.lastOrNull()?.code ?: -1
                    if (tail105 >= 55296 && tail105 <= 56319 && ",\"text\":".length > 0 && !(",\"text\":"[0].code >= 56320 && ",\"text\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail105)
                    }
                    out.append(",\"text\":")
                    PlanJsonNumber.appendJsonString(out, ruby.text)
                    val tail106 = out.lastOrNull()?.code ?: -1
                    if (tail106 >= 55296 && tail106 <= 56319 && ",\"centerX\":".length > 0 && !(",\"centerX\":"[0].code >= 56320 && ",\"centerX\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail106)
                    }
                    out.append(",\"centerX\":")
                    val tail107 = out.lastOrNull()?.code ?: -1
                    if (tail107 >= 55296 && tail107 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(ruby.centerX)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(ruby.centerX))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(ruby.centerX))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail107)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(ruby.centerX)))
                    val tail108 = out.lastOrNull()?.code ?: -1
                    if (tail108 >= 55296 && tail108 <= 56319 && ",\"baselineY\":".length > 0 && !(",\"baselineY\":"[0].code >= 56320 && ",\"baselineY\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail108)
                    }
                    out.append(",\"baselineY\":")
                    val tail109 = out.lastOrNull()?.code ?: -1
                    if (tail109 >= 55296 && tail109 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(ruby.baselineY)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(ruby.baselineY))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(ruby.baselineY))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail109)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(ruby.baselineY)))
                    val tail110 = out.lastOrNull()?.code ?: -1
                    if (tail110 >= 55296 && tail110 <= 56319 && ",\"fontSize\":".length > 0 && !(",\"fontSize\":"[0].code >= 56320 && ",\"fontSize\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail110)
                    }
                    out.append(",\"fontSize\":")
                    val tail111 = out.lastOrNull()?.code ?: -1
                    if (tail111 >= 55296 && tail111 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(ruby.fontSize)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(ruby.fontSize))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(ruby.fontSize))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail111)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(ruby.fontSize)))
                    val ascent = ruby.ascent
                    if ((ascent != null)) {
                        val tail112 = out.lastOrNull()?.code ?: -1
                        if (tail112 >= 55296 && tail112 <= 56319 && ",\"ascent\":".length > 0 && !(",\"ascent\":"[0].code >= 56320 && ",\"ascent\":"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail112)
                        }
                        out.append(",\"ascent\":")
                        val tail113 = out.lastOrNull()?.code ?: -1
                        if (tail113 >= 55296 && tail113 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(ascent)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(ascent))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(ascent))[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail113)
                        }
                        out.append((PlanJsonNumber.ecmaJsonNumber(ascent)))
                    }
                    val tail114 = out.lastOrNull()?.code ?: -1
                    if (tail114 >= 55296 && tail114 <= 56319 && ",\"fontWeight\":".length > 0 && !(",\"fontWeight\":"[0].code >= 56320 && ",\"fontWeight\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail114)
                    }
                    out.append(",\"fontWeight\":")
                    val tail115 = out.lastOrNull()?.code ?: -1
                    if (tail115 >= 55296 && tail115 <= 56319 && ((ruby.fontWeight).toString()).length > 0 && !(((ruby.fontWeight).toString())[0].code >= 56320 && ((ruby.fontWeight).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail115)
                    }
                    out.append(((ruby.fontWeight).toString()))
                    if ((ruby.fontFamilies.size > 0)) {
                        val tail116 = out.lastOrNull()?.code ?: -1
                        if (tail116 >= 55296 && tail116 <= 56319 && ",\"fontFamilies\":[".length > 0 && !(",\"fontFamilies\":["[0].code >= 56320 && ",\"fontFamilies\":["[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail116)
                        }
                        out.append(",\"fontFamilies\":[")
                        var firstFamily = true
                        run {
                            var _g = 0
                            val _g1_4 = ruby.fontFamilies
                            while ((_g < _g1_4.size)) {
                                val family = _g1_4[_g]
                                ++_g
                                if ((!firstFamily)) {
                                    val tail117 = out.lastOrNull()?.code ?: -1
                                    if (tail117 >= 55296 && tail117 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                                        throw UStringException.UnpairedSurrogate(tail117)
                                    }
                                    out.append(",")
                                }
                                firstFamily = false
                                PlanJsonNumber.appendJsonString(out, family)
                            }
                        }
                        val tail118 = out.lastOrNull()?.code ?: -1
                        if (tail118 >= 55296 && tail118 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail118)
                        }
                        out.append("]")
                    }
                    val tail119 = out.lastOrNull()?.code ?: -1
                    if (tail119 >= 55296 && tail119 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail119)
                    }
                    out.append("}")
                }
            }
            val tail120 = out.lastOrNull()?.code ?: -1
            if (tail120 >= 55296 && tail120 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail120)
            }
            out.append("]")
        }
        if ((plan.bopomofoDecisions.size > 0)) {
            val tail121 = out.lastOrNull()?.code ?: -1
            if (tail121 >= 55296 && tail121 <= 56319 && ",\"bopomofoDecisions\":[".length > 0 && !(",\"bopomofoDecisions\":["[0].code >= 56320 && ",\"bopomofoDecisions\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail121)
            }
            out.append(",\"bopomofoDecisions\":[")
            var firstBopomofo = true
            run {
                val _g1_5 = plan.bopomofoDecisions
                for (bopomofo in _g1_5) {
                    if ((!firstBopomofo)) {
                        val tail122 = out.lastOrNull()?.code ?: -1
                        if (tail122 >= 55296 && tail122 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail122)
                        }
                        out.append(",")
                    }
                    firstBopomofo = false
                    val tail123 = out.lastOrNull()?.code ?: -1
                    if (tail123 >= 55296 && tail123 <= 56319 && "{\"baseRangeStart\":".length > 0 && !("{\"baseRangeStart\":"[0].code >= 56320 && "{\"baseRangeStart\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail123)
                    }
                    out.append("{\"baseRangeStart\":")
                    val tail124 = out.lastOrNull()?.code ?: -1
                    if (tail124 >= 55296 && tail124 <= 56319 && ((bopomofo.baseRangeStart).toString()).length > 0 && !(((bopomofo.baseRangeStart).toString())[0].code >= 56320 && ((bopomofo.baseRangeStart).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail124)
                    }
                    out.append(((bopomofo.baseRangeStart).toString()))
                    val tail125 = out.lastOrNull()?.code ?: -1
                    if (tail125 >= 55296 && tail125 <= 56319 && ",\"baseRangeEnd\":".length > 0 && !(",\"baseRangeEnd\":"[0].code >= 56320 && ",\"baseRangeEnd\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail125)
                    }
                    out.append(",\"baseRangeEnd\":")
                    val tail126 = out.lastOrNull()?.code ?: -1
                    if (tail126 >= 55296 && tail126 <= 56319 && ((bopomofo.baseRangeEnd).toString()).length > 0 && !(((bopomofo.baseRangeEnd).toString())[0].code >= 56320 && ((bopomofo.baseRangeEnd).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail126)
                    }
                    out.append(((bopomofo.baseRangeEnd).toString()))
                    val tail127 = out.lastOrNull()?.code ?: -1
                    if (tail127 >= 55296 && tail127 <= 56319 && ",\"text\":".length > 0 && !(",\"text\":"[0].code >= 56320 && ",\"text\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail127)
                    }
                    out.append(",\"text\":")
                    PlanJsonNumber.appendJsonString(out, bopomofo.text)
                    val tail128 = out.lastOrNull()?.code ?: -1
                    if (tail128 >= 55296 && tail128 <= 56319 && ",\"fontWeight\":".length > 0 && !(",\"fontWeight\":"[0].code >= 56320 && ",\"fontWeight\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail128)
                    }
                    out.append(",\"fontWeight\":")
                    val tail129 = out.lastOrNull()?.code ?: -1
                    if (tail129 >= 55296 && tail129 <= 56319 && ((bopomofo.fontWeight).toString()).length > 0 && !(((bopomofo.fontWeight).toString())[0].code >= 56320 && ((bopomofo.fontWeight).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail129)
                    }
                    out.append(((bopomofo.fontWeight).toString()))
                    if ((bopomofo.fontFamilies.size > 0)) {
                        val tail130 = out.lastOrNull()?.code ?: -1
                        if (tail130 >= 55296 && tail130 <= 56319 && ",\"fontFamilies\":[".length > 0 && !(",\"fontFamilies\":["[0].code >= 56320 && ",\"fontFamilies\":["[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail130)
                        }
                        out.append(",\"fontFamilies\":[")
                        var firstFamily_2 = true
                        run {
                            var _g_2 = 0
                            val _g1_6 = bopomofo.fontFamilies
                            while ((_g_2 < _g1_6.size)) {
                                val family_2 = _g1_6[_g_2]
                                ++_g_2
                                if ((!firstFamily_2)) {
                                    val tail131 = out.lastOrNull()?.code ?: -1
                                    if (tail131 >= 55296 && tail131 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                                        throw UStringException.UnpairedSurrogate(tail131)
                                    }
                                    out.append(",")
                                }
                                firstFamily_2 = false
                                PlanJsonNumber.appendJsonString(out, family_2)
                            }
                        }
                        val tail132 = out.lastOrNull()?.code ?: -1
                        if (tail132 >= 55296 && tail132 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail132)
                        }
                        out.append("]")
                    }
                    val tail133 = out.lastOrNull()?.code ?: -1
                    if (tail133 >= 55296 && tail133 <= 56319 && ",\"placements\":[".length > 0 && !(",\"placements\":["[0].code >= 56320 && ",\"placements\":["[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail133)
                    }
                    out.append(",\"placements\":[")
                    var firstPlacement = true
                    run {
                        var _g_3 = 0
                        val _g1_7 = bopomofo.placements
                        while ((_g_3 < _g1_7.size)) {
                            val placement = _g1_7[_g_3]
                            ++_g_3
                            if ((!firstPlacement)) {
                                val tail134 = out.lastOrNull()?.code ?: -1
                                if (tail134 >= 55296 && tail134 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                                    throw UStringException.UnpairedSurrogate(tail134)
                                }
                                out.append(",")
                            }
                            firstPlacement = false
                            val tail135 = out.lastOrNull()?.code ?: -1
                            if (tail135 >= 55296 && tail135 <= 56319 && "{\"text\":".length > 0 && !("{\"text\":"[0].code >= 56320 && "{\"text\":"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail135)
                            }
                            out.append("{\"text\":")
                            PlanJsonNumber.appendJsonString(out, placement.text)
                            val tail136 = out.lastOrNull()?.code ?: -1
                            if (tail136 >= 55296 && tail136 <= 56319 && ",\"left\":".length > 0 && !(",\"left\":"[0].code >= 56320 && ",\"left\":"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail136)
                            }
                            out.append(",\"left\":")
                            val tail137 = out.lastOrNull()?.code ?: -1
                            if (tail137 >= 55296 && tail137 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(placement.left)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(placement.left))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(placement.left))[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail137)
                            }
                            out.append((PlanJsonNumber.ecmaJsonNumber(placement.left)))
                            val tail138 = out.lastOrNull()?.code ?: -1
                            if (tail138 >= 55296 && tail138 <= 56319 && ",\"top\":".length > 0 && !(",\"top\":"[0].code >= 56320 && ",\"top\":"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail138)
                            }
                            out.append(",\"top\":")
                            val tail139 = out.lastOrNull()?.code ?: -1
                            if (tail139 >= 55296 && tail139 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(placement.top)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(placement.top))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(placement.top))[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail139)
                            }
                            out.append((PlanJsonNumber.ecmaJsonNumber(placement.top)))
                            val tail140 = out.lastOrNull()?.code ?: -1
                            if (tail140 >= 55296 && tail140 <= 56319 && ",\"width\":".length > 0 && !(",\"width\":"[0].code >= 56320 && ",\"width\":"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail140)
                            }
                            out.append(",\"width\":")
                            val tail141 = out.lastOrNull()?.code ?: -1
                            if (tail141 >= 55296 && tail141 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(placement.width)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(placement.width))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(placement.width))[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail141)
                            }
                            out.append((PlanJsonNumber.ecmaJsonNumber(placement.width)))
                            val tail142 = out.lastOrNull()?.code ?: -1
                            if (tail142 >= 55296 && tail142 <= 56319 && ",\"height\":".length > 0 && !(",\"height\":"[0].code >= 56320 && ",\"height\":"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail142)
                            }
                            out.append(",\"height\":")
                            val tail143 = out.lastOrNull()?.code ?: -1
                            if (tail143 >= 55296 && tail143 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(placement.height)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(placement.height))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(placement.height))[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail143)
                            }
                            out.append((PlanJsonNumber.ecmaJsonNumber(placement.height)))
                            val tail144 = out.lastOrNull()?.code ?: -1
                            if (tail144 >= 55296 && tail144 <= 56319 && ",\"role\":".length > 0 && !(",\"role\":"[0].code >= 56320 && ",\"role\":"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail144)
                            }
                            out.append(",\"role\":")
                            PlanJsonNumber.appendJsonString(out, placement.role)
                            val tail145 = out.lastOrNull()?.code ?: -1
                            if (tail145 >= 55296 && tail145 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                                throw UStringException.UnpairedSurrogate(tail145)
                            }
                            out.append("}")
                        }
                    }
                    val tail146 = out.lastOrNull()?.code ?: -1
                    if (tail146 >= 55296 && tail146 <= 56319 && "]}".length > 0 && !("]}"[0].code >= 56320 && "]}"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail146)
                    }
                    out.append("]}")
                }
            }
            val tail147 = out.lastOrNull()?.code ?: -1
            if (tail147 >= 55296 && tail147 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail147)
            }
            out.append("]")
        }
        if ((plan.decorationSegments.size > 0)) {
            val tail148 = out.lastOrNull()?.code ?: -1
            if (tail148 >= 55296 && tail148 <= 56319 && ",\"decorationSegments\":[".length > 0 && !(",\"decorationSegments\":["[0].code >= 56320 && ",\"decorationSegments\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail148)
            }
            out.append(",\"decorationSegments\":[")
            var firstSegment = true
            run {
                val _g1_8 = plan.decorationSegments
                for (seg in _g1_8) {
                    if ((!firstSegment)) {
                        val tail149 = out.lastOrNull()?.code ?: -1
                        if (tail149 >= 55296 && tail149 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail149)
                        }
                        out.append(",")
                    }
                    firstSegment = false
                    val tail150 = out.lastOrNull()?.code ?: -1
                    if (tail150 >= 55296 && tail150 <= 56319 && "{\"kind\":".length > 0 && !("{\"kind\":"[0].code >= 56320 && "{\"kind\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail150)
                    }
                    out.append("{\"kind\":")
                    PlanJsonNumber.appendJsonString(out, seg.kind)
                    val tail151 = out.lastOrNull()?.code ?: -1
                    if (tail151 >= 55296 && tail151 <= 56319 && ",\"left\":".length > 0 && !(",\"left\":"[0].code >= 56320 && ",\"left\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail151)
                    }
                    out.append(",\"left\":")
                    val tail152 = out.lastOrNull()?.code ?: -1
                    if (tail152 >= 55296 && tail152 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(seg.left)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(seg.left))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(seg.left))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail152)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(seg.left)))
                    val tail153 = out.lastOrNull()?.code ?: -1
                    if (tail153 >= 55296 && tail153 <= 56319 && ",\"top\":".length > 0 && !(",\"top\":"[0].code >= 56320 && ",\"top\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail153)
                    }
                    out.append(",\"top\":")
                    val tail154 = out.lastOrNull()?.code ?: -1
                    if (tail154 >= 55296 && tail154 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(seg.top)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(seg.top))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(seg.top))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail154)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(seg.top)))
                    val tail155 = out.lastOrNull()?.code ?: -1
                    if (tail155 >= 55296 && tail155 <= 56319 && ",\"right\":".length > 0 && !(",\"right\":"[0].code >= 56320 && ",\"right\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail155)
                    }
                    out.append(",\"right\":")
                    val tail156 = out.lastOrNull()?.code ?: -1
                    if (tail156 >= 55296 && tail156 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(seg.right)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(seg.right))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(seg.right))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail156)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(seg.right)))
                    val tail157 = out.lastOrNull()?.code ?: -1
                    if (tail157 >= 55296 && tail157 <= 56319 && ",\"sourceRangeStart\":".length > 0 && !(",\"sourceRangeStart\":"[0].code >= 56320 && ",\"sourceRangeStart\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail157)
                    }
                    out.append(",\"sourceRangeStart\":")
                    val tail158 = out.lastOrNull()?.code ?: -1
                    if (tail158 >= 55296 && tail158 <= 56319 && ((seg.sourceRangeStart).toString()).length > 0 && !(((seg.sourceRangeStart).toString())[0].code >= 56320 && ((seg.sourceRangeStart).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail158)
                    }
                    out.append(((seg.sourceRangeStart).toString()))
                    val tail159 = out.lastOrNull()?.code ?: -1
                    if (tail159 >= 55296 && tail159 <= 56319 && ",\"sourceRangeEnd\":".length > 0 && !(",\"sourceRangeEnd\":"[0].code >= 56320 && ",\"sourceRangeEnd\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail159)
                    }
                    out.append(",\"sourceRangeEnd\":")
                    val tail160 = out.lastOrNull()?.code ?: -1
                    if (tail160 >= 55296 && tail160 <= 56319 && ((seg.sourceRangeEnd).toString()).length > 0 && !(((seg.sourceRangeEnd).toString())[0].code >= 56320 && ((seg.sourceRangeEnd).toString())[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail160)
                    }
                    out.append(((seg.sourceRangeEnd).toString()))
                    val tail161 = out.lastOrNull()?.code ?: -1
                    if (tail161 >= 55296 && tail161 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail161)
                    }
                    out.append("}")
                }
            }
            val tail162 = out.lastOrNull()?.code ?: -1
            if (tail162 >= 55296 && tail162 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail162)
            }
            out.append("]")
        }
        if ((plan.emphasisDots.size > 0)) {
            val tail163 = out.lastOrNull()?.code ?: -1
            if (tail163 >= 55296 && tail163 <= 56319 && ",\"emphasisDots\":[".length > 0 && !(",\"emphasisDots\":["[0].code >= 56320 && ",\"emphasisDots\":["[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail163)
            }
            out.append(",\"emphasisDots\":[")
            var firstDot = true
            run {
                val _g1_9 = plan.emphasisDots
                for (dot in _g1_9) {
                    if ((!firstDot)) {
                        val tail164 = out.lastOrNull()?.code ?: -1
                        if (tail164 >= 55296 && tail164 <= 56319 && ",".length > 0 && !(","[0].code >= 56320 && ","[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail164)
                        }
                        out.append(",")
                    }
                    firstDot = false
                    val tail165 = out.lastOrNull()?.code ?: -1
                    if (tail165 >= 55296 && tail165 <= 56319 && "{\"clusterRangeStart\":".length > 0 && !("{\"clusterRangeStart\":"[0].code >= 56320 && "{\"clusterRangeStart\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail165)
                    }
                    out.append("{\"clusterRangeStart\":")
                    val clusterRangeStart = dot.clusterRangeStart
                    if ((clusterRangeStart != null)) {
                        val tail166 = out.lastOrNull()?.code ?: -1
                        if (tail166 >= 55296 && tail166 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(clusterRangeStart)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(clusterRangeStart))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(clusterRangeStart))[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail166)
                        }
                        out.append((PlanJsonNumber.ecmaJsonNumber(clusterRangeStart)))
                    } else {
                        val tail167 = out.lastOrNull()?.code ?: -1
                        if (tail167 >= 55296 && tail167 <= 56319 && "null".length > 0 && !("null"[0].code >= 56320 && "null"[0].code <= 57343)) {
                            throw UStringException.UnpairedSurrogate(tail167)
                        }
                        out.append("null")
                    }
                    val tail168 = out.lastOrNull()?.code ?: -1
                    if (tail168 >= 55296 && tail168 <= 56319 && ",\"anchorX\":".length > 0 && !(",\"anchorX\":"[0].code >= 56320 && ",\"anchorX\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail168)
                    }
                    out.append(",\"anchorX\":")
                    val tail169 = out.lastOrNull()?.code ?: -1
                    if (tail169 >= 55296 && tail169 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(dot.anchorX)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(dot.anchorX))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(dot.anchorX))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail169)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(dot.anchorX)))
                    val tail170 = out.lastOrNull()?.code ?: -1
                    if (tail170 >= 55296 && tail170 <= 56319 && ",\"anchorY\":".length > 0 && !(",\"anchorY\":"[0].code >= 56320 && ",\"anchorY\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail170)
                    }
                    out.append(",\"anchorY\":")
                    val tail171 = out.lastOrNull()?.code ?: -1
                    if (tail171 >= 55296 && tail171 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(dot.anchorY)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(dot.anchorY))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(dot.anchorY))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail171)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(dot.anchorY)))
                    val tail172 = out.lastOrNull()?.code ?: -1
                    if (tail172 >= 55296 && tail172 <= 56319 && ",\"dotDiameter\":".length > 0 && !(",\"dotDiameter\":"[0].code >= 56320 && ",\"dotDiameter\":"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail172)
                    }
                    out.append(",\"dotDiameter\":")
                    val tail173 = out.lastOrNull()?.code ?: -1
                    if (tail173 >= 55296 && tail173 <= 56319 && (PlanJsonNumber.ecmaJsonNumber(dot.dotDiameter)).length > 0 && !((PlanJsonNumber.ecmaJsonNumber(dot.dotDiameter))[0].code >= 56320 && (PlanJsonNumber.ecmaJsonNumber(dot.dotDiameter))[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail173)
                    }
                    out.append((PlanJsonNumber.ecmaJsonNumber(dot.dotDiameter)))
                    val tail174 = out.lastOrNull()?.code ?: -1
                    if (tail174 >= 55296 && tail174 <= 56319 && "}".length > 0 && !("}"[0].code >= 56320 && "}"[0].code <= 57343)) {
                        throw UStringException.UnpairedSurrogate(tail174)
                    }
                    out.append("}")
                }
            }
            val tail175 = out.lastOrNull()?.code ?: -1
            if (tail175 >= 55296 && tail175 <= 56319 && "]".length > 0 && !("]"[0].code >= 56320 && "]"[0].code <= 57343)) {
                throw UStringException.UnpairedSurrogate(tail175)
            }
            out.append("]")
        }
    }

    private fun endReasonName(reason: PlanEndReason): String {
        return when (reason) {
            PlanEndReason.AutoWrap -> "AutoWrap"
            PlanEndReason.MandatoryBreak -> "MandatoryBreak"
            PlanEndReason.ParagraphEnd -> "ParagraphEnd"
        }
    }
}
