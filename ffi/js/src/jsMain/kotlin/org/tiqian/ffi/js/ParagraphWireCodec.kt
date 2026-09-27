package org.tiqian.ffi.js

import org.tiqian.core.DEFAULT_EMPHASIS_DOT_GAP_EM
import org.tiqian.core.DecorationKind
import org.tiqian.core.DecorationSpan
import org.tiqian.core.Ic
import org.tiqian.core.InlineBoxOuterSpacing
import org.tiqian.core.InlineBoxSpan
import org.tiqian.core.InlineObjectSpan
import org.tiqian.core.LayoutConstraints
import org.tiqian.core.LayoutInput
import org.tiqian.core.LayoutResult
import org.tiqian.core.LineBreakPolicy
import org.tiqian.core.LineBreakSpan
import org.tiqian.core.LineLengthGrid
import org.tiqian.core.ParagraphStyle
import org.tiqian.core.TextRange
import org.tiqian.core.TextSpan
import org.tiqian.core.TextStyle
import org.tiqian.core.TiqianTextContent
import org.tiqian.font.FontMetricsResolver
import org.tiqian.layout.ExplainableStubParagraphLayoutEngine
import org.tiqian.layout.LookaheadLineBreaker
import org.tiqian.layout.toPlanWithDiagnosticsJson
import org.tiqian.layout.toPreparedParagraphJson
import org.tiqian.protocol.DecorationInput
import org.tiqian.protocol.InlineBoxInput
import org.tiqian.protocol.InlineObjectInput
import org.tiqian.protocol.LineBreakSpanInput
import org.tiqian.protocol.ParagraphRequest
import org.tiqian.protocol.ParagraphRequestChecks
import org.tiqian.protocol.ParagraphRequestException
import org.tiqian.protocol.TextSpanInput
import org.tiqian.shaping.TextShaper

/**
 * Separator wire codec of the layout engine (ADR 0053 SingleEngineFace).
 * Wire decoding, validation and [LayoutInput] assembly live with the engine
 * so every JS host consumes one entry; ffi layers only transport strings.
 * The encoding itself is the ADR 0050 js ABI: record, field and family
 * separators, flat primitive parameters in, one plan JSON string out.
 *
 * This file now also provides DTO-based entry points (corrective wave 5/#106).
 * The legacy string-based entry points are retained temporarily for tests;
 * they are deleted when the last test migrates to DTO fixtures.
 *
 * Domain validation is the generated single-source request model
 * (Stage1-P5b): the parse functions only decode the wire packing and keep
 * the wire-shape issue names (Invalid*Wire, InvalidTextSpanItalic); every
 * domain check runs once through [ParagraphRequestChecks.validate] on the
 * generated [ParagraphRequest], and the caught
 * [ParagraphRequestException] is rethrown as an
 * [IllegalArgumentException] whose message is the published issue name —
 * the same message channel the replaced handwritten require blocks used.
 */

private const val RECORD_SEPARATOR = "\u001e"
private const val FIELD_SEPARATOR = "\u001d"
private const val FAMILY_SEPARATOR = "\u001f"

/** Validates the generated request and keeps the name-as-message channel. */
private fun validateRequest(request: ParagraphRequest) {
    try {
        ParagraphRequestChecks.validate(request)
    } catch (error: ParagraphRequestException) {
        throw IllegalArgumentException(error.message)
    }
}

private fun parseBoundaries(value: String): List<Int> =
    value.split(',')
        .filter(String::isNotBlank)
        .map { it.toInt() }

private fun parseDecorations(value: String): List<DecorationInput> =
    value.split(RECORD_SEPARATOR)
        .filter(String::isNotBlank)
        .map { record ->
            val fields = record.split(FIELD_SEPARATOR)
            require(fields.size == 3) { "InvalidDecorationWire" }
            DecorationInput(
                start = fields[0].toInt(),
                end = fields[1].toInt(),
                kind = fields[2],
            )
        }

private fun parseTextSpans(value: String): List<TextSpanInput> =
    value.split(RECORD_SEPARATOR)
        .filter(String::isNotBlank)
        .map { record ->
            val fields = record.split(FIELD_SEPARATOR)
            require(fields.size == 7) { "InvalidTextSpanWire" }
            TextSpanInput(
                start = fields[0].toInt(),
                end = fields[1].toInt(),
                families = fields[2].split(FAMILY_SEPARATOR).filter(String::isNotBlank).toMutableList(),
                fontSizePx = fields[3].toDouble(),
                fontWeight = fields[4].toInt(),
                italic = when (fields[5]) {
                    "true" -> true
                    "false" -> false
                    else -> error("InvalidTextSpanItalic")
                },
                baselineShift = fields[6].toDouble(),
            )
        }

private fun parseInlineBoxes(value: String): List<InlineBoxInput> =
    value.split(RECORD_SEPARATOR)
        .filter(String::isNotBlank)
        .map { record ->
            val fields = record.split(FIELD_SEPARATOR)
            require(fields.size == 4 || fields.size == 5) { "InvalidInlineBoxWire" }
            InlineBoxInput(
                start = fields[0].toInt(),
                end = fields[1].toInt(),
                inlineStart = fields[2].toDouble(),
                inlineEnd = fields[3].toDouble(),
                outerSpacing = fields.getOrNull(4) ?: "Narrow",
            )
        }

private fun parseLineBreakSpans(value: String): List<LineBreakSpanInput> =
    value.split(RECORD_SEPARATOR)
        .filter(String::isNotBlank)
        .map { record ->
            val fields = record.split(FIELD_SEPARATOR)
            require(fields.size == 3) { "InvalidLineBreakSpanWire" }
            LineBreakSpanInput(
                start = fields[0].toInt(),
                end = fields[1].toInt(),
                policy = fields[2],
            )
        }

private fun parseInlineObjects(value: String): List<InlineObjectInput> =
    value.split(RECORD_SEPARATOR)
        .filter(String::isNotBlank)
        .map { record ->
            val fields = record.split(FIELD_SEPARATOR)
            require(fields.size == 5) { "InvalidInlineObjectWire" }
            InlineObjectInput(
                start = fields[0].toInt(),
                end = fields[1].toInt(),
                advance = fields[2].toDouble(),
                ascent = fields[3].toDouble(),
                descent = fields[4].toDouble(),
            )
        }

private fun toInternal(span: TextSpanInput, locale: String): TextSpan =
    TextSpan(
        range = TextRange(span.start, span.end),
        style = TextStyle(
            fontFamilies = span.families.filter(String::isNotBlank),
            fontSize = span.fontSizePx.toFloat(),
            locale = locale,
            fontWeight = span.fontWeight,
            italic = span.italic,
            baselineShift = span.baselineShift.toFloat(),
        ),
    )

private fun toInternal(span: LineBreakSpanInput): LineBreakSpan =
    LineBreakSpan(TextRange(span.start, span.end), LineBreakPolicy.valueOf(span.policy))

private fun toInternal(box: InlineBoxInput): InlineBoxSpan =
    InlineBoxSpan(
        TextRange(box.start, box.end),
        box.inlineStart.toFloat(),
        box.inlineEnd.toFloat(),
        InlineBoxOuterSpacing.valueOf(box.outerSpacing),
    )

private fun toInternal(objectInput: InlineObjectInput): InlineObjectSpan =
    InlineObjectSpan(
        TextRange(objectInput.start, objectInput.end),
        objectInput.advance.toFloat(),
        objectInput.ascent.toFloat(),
        objectInput.descent.toFloat(),
    )

private fun toInternal(decoration: DecorationInput): DecorationSpan =
    DecorationSpan(
        range = TextRange(decoration.start, decoration.end),
        kind = DecorationKind.valueOf(decoration.kind),
    )

class ParagraphWireCodec(
    private val textShaper: TextShaper,
    private val fontMetricsResolver: FontMetricsResolver,
) {
    fun plan(
        text: String,
        maxWidthPx: Double,
        fontFamilies: String,
        fontSizePx: Double,
        lineHeightPx: Double,
        locale: String,
        fontWeight: Int,
        italic: Boolean,
        firstLineIndentIc: Double,
        lineLengthGridEnabled: Boolean,
        sourceBoundaries: String,
        textSpans: String,
        inlineBoxes: String,
        lineBreakSpans: String,
        inlineObjects: String = "",
        renderEvidenceOverride: Boolean? = null,
    ): String {
        val result = layout(
            text = text,
            maxWidthPx = maxWidthPx,
            fontFamilies = fontFamilies,
            fontSizePx = fontSizePx,
            lineHeightPx = lineHeightPx,
            locale = locale,
            fontWeight = fontWeight,
            italic = italic,
            firstLineIndentIc = firstLineIndentIc,
            lineLengthGridEnabled = lineLengthGridEnabled,
            sourceBoundaries = sourceBoundaries,
            textSpans = textSpans,
            inlineBoxes = inlineBoxes,
            lineBreakSpans = lineBreakSpans,
            inlineObjects = inlineObjects,
        )
        // WorkerRichPlanEvidence: the Worker runs the pure exact session, so
        // evidence exists for exactly the non-plain wire shapes the runtime
        // path also evidences (inline objects and styled/boxed runs); plain
        // plans stay byte-identical to the evidence-free form.
        // The wire derives evidence from the wire-visible collections. The host
        // passes the six-collection verdict as the override because sourceSpans
        // and domInlineObjects never travel the wire.
        return result.toPreparedParagraphJson(
            renderEvidence = renderEvidenceOverride ?: (textSpans.isNotBlank() ||
                inlineBoxes.isNotBlank() ||
                result.input.inlineObjects.isNotEmpty()),
        )
    }

    /**
     * Plan-plus-diagnostics envelope for the TsHost worker/precompute path.
     * [zeroAdvanceEpsilonPx] is the host threshold (ZERO_ADVANCE_EPSILON on
     * the web host), passed in so the layout module holds no host policy.
     * Diagnostics carry facts only — the verdicts for the web pipeline's
     * named capability checks stay host-side.
     */
    fun planWithDiagnostics(
        text: String,
        maxWidthPx: Double,
        fontFamilies: String,
        fontSizePx: Double,
        lineHeightPx: Double,
        locale: String,
        fontWeight: Int,
        italic: Boolean,
        firstLineIndentIc: Double,
        lineLengthGridEnabled: Boolean,
        sourceBoundaries: String,
        textSpans: String,
        inlineBoxes: String,
        lineBreakSpans: String,
        inlineObjects: String = "",
        zeroAdvanceEpsilonPx: Double,
        decorations: String = "",
        emphasisDotGapEm: Double? = null,
        renderEvidenceOverride: Boolean? = null,
    ): String {
        val result = layout(
            text = text,
            maxWidthPx = maxWidthPx,
            fontFamilies = fontFamilies,
            fontSizePx = fontSizePx,
            lineHeightPx = lineHeightPx,
            locale = locale,
            fontWeight = fontWeight,
            italic = italic,
            firstLineIndentIc = firstLineIndentIc,
            lineLengthGridEnabled = lineLengthGridEnabled,
            sourceBoundaries = sourceBoundaries,
            textSpans = textSpans,
            inlineBoxes = inlineBoxes,
            lineBreakSpans = lineBreakSpans,
            inlineObjects = inlineObjects,
            decorations = decorations,
            emphasisDotGapEm = emphasisDotGapEm,
        )
        return result.toPlanWithDiagnosticsJson(
            // The wire derives evidence from the wire-visible collections. The
            // host passes the six-collection verdict as the override because
            // sourceSpans and domInlineObjects never travel the wire.
            renderEvidence = renderEvidenceOverride ?: (textSpans.isNotBlank() ||
                inlineBoxes.isNotBlank() ||
                decorations.isNotBlank() ||
                result.input.inlineObjects.isNotEmpty()),
            zeroAdvanceEpsilonPx = zeroAdvanceEpsilonPx.toFloat(),
        )
    }

    private fun layout(
        text: String,
        maxWidthPx: Double,
        fontFamilies: String,
        fontSizePx: Double,
        lineHeightPx: Double,
        locale: String,
        fontWeight: Int,
        italic: Boolean,
        firstLineIndentIc: Double,
        lineLengthGridEnabled: Boolean,
        sourceBoundaries: String,
        textSpans: String,
        inlineBoxes: String,
        lineBreakSpans: String,
        inlineObjects: String = "",
        decorations: String = "",
        emphasisDotGapEm: Double? = null,
    ): LayoutResult {
        val families = fontFamilies.split(FAMILY_SEPARATOR).filter(String::isNotBlank)
        val request = ParagraphRequest(
            // The wire shape carries no font session id; the field exists in
            // the shared model for the lanes that have one.
            fontSessionId = "",
            text = text,
            maxWidthPx = maxWidthPx,
            fontFamilies = families.toMutableList(),
            fontSizePx = fontSizePx,
            lineHeightPx = lineHeightPx,
            locale = locale,
            fontWeight = fontWeight,
            italic = italic,
            firstLineIndentIc = firstLineIndentIc,
            lineLengthGridEnabled = lineLengthGridEnabled,
            emphasisDotGapEm = emphasisDotGapEm,
            sourceBoundaries = parseBoundaries(sourceBoundaries).toMutableList(),
            textSpans = parseTextSpans(textSpans).toMutableList(),
            lineBreakSpans = parseLineBreakSpans(lineBreakSpans).toMutableList(),
            inlineBoxes = parseInlineBoxes(inlineBoxes).toMutableList(),
            inlineObjects = parseInlineObjects(inlineObjects).toMutableList(),
            decorations = parseDecorations(decorations).toMutableList(),
        )
        validateRequest(request)
        val gapEm = emphasisDotGapEm ?: DEFAULT_EMPHASIS_DOT_GAP_EM.toDouble()
        val input = LayoutInput(
            content = TiqianTextContent(
                text = text,
                spans = request.textSpans.map { toInternal(it, locale) },
                sourceBoundaries = request.sourceBoundaries.toSet(),
                lineBreakSpans = request.lineBreakSpans.map { toInternal(it) },
            ),
            textStyle = TextStyle(
                fontFamilies = families,
                fontSize = fontSizePx.toFloat(),
                locale = locale,
                fontWeight = fontWeight,
                italic = italic,
            ),
            paragraphStyle = ParagraphStyle(
                lineHeight = lineHeightPx.toFloat(),
                firstLineIndent = Ic(firstLineIndentIc.toFloat()),
                lineLengthGrid = LineLengthGrid(enabled = lineLengthGridEnabled),
                emphasisDotGapEm = gapEm.toFloat(),
            ),
            constraints = LayoutConstraints(maxWidth = maxWidthPx.toFloat()),
            decorations = request.decorations.map { toInternal(it) },
            inlineBoxes = request.inlineBoxes.map { toInternal(it) },
            inlineObjects = request.inlineObjects.map { toInternal(it) },
        )
        return ExplainableStubParagraphLayoutEngine(
            lineBreaker = LookaheadLineBreaker(),
            fontMetricsResolver = fontMetricsResolver,
            textShaper = textShaper,
        ).layout(input)
    }

    // DTO-based entry points (corrective wave 5/#106)
    // These replace the legacy string-based wire format.

    fun plan(request: WorkerLayoutRequestDto): String {
        val result = layout(request)
        return result.toPreparedParagraphJson(
            renderEvidence = request.renderEvidence,
        )
    }

    fun planWithDiagnostics(
        request: PrepareParagraphRequestDto,
        zeroAdvanceEpsilonPx: Double,
    ): String {
        val result = layout(request)
        return result.toPlanWithDiagnosticsJson(
            renderEvidence = request.renderEvidenceOverride ?: (request.textSpans.isNotEmpty() ||
                request.inlineBoxes.isNotEmpty() ||
                request.decorations.isNotEmpty() ||
                result.input.inlineObjects.isNotEmpty()),
            zeroAdvanceEpsilonPx = zeroAdvanceEpsilonPx.toFloat(),
        )
    }

    private fun layout(request: WorkerLayoutRequestDto): LayoutResult {
        return layout(
            text = request.text,
            maxWidthPx = request.maxWidthPx,
            fontFamilies = request.fontFamilies.joinToString(FAMILY_SEPARATOR),
            fontSizePx = request.fontSizePx,
            lineHeightPx = request.lineHeightPx,
            locale = request.locale,
            fontWeight = request.fontWeight,
            italic = request.italic,
            firstLineIndentIc = request.firstLineIndentIc,
            lineLengthGridEnabled = request.lineLengthGridEnabled,
            sourceBoundaries = request.sourceBoundaries.joinToString(","),
            textSpans = request.textSpans.joinToString(RECORD_SEPARATOR) { span ->
                "${span.start}${FIELD_SEPARATOR}${span.end}${FIELD_SEPARATOR}" +
                "${span.fontFamilies.joinToString(FAMILY_SEPARATOR)}${FIELD_SEPARATOR}" +
                "${span.fontSize}${FIELD_SEPARATOR}${span.fontWeight}${FIELD_SEPARATOR}" +
                "${span.italic}${FIELD_SEPARATOR}${span.baselineShift}"
            },
            inlineBoxes = request.inlineBoxes.joinToString(RECORD_SEPARATOR) { box ->
                "${box.start}${FIELD_SEPARATOR}${box.end}${FIELD_SEPARATOR}" +
                "${box.inlineStart}${FIELD_SEPARATOR}${box.inlineEnd}${FIELD_SEPARATOR}" +
                "${box.outerSpacing}"
            },
            lineBreakSpans = request.lineBreakSpans.joinToString(RECORD_SEPARATOR) { span ->
                "${span.start}${FIELD_SEPARATOR}${span.end}${FIELD_SEPARATOR}${span.policy}"
            },
            inlineObjects = request.inlineObjects.joinToString(RECORD_SEPARATOR) { obj ->
                "${obj.start}${FIELD_SEPARATOR}${obj.end}${FIELD_SEPARATOR}" +
                "${obj.advance}${FIELD_SEPARATOR}${obj.ascent}${FIELD_SEPARATOR}${obj.descent}"
            },
        )
    }

    private fun layout(request: PrepareParagraphRequestDto): LayoutResult {
        val model = ParagraphRequest(
            fontSessionId = "",
            text = request.text,
            maxWidthPx = request.maxWidthPx,
            fontFamilies = request.fontFamilies.toMutableList(),
            fontSizePx = request.fontSizePx,
            lineHeightPx = request.lineHeightPx,
            locale = request.locale,
            fontWeight = request.fontWeight,
            italic = request.italic,
            firstLineIndentIc = request.firstLineIndentIc,
            lineLengthGridEnabled = request.lineLengthGridEnabled,
            emphasisDotGapEm = request.emphasisDotGapEm,
            sourceBoundaries = request.sourceBoundaries.toMutableList(),
            textSpans = request.textSpans.map { span ->
                TextSpanInput(
                    start = span.start,
                    end = span.end,
                    families = span.fontFamilies.toMutableList(),
                    fontSizePx = span.fontSize,
                    fontWeight = span.fontWeight,
                    italic = span.italic,
                    baselineShift = span.baselineShift,
                )
            }.toMutableList(),
            lineBreakSpans = request.lineBreakSpans.map { span ->
                LineBreakSpanInput(start = span.start, end = span.end, policy = span.policy)
            }.toMutableList(),
            inlineBoxes = request.inlineBoxes.map { box ->
                InlineBoxInput(
                    start = box.start,
                    end = box.end,
                    inlineStart = box.inlineStart,
                    inlineEnd = box.inlineEnd,
                    outerSpacing = box.outerSpacing,
                )
            }.toMutableList(),
            inlineObjects = request.inlineObjects.map { obj ->
                InlineObjectInput(
                    start = obj.start,
                    end = obj.end,
                    advance = obj.advance,
                    ascent = obj.ascent,
                    descent = obj.descent,
                )
            }.toMutableList(),
            decorations = request.decorations.map { deco ->
                DecorationInput(start = deco.start, end = deco.end, kind = deco.kind)
            }.toMutableList(),
        )
        validateRequest(model)
        val gapEm = request.emphasisDotGapEm ?: DEFAULT_EMPHASIS_DOT_GAP_EM.toDouble()
        val input = LayoutInput(
            content = TiqianTextContent(
                text = request.text,
                spans = model.textSpans.map { toInternal(it, request.locale) },
                sourceBoundaries = model.sourceBoundaries.toSet(),
                lineBreakSpans = model.lineBreakSpans.map { toInternal(it) },
            ),
            textStyle = TextStyle(
                fontFamilies = model.fontFamilies.filter(String::isNotBlank),
                fontSize = request.fontSizePx.toFloat(),
                locale = request.locale,
                fontWeight = request.fontWeight,
                italic = request.italic,
            ),
            paragraphStyle = ParagraphStyle(
                lineHeight = request.lineHeightPx.toFloat(),
                firstLineIndent = Ic(request.firstLineIndentIc.toFloat()),
                lineLengthGrid = LineLengthGrid(enabled = request.lineLengthGridEnabled),
                emphasisDotGapEm = gapEm.toFloat(),
            ),
            constraints = LayoutConstraints(maxWidth = request.maxWidthPx.toFloat()),
            decorations = model.decorations.map { toInternal(it) },
            inlineBoxes = model.inlineBoxes.map { toInternal(it) },
            inlineObjects = model.inlineObjects.map { toInternal(it) },
        )
        return ExplainableStubParagraphLayoutEngine(
            lineBreaker = LookaheadLineBreaker(),
            fontMetricsResolver = fontMetricsResolver,
            textShaper = textShaper,
        ).layout(input)
    }
}
