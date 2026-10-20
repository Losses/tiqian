package org.tiqian.shared.fontface;

// Shared layer: CSSOM @font-face boundary parsing (boring single-source, generated to TS).
// Consumes protocol types (FontFace/LayoutTree/FontSession, landed main=17062f27) only as
// plain concrete data; this module is file-domain-isolated from sibling shared modules
// (sh-3 Html, sh-4 SnapshotSource, sh-5 PreparedDom).
//
// Canonical behavior mirrors platforms/web/client/core/src/sampler/font-face-boundaries.ts
// (the TS hand-written original this replaces) byte-for-byte on boundary OUTPUT.

class FontFaceBoundaries {
    static final FAMILY_SEPARATOR = "\u001f";
    static final RECORD_SEPARATOR = "\u001e";
    static final FIELD_SEPARATOR = "\u001d";

    public static function workerSnapshotSubsetSourceBoundaries(faceMetadata: Array<FontFaceRecord>, request: WorkerFontContractRequest): Array<Float> {
        final faces: Array<SelectedMetadataFace> = [];
        var idx = 0;
        while (idx < faceMetadata.length) {
            final face = faceMetadata[idx];
            faces.push({
                family: face.family,
                localNames: face.localNames,
                style: face.style,
                weight: face.weight,
                unicodeRange: face.unicodeRange,
                publicUrl: face.publicUrl,
                faceIndex: face.faceIndex,
                sourceOrder: face.sourceOrder,
                unicodeRanges: parseUnicodeRange(face.unicodeRange),
            });
            idx += 1;
        }
        final baseStyle: FontFaceStyle = {
            fontFamilies: splitFontFamilies(request.fontFamilies),
            fontSizePx: request.fontSizePx,
            fontWeight: request.fontWeight,
            italic: request.italic,
            baselineShiftPx: 0.0,
        };
        if (faces.length == 0 || baseStyle.fontFamilies.length == 0
            || !Math.isFinite(baseStyle.fontSizePx) || baseStyle.fontSizePx <= 0.0
            || !Math.isFinite(baseStyle.fontWeight)) {
            throw new FontFaceBoundaryException("InvalidLayoutWorkerFontContract");
        }
        return sourceBoundariesForSelectedFace(request.text, baseStyle, workerTextSpans(request.textSpans), faces);
    }

    public static function mergeSerializedSourceBoundaries(existing: Array<Float>, additional: Array<Float>): String {
        final boundaries: Array<Float> = [];
        var idx = 0;
        while (idx < existing.length) {
            if (!containsFloat(boundaries, existing[idx])) {
                boundaries.push(existing[idx]);
            }
            idx += 1;
        }
        var addIdx = 0;
        while (addIdx < additional.length) {
            if (!containsFloat(boundaries, additional[addIdx])) {
                boundaries.push(additional[addIdx]);
            }
            addIdx += 1;
        }
        var vIdx = 0;
        while (vIdx < boundaries.length) {
            final b = boundaries[vIdx];
            if (!isSafeInteger(b) || b < 0.0) {
                throw new FontFaceBoundaryException("InvalidSourceBoundary");
            }
            vIdx += 1;
        }
        sortFloatsAscending(boundaries);
        return boundaries.join(",");
    }

    static function sourceBoundariesForSelectedFace(text: String, baseStyle: FontFaceStyle, spans: Array<WorkerTextSpan>, faces: Array<SelectedMetadataFace>): Array<Float> {
        final boundaries: Array<Float> = [];
        var offset = 0.0;
        var previousSignature: Null<String> = null;
        var i = 0;
        final len = text.length;
        while (i < len) {
            var codePoint = text.charCodeAt(i);
            var isPair = false;
            if (codePoint >= 0xD800 && codePoint <= 0xDBFF && i + 1 < len) {
                final low = text.charCodeAt(i + 1);
                if (low >= 0xDC00 && low <= 0xDFFF) {
                    codePoint = 0x10000 + ((codePoint - 0xD800) << 10) + (low - 0xDC00);
                    isPair = true;
                }
            }
            if (isStructuralNoShapeControl(codePoint)) {
                offset += isPair ? 2.0 : 1.0;
                i += isPair ? 2 : 1;
                continue;
            }
            final style = findStyle(spans, offset, baseStyle);
            final record = selectMetadataFace(faces, style, codePoint);
            final signature = signatureOf(record, style);
            if (previousSignature != null && signature != previousSignature) {
                boundaries.push(offset);
            }
            previousSignature = signature;
            offset += isPair ? 2.0 : 1.0;
            i += isPair ? 2 : 1;
        }
        return boundaries;
    }

    static function findStyle(spans: Array<WorkerTextSpan>, offset: Float, baseStyle: FontFaceStyle): FontFaceStyle {
        var idx = spans.length - 1;
        while (idx >= 0) {
            final span = spans[idx];
            if (offset >= span.start && offset < span.end) {
                return span;
            }
            idx -= 1;
        }
        return baseStyle;
    }

    static function signatureOf(record: SelectedMetadataFace, style: FontFaceStyle): String {
        return metadataFaceIdentity(record)
            + "|" + style.fontFamilies.join(FAMILY_SEPARATOR)
            + "|" + Std.string(style.fontSizePx)
            + "|" + Std.string(style.fontWeight)
            + "|" + (style.italic ? "true" : "false")
            + "|" + Std.string(style.baselineShiftPx);
    }

    static function metadataFaceIdentity(face: SelectedMetadataFace): String {
        return face.family
            + FAMILY_SEPARATOR + face.style
            + FAMILY_SEPARATOR + face.weight.join(",")
            + FAMILY_SEPARATOR + face.unicodeRange
            + FAMILY_SEPARATOR + face.publicUrl
            + FAMILY_SEPARATOR + Std.string(face.faceIndex)
            + FAMILY_SEPARATOR + Std.string(face.sourceOrder);
    }

    static function selectMetadataFace(faces: Array<SelectedMetadataFace>, style: FontFaceStyle, codePoint: Int): SelectedMetadataFace {
        final desiredStyle = style.italic ? "italic" : "normal";
        var familyIdx = 0;
        while (familyIdx < style.fontFamilies.length) {
            final family = style.fontFamilies[familyIdx];
            final familyMatches: Array<SelectedMetadataFace> = [];
            var faceIdx = 0;
            while (faceIdx < faces.length) {
                final face = faces[faceIdx];
                if (fontRecordMatchesFamily(face, family) && face.style == desiredStyle) {
                    familyMatches.push(face);
                }
                faceIdx += 1;
            }
            final weightMatched = cssWeightMatched(familyMatches, style.fontWeight);
            var mIdx = weightMatched.length - 1;
            while (mIdx >= 0) {
                final face = weightMatched[mIdx];
                if (unicodeRangeContains(face.unicodeRanges, codePoint)) {
                    return face;
                }
                mIdx -= 1;
            }
            familyIdx += 1;
        }
        throw new FontFaceBoundaryException("NoSnapshotFontFace:families=" + style.fontFamilies.join(",")
            + ";weight=" + Std.string(style.fontWeight)
            + ";italic=" + (style.italic ? "true" : "false")
            + ";text=" + Std.string(codePoint));
    }

    static function cssWeightMatched(records: Array<SelectedMetadataFace>, requested: Float): Array<SelectedMetadataFace> {
        if (records.length <= 1) {
            return records;
        }
        final ranked: Array<SelectedMetadataFace> = [];
        final ranks: Array<Array<Float>> = [];
        var idx = 0;
        while (idx < records.length) {
            ranked.push(records[idx]);
            ranks.push(cssWeightPreference(records[idx].weight, requested));
            idx += 1;
        }
        var i = 1;
        while (i < ranked.length) {
            final keyRecord = ranked[i];
            final keyRank = ranks[i];
            var j = i - 1;
            while (j >= 0 && rankLess(keyRank, ranks[j])) {
                ranked[j + 1] = ranked[j];
                ranks[j + 1] = ranks[j];
                j -= 1;
            }
            ranked[j + 1] = keyRecord;
            ranks[j + 1] = keyRank;
            i += 1;
        }
        final best = ranks[0];
        final result: Array<SelectedMetadataFace> = [];
        var k = 0;
        while (k < ranked.length) {
            if (ranks[k][0] == best[0] && ranks[k][1] == best[1]) {
                result.push(ranked[k]);
            }
            k += 1;
        }
        return result;
    }

    static function rankLess(a: Array<Float>, b: Array<Float>): Bool {
        if (a[0] < b[0]) {
            return true;
        }
        if (a[0] > b[0]) {
            return false;
        }
        return a[1] < b[1];
    }

    static function cssWeightPreference(range: Array<Float>, requested: Float): Array<Float> {
        if (range.length < 2) {
            return [Math.POSITIVE_INFINITY, Math.POSITIVE_INFINITY];
        }
        final low = range[0];
        final high = range[1];
        if (!Math.isFinite(low) || !Math.isFinite(high) || high < low) {
            return [Math.POSITIVE_INFINITY, Math.POSITIVE_INFINITY];
        }
        if (requested >= low && requested <= high) {
            return [0.0, 0.0];
        }
        if (requested >= 400.0 && requested <= 500.0) {
            if (low > requested && low <= 500.0) {
                return [1.0, low - requested];
            }
            if (high < requested) {
                return [2.0, requested - high];
            }
            return [3.0, low - 500.0];
        }
        if (requested < 400.0) {
            return high < requested ? [1.0, requested - high] : [2.0, low - requested];
        }
        return low > requested ? [1.0, low - requested] : [2.0, requested - high];
    }

    static function fontRecordMatchesFamily(record: FontFaceRecord, requestedFamily: String): Bool {
        final requested = StringTools.trim(requestedFamily).toLowerCase();
        if (requested == "") {
            return false;
        }
        if (record.family.toLowerCase() == requested) {
            return true;
        }
        final names = record.localNames;
        if (names != null) {
            var idx = 0;
            while (idx < names.length) {
                if (StringTools.trim(names[idx]).toLowerCase() == requested) {
                    return true;
                }
                idx += 1;
            }
        }
        return false;
    }

    public static function parseUnicodeRange(value: String): Null<Array<Array<Int>>> {
        final serialized = StringTools.trim(value);
        if (serialized == "") {
            return null;
        }
        final ranges: Array<Array<Int>> = [];
        final items = serialized.split(",");
        var itemIdx = 0;
        while (itemIdx < items.length) {
            final token = StringTools.trim(items[itemIdx]).toUpperCase();
            if (token.length >= 2 && token.substr(0, 2) == "U+") {
                final body = token.substr(2);
                if (isHexOrQuestion(body) && body.indexOf("?") >= 0) {
                    ranges.push([
                        parseHexWithQuestion(body, false),
                        parseHexWithQuestion(body, true),
                    ]);
                } else {
                    final pair = parseHexPair(body);
                    if (pair != null) {
                        ranges.push(pair);
                    }
                }
            }
            itemIdx += 1;
        }
        return ranges;
    }

    static function unicodeRangeContains(ranges: Null<Array<Array<Int>>>, codePoint: Int): Bool {
        if (ranges == null) {
            return true;
        }
        var idx = 0;
        while (idx < ranges.length) {
            final range = ranges[idx];
            if (codePoint >= range[0] && codePoint <= range[1]) {
                return true;
            }
            idx += 1;
        }
        return false;
    }

    static function workerTextSpans(value: Null<String>): Array<WorkerTextSpan> {
        if (value == null || value == "") {
            return [];
        }
        final result: Array<WorkerTextSpan> = [];
        final records = value.split(RECORD_SEPARATOR);
        var recIdx = 0;
        while (recIdx < records.length) {
            final fields = records[recIdx].split(FIELD_SEPARATOR);
            if (fields.length != 7) {
                throw new FontFaceBoundaryException("InvalidLayoutWorkerTextSpan");
            }
            final start = parseNumber(fields[0]);
            final end = parseNumber(fields[1]);
            final fontFamilies = splitFontFamilies(fields[2]);
            final fontSizePx = parseNumber(fields[3]);
            final fontWeight = parseNumber(fields[4]);
            final italicString = fields[5];
            final baselineShiftPx = parseNumber(fields[6]);
            final italic = italicString == "true";
            final span: WorkerTextSpan = {
                start: start,
                end: end,
                fontFamilies: fontFamilies,
                fontSizePx: fontSizePx,
                fontWeight: fontWeight,
                italic: italic,
                baselineShiftPx: baselineShiftPx,
            };
            if (!isSafeInteger(span.start) || !isSafeInteger(span.end)
                || span.start < 0.0 || span.end <= span.start
                || span.fontFamilies.length == 0
                || !Math.isFinite(span.fontSizePx) || span.fontSizePx <= 0.0
                || !Math.isFinite(span.fontWeight)
                || !Math.isFinite(span.baselineShiftPx)
                || (italicString != "true" && italicString != "false")) {
                throw new FontFaceBoundaryException("InvalidLayoutWorkerTextSpan");
            }
            result.push(span);
            recIdx += 1;
        }
        return result;
    }

    static function splitFontFamilies(serialized: String): Array<String> {
        final result: Array<String> = [];
        final parts = serialized.split(FAMILY_SEPARATOR);
        var idx = 0;
        while (idx < parts.length) {
            if (parts[idx] != "") {
                result.push(parts[idx]);
            }
            idx += 1;
        }
        return result;
    }

    static function parseNumber(text: String): Float {
        final trimmed = StringTools.trim(text);
        if (trimmed == "") {
            return Math.NaN;
        }
        var sign = 1.0;
        var start = 0;
        if (trimmed.substr(0, 1) == "-") {
            sign = -1.0;
            start = 1;
        } else if (trimmed.substr(0, 1) == "+") {
            start = 1;
        }
        if (start >= trimmed.length) {
            return Math.NaN;
        }
        var value = 0.0;
        var idx = start;
        var sawDigit = false;
        while (idx < trimmed.length) {
            final digit = decimalDigitValue(trimmed.charCodeAt(idx));
            if (digit < 0) {
                break;
            }
            value = value * 10.0 + digit;
            sawDigit = true;
            idx += 1;
        }
        if (idx < trimmed.length && trimmed.substr(idx, 1) == ".") {
            idx += 1;
            var frac = 0.1;
            while (idx < trimmed.length) {
                final digit = decimalDigitValue(trimmed.charCodeAt(idx));
                if (digit < 0) {
                    break;
                }
                value = value + digit * frac;
                frac = frac * 0.1;
                sawDigit = true;
                idx += 1;
            }
        }
        if (!sawDigit || idx != trimmed.length) {
            return Math.NaN;
        }
        return value * sign;
    }

    static function isSafeInteger(value: Float): Bool {
        return value % 1.0 == 0.0 && Math.abs(value) <= 9007199254740991.0;
    }

    static function isStructuralNoShapeControl(codePoint: Int): Bool {
        return codePoint == 0x000A || codePoint == 0x000B || codePoint == 0x000C
            || codePoint == 0x000D || codePoint == 0x0085 || codePoint == 0x200B
            || codePoint == 0x2028 || codePoint == 0x2029;
    }

    static function containsFloat(arr: Array<Float>, value: Float): Bool {
        var idx = 0;
        while (idx < arr.length) {
            if (arr[idx] == value) {
                return true;
            }
            idx += 1;
        }
        return false;
    }

    static function sortFloatsAscending(arr: Array<Float>): Void {
        var i = 1;
        while (i < arr.length) {
            final key = arr[i];
            var j = i - 1;
            while (j >= 0 && arr[j] > key) {
                arr[j + 1] = arr[j];
                j -= 1;
            }
            arr[j + 1] = key;
            i += 1;
        }
    }

    static function parseHexPair(text: String): Null<Array<Int>> {
        final dash = text.indexOf("-");
        if (dash >= 0) {
            final left = text.substr(0, dash);
            final right = text.substr(dash + 1);
            if (!isHex(left) || !isHex(right)) {
                return null;
            }
            return [parseHex(left), parseHex(right)];
        }
        if (!isHex(text)) {
            return null;
        }
        final value = parseHex(text);
        return [value, value];
    }

    static function parseHex(text: String): Int {
        var value = 0;
        var idx = 0;
        while (idx < text.length) {
            value = value * 16 + hexDigitValue(text.charCodeAt(idx));
            idx += 1;
        }
        return value;
    }

    static function parseHexWithQuestion(text: String, max: Bool): Int {
        var value = 0;
        var idx = 0;
        while (idx < text.length) {
            final code = text.charCodeAt(idx);
            var digit = 0;
            if (code == 63) {
                if (max) {
                    digit = 15;
                } else {
                    digit = 0;
                }
            } else {
                digit = hexDigitValue(code);
            }
            value = value * 16 + digit;
            idx += 1;
        }
        return value;
    }

    static function isHex(text: String): Bool {
        if (text.length < 1 || text.length > 6) {
            return false;
        }
        var idx = 0;
        while (idx < text.length) {
            final code = text.charCodeAt(idx);
            if (!((code >= 48 && code <= 57) || (code >= 65 && code <= 70))) {
                return false;
            }
            idx += 1;
        }
        return true;
    }

    static function isHexOrQuestion(text: String): Bool {
        if (text.length < 1 || text.length > 6) {
            return false;
        }
        var idx = 0;
        while (idx < text.length) {
            final code = text.charCodeAt(idx);
            if (!((code >= 48 && code <= 57) || (code >= 65 && code <= 70) || code == 63)) {
                return false;
            }
            idx += 1;
        }
        return true;
    }

    static function hexDigitValue(code: Int): Int {
        if (code >= 48 && code <= 57) {
            return code - 48;
        }
        return code - 55;
    }

    static function decimalDigitValue(code: Int): Int {
        if (code >= 48 && code <= 57) {
            return code - 48;
        }
        return -1;
    }
}

typedef FontFaceRecord = {
    var family: String;
    var localNames: Null<Array<String>>;
    var style: String;
    var weight: Array<Float>;
    var unicodeRange: String;
    var publicUrl: String;
    var faceIndex: Float;
    var sourceOrder: Float;
}

typedef FontFaceStyle = {
    var fontFamilies: Array<String>;
    var fontSizePx: Float;
    var fontWeight: Float;
    var italic: Bool;
    var baselineShiftPx: Float;
}

typedef WorkerTextSpan = {
    var start: Float;
    var end: Float;
    var fontFamilies: Array<String>;
    var fontSizePx: Float;
    var fontWeight: Float;
    var italic: Bool;
    var baselineShiftPx: Float;
}

typedef WorkerFontContractRequest = {
    var text: String;
    var fontFamilies: String;
    var fontSizePx: Float;
    var fontWeight: Float;
    var italic: Bool;
    var textSpans: Null<String>;
}

typedef SelectedMetadataFace = {
    var family: String;
    var localNames: Null<Array<String>>;
    var style: String;
    var weight: Array<Float>;
    var unicodeRange: String;
    var publicUrl: String;
    var faceIndex: Float;
    var sourceOrder: Float;
    var unicodeRanges: Null<Array<Array<Int>>>;
}
