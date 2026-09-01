package org.tiqian.layout;

import org.tiqian.core.UnicodeEastAsianSpacing;
import org.tiqian.core.UnicodeScriptEvidence;
import org.tiqian.core.UnicodeScriptEvidenceClassifier;
import org.tiqian.core.UnicodeNumber;
import org.tiqian.core.UnicodeWordCharacter;
import org.tiqian.font.FontRole;
import org.tiqian.font.FontRoleContext;

class ContextualQuoteRoleResolver {
    private final text:String;
    private final pairs:Array<QuotePair>;
    private final context:FontRoleContext;
    private final pairByOpen:Map<Int, QuotePair> = new Map<Int, QuotePair>();
    private final pairByClose:Map<Int, QuotePair> = new Map<Int, QuotePair>();
    private final parentByPair:Map<QuotePair, Null<QuotePair>> = new Map<QuotePair, Null<QuotePair>>();

    public function new(text:String, pairs:Array<QuotePair>, context:FontRoleContext) {
        this.text = text;
        this.pairs = pairs;
        this.context = context;
        for (pair in pairs) {
            pairByOpen.set(pair.openIndex, pair);
            pairByClose.set(pair.closeIndex, pair);
        }
        for (pair in pairs) {
            parentByPair.set(pair, findParent(pair));
        }
    }

    public function resolve():Array<QuoteRoleDecision> {
        final decisions:Array<QuoteRoleDecision> = [];
        final resolvedPairs:Map<QuotePair, FontRole> = new Map<QuotePair, FontRole>();
        final ordered:Array<QuotePair> = pairs.copy();
        ordered.sort(function(left:QuotePair, right:QuotePair):Int {
            final byOpen:Int = left.openIndex - right.openIndex;
            return byOpen != 0 ? byOpen : right.closeIndex - left.closeIndex;
        });

        for (pair in ordered) {
            final resolution:Resolution = resolvePair(pair, resolvedPairs);
            resolvedPairs.set(pair, resolution.role);
            decisions.push(new QuoteRoleDecision(pair.openIndex, resolution.role, resolution.source, resolution.reason));
            decisions.push(new QuoteRoleDecision(pair.closeIndex, resolution.role, resolution.source, resolution.reason));
        }

        final pairedIndices:Map<Int, Bool> = new Map<Int, Bool>();
        for (pair in pairs) {
            pairedIndices.set(pair.openIndex, true);
            pairedIndices.set(pair.closeIndex, true);
        }
        for (index in 0...text.length) {
            if (pairedIndices.exists(index) || !isAmbiguousCurlyQuote(text.charCodeAt(index))) {
                continue;
            }
            final resolution:Resolution = resolveUnmatched(index);
            decisions.push(new QuoteRoleDecision(index, resolution.role, resolution.source, resolution.reason));
        }
        decisions.sort(function(left:QuoteRoleDecision, right:QuoteRoleDecision):Int return left.index - right.index);
        return decisions;
    }

    private function resolvePair(pair:QuotePair, resolvedPairs:Map<QuotePair, FontRole>):Resolution {
        final parent:Null<QuotePair> = parentByPair.get(pair);
        final enclosingStart:Int = parent == null ? 0 : parent.openIndex + 1;
        final enclosingEnd:Int = parent == null ? text.length : parent.closeIndex;
        final outerEvidence:ScriptEvidenceState = new ScriptEvidenceState();
        outerEvidence.addRange(pairByOpen, enclosingStart, pair.openIndex, text);
        outerEvidence.addRange(pairByOpen, pair.closeIndex + 1, enclosingEnd, text);
        final contentEvidence:ScriptEvidenceState = new ScriptEvidenceState();
        contentEvidence.addRange(pairByOpen, pair.openIndex + 1, pair.closeIndex, text);

        if (charAt(pair.openIndex - 1) != null && isAsciiSpaceOrTab(charAt(pair.openIndex - 1))
            && contentEvidence.hasWestern && !contentEvidence.hasCjk) {
            return new Resolution(FontRole.LatinText, "DelimitedWesternQuotationRun", "whitespace-delimited-wholly-western-quotation");
        }
        if (isNonCjkWordInternalQuotePair(pair)) {
            return new Resolution(FontRole.LatinText, "NonCjkWordInternalQuotePair", "non-cjk-word-internal-quotation");
        }
        final outerRole:Null<FontRole> = outerEvidence.unambiguousRole();
        if (outerRole != null) {
            return new Resolution(outerRole, "PairedPunctuationOuterScriptContext", "quote-pair-inherits-enclosing-level-script");
        }
        if (outerEvidence.isMixed) {
            return paragraphLanguageResolution("mixed-enclosing-level-script");
        }
        if (parent != null) {
            final enclosingRole:Null<FontRole> = resolvedPairs.get(parent);
            if (enclosingRole != null) {
                return new Resolution(enclosingRole, "PairedPunctuationEnclosingQuoteContext", "quote-pair-inherits-enclosing-quotation");
            }
        }
        final contentRole:Null<FontRole> = contentEvidence.unambiguousRole();
        if (contentRole != null) {
            return new Resolution(contentRole, "PairedPunctuationContentScriptContext", "quoted-content-script");
        }
        return paragraphLanguageResolution(contentEvidence.isMixed ? "mixed-quoted-content" : "no-strong-script-context");
    }

    private function resolveUnmatched(index:Int):Resolution {
        if (text.charCodeAt(index) == 0x2019 && isNonCjkInWordApostrophe(index)) {
            return new Resolution(FontRole.LatinText, "NonCjkInWordApostrophe", "non-cjk-in-word-apostrophe");
        }
        if (isDigitBoundClosingQuote(index)) {
            return new Resolution(FontRole.LatinText, "NumericPrimeUnmatchedQuote", "digit-bound-unmatched-quote-as-prime");
        }
        final leftRole:Null<FontRole> = nearestStrongScriptRole(index - 1, -1);
        final rightRole:Null<FontRole> = nearestStrongScriptRole(index + 1, 1);
        if (isAsciiSpaceOrTab(charAt(index - 1)) && rightRole == FontRole.LatinText) {
            return new Resolution(FontRole.LatinText, "DelimitedUnmatchedWesternQuote", "whitespace-delimited-unmatched-western-quote");
        }
        if (leftRole != null && (rightRole == null || rightRole == leftRole)) {
            return new Resolution(leftRole, "UnmatchedQuoteSurroundingScriptContext", "unmatched-quote-surrounding-script");
        }
        if (rightRole != null && leftRole == null) {
            return new Resolution(rightRole, "UnmatchedQuoteSurroundingScriptContext", "unmatched-quote-surrounding-script");
        }
        final reason:String = leftRole != null && rightRole != null ? "conflicting-unmatched-quote-context" : "no-unmatched-quote-context";
        return paragraphLanguageResolution(reason);
    }

    private function nearestStrongScriptRole(startIndex:Int, direction:Int):Null<FontRole> {
        var index:Int = startIndex;
        while (index >= 0 && index < text.length) {
            if (direction < 0) {
                final pair:QuotePair = pairByClose.get(index);
                if (pair != null) { index = pair.openIndex - 1; continue; }
            } else {
                final pair:QuotePair = pairByOpen.get(index);
                if (pair != null) { index = pair.closeIndex + 1; continue; }
            }
            final scalarStart:Int = direction < 0 && isLowSurrogate(text.charCodeAt(index)) && index > 0 && isHighSurrogate(text.charCodeAt(index - 1)) ? index - 1 : index;
            final length:Int = codePointLengthAt(scalarStart, text.length);
            final role:Null<FontRole> = strongScriptRole(scalarStart, length);
            if (role != null) return role;
            index = direction < 0 ? scalarStart - 1 : scalarStart + length;
        }
        return null;
    }

    private function paragraphLanguageResolution(reason:String):Resolution {
        final role:FontRole = UnicodeEastAsianSpacing.isChineseLanguageContext(context.locale) ? FontRole.CjkPunctuation : FontRole.LatinText;
        return new Resolution(role, "ParagraphLanguageQuoteContext", reason + "; paragraph-language=" + context.locale);
    }

    private function findParent(pair:QuotePair):Null<QuotePair> {
        var result:Null<QuotePair> = null;
        for (candidate in pairs) {
            if (candidate != pair && candidate.openIndex < pair.openIndex && candidate.closeIndex > pair.closeIndex
                && (result == null || candidate.closeIndex - candidate.openIndex < result.closeIndex - result.openIndex)) result = candidate;
        }
        return result;
    }

    private function strongScriptRole(index:Int, length:Int):Null<FontRole> {
        return switch (UnicodeScriptEvidenceClassifier.classify(codePointAt(index, index + length))) {
            case UnicodeScriptEvidence.EastAsian: FontRole.CjkPunctuation;
            case UnicodeScriptEvidence.Other: FontRole.LatinText;
            case UnicodeScriptEvidence.Neutral: null;
        };
    }

    private function isNonCjkInWordApostrophe(index:Int):Bool {
        final before:Null<Int> = codePointBefore(index);
        final after:Null<Int> = codePointAtOrNull(index + 1);
        return before != null && after != null && isNonCjkWordCharacter(before) && isNonCjkWordCharacter(after)
            && (isNonCjkNonNumericWordCharacter(before) || isNonCjkNonNumericWordCharacter(after));
    }

    private function isDigitBoundClosingQuote(index:Int):Bool {
        final before:Null<Int> = codePointBefore(index);
        return (text.charCodeAt(index) == 0x2019 || text.charCodeAt(index) == 0x201D) && before != null && UnicodeNumber.contains(before);
    }

    private function isNonCjkWordInternalQuotePair(pair:QuotePair):Bool {
        final before:Null<Int> = codePointBefore(pair.openIndex);
        final after:Null<Int> = codePointAtOrNull(pair.closeIndex + 1);
        if (before == null || after == null || !isNonCjkNonNumericWordCharacter(before) || !isNonCjkNonNumericWordCharacter(after)) return false;
        var index:Int = pair.openIndex + 1;
        while (index < pair.closeIndex) {
            final codePoint:Null<Int> = codePointAtOrNull(index);
            if (codePoint == null || !isNonCjkWordCharacter(codePoint)) return false;
            index += codePoint > 0xFFFF ? 2 : 1;
        }
        return true;
    }

    private function isNonCjkWordCharacter(codePoint:Int):Bool return UnicodeWordCharacter.contains(codePoint) && UnicodeScriptEvidenceClassifier.classify(codePoint) != UnicodeScriptEvidence.EastAsian;
    private function isNonCjkNonNumericWordCharacter(codePoint:Int):Bool return isNonCjkWordCharacter(codePoint) && !UnicodeNumber.contains(codePoint) && !isFullwidth(codePoint);
    private function isFullwidth(codePoint:Int):Bool return codePoint == 0x3000 || (codePoint >= 0xFF01 && codePoint <= 0xFF60) || (codePoint >= 0xFFE0 && codePoint <= 0xFFE6);
    private static function isAmbiguousCurlyQuote(codePoint:Int):Bool return codePoint == 0x2018 || codePoint == 0x2019 || codePoint == 0x201C || codePoint == 0x201D;
    private static function isAsciiSpaceOrTab(codePoint:Null<Int>):Bool return codePoint != null && (codePoint == 0x20 || codePoint == 0x09);
    private static function isHighSurrogate(codePoint:Int):Bool return codePoint >= 0xD800 && codePoint <= 0xDBFF;
    private static function isLowSurrogate(codePoint:Int):Bool return codePoint >= 0xDC00 && codePoint <= 0xDFFF;
    private function charAt(index:Int):Null<Int> return index >= 0 && index < text.length ? text.charCodeAt(index) : null;
    private function codePointAt(index:Int, end:Int):Int { final high:Int = text.charCodeAt(index); if (!isHighSurrogate(high) || index + 1 >= end) return high; final low:Int = text.charCodeAt(index + 1); return isLowSurrogate(low) ? 0x10000 + ((high - 0xD800) << 10) + low - 0xDC00 : high; }
    private function codePointLengthAt(index:Int, end:Int):Int return isHighSurrogate(text.charCodeAt(index)) && index + 1 < end && isLowSurrogate(text.charCodeAt(index + 1)) ? 2 : 1;
    private function codePointBefore(index:Int):Null<Int> { if (index <= 0) return null; final low:Int = text.charCodeAt(index - 1); if (!isLowSurrogate(low) || index < 2) return low; final high:Int = text.charCodeAt(index - 2); return isHighSurrogate(high) ? 0x10000 + ((high - 0xD800) << 10) + low - 0xDC00 : low; }
    private function codePointAtOrNull(index:Int):Null<Int> return index < 0 || index >= text.length ? null : codePointAt(index, text.length);
}

private class ScriptEvidenceState {
    public var hasCjk:Bool = false;
    public var hasWestern:Bool = false;
    public var isMixed(get, never):Bool;
    private function get_isMixed():Bool return hasCjk && hasWestern;
    public function new() {}
    public function addRange(pairByOpen:Map<Int, QuotePair>, start:Int, end:Int, text:String):Void {
        var index:Int = start;
        while (index < end) {
            final nested:QuotePair = pairByOpen.get(index);
            if (nested != null && nested.closeIndex < end) { index = nested.closeIndex + 1; continue; }
            final length:Int = ContextualQuoteRoleResolverCodePoint.lengthAt(text, index, end);
            switch (UnicodeScriptEvidenceClassifier.classify(ContextualQuoteRoleResolverCodePoint.at(text, index, end))) {
                case UnicodeScriptEvidence.EastAsian: hasCjk = true;
                case UnicodeScriptEvidence.Other: hasWestern = true;
                case UnicodeScriptEvidence.Neutral:
            }
            index += length;
        }
    }
    public function unambiguousRole():Null<FontRole> return hasCjk && !hasWestern ? FontRole.CjkPunctuation : hasWestern && !hasCjk ? FontRole.LatinText : null;
}

private class Resolution {
    public final role:FontRole; public final source:String; public final reason:String;
    public function new(role:FontRole, source:String, reason:String) { this.role = role; this.source = source; this.reason = reason; }
}

private class ContextualQuoteRoleResolverCodePoint {
    public static function at(text:String, index:Int, end:Int):Int { final high:Int = text.charCodeAt(index); if (high < 0xD800 || high > 0xDBFF || index + 1 >= end) return high; final low:Int = text.charCodeAt(index + 1); return low >= 0xDC00 && low <= 0xDFFF ? 0x10000 + ((high - 0xD800) << 10) + low - 0xDC00 : high; }
    public static function lengthAt(text:String, index:Int, end:Int):Int return text.charCodeAt(index) >= 0xD800 && text.charCodeAt(index) <= 0xDBFF && index + 1 < end && text.charCodeAt(index + 1) >= 0xDC00 && text.charCodeAt(index + 1) <= 0xDFFF ? 2 : 1;
}
