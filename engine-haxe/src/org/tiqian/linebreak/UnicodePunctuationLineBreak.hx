package org.tiqian.linebreak;

import org.tiqian.core.TiqianIllegalArgumentException;
import org.tiqian.core.TextRangeError;

enum UnicodePunctuationLineBreakClass {
    BreakAfter; BreakBoth; ClosePunctuation; CloseParenthesis; Exclamation; HyphenHH; Hyphen;
    Inseparable; InfixNumericSeparator; Nonstarter; OpenPunctuation; Quotation;
    SymbolsAllowingBreakAfter; Other;
}

class UnicodePunctuationLineBreak {
    public static inline final DATA_REVISION:String = "17.0.0";
    public static inline final DATA_SOURCE:String = "https://www.unicode.org/Public/17.0.0/ucd/LineBreak.txt";
    public static inline final DATA_SHA256:String = "e6a18fa91f8f6a6f8e534b1d3f128c21ada45bfe152eb6b1bcc5e15fd8ac92e6";
    public static function classOf(codePoint:Int):UnicodePunctuationLineBreakClass {
        if (codePoint < 0 || codePoint > 0x10FFFF)
            throw new TiqianIllegalArgumentException(Message("Not a Unicode scalar value: " + codePoint));
        if (codePoint >= 0xD800 && codePoint <= 0xDFFF)
            throw new TiqianIllegalArgumentException(Message("Surrogate is not a Unicode scalar value: " + codePoint));
        return switch (UnicodePunctuationLineBreakData.lookup(codePoint)) {
            case 0: BreakAfter; case 1: BreakBoth; case 2: ClosePunctuation;
            case 3: CloseParenthesis; case 4: Exclamation; case 5: HyphenHH;
            case 6: Hyphen; case 7: Inseparable; case 8: InfixNumericSeparator;
            case 9: Nonstarter; case 10: OpenPunctuation; case 11: Quotation;
            case 12: SymbolsAllowingBreakAfter; default: Other;
        };
    }
}
