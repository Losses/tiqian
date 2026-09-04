package org.tiqian.layout;

import org.tiqian.core.*;
import org.tiqian.clreq.*;
import org.tiqian.shaping.TextShaper.ITextShaper;
import org.tiqian.shaping.TextShaper.ShapingInput;
import org.tiqian.shaping.TextShaper.ShapingResult;
import org.tiqian.shaping.TextShaper.ExplainableStubTextShaper;
import org.tiqian.layout.ParagraphLayoutEngine.ExplainableStubParagraphLayoutEngine;
import org.tiqian.layout.WidthIndependentAnnotationCache.WidthIndependentAnnotationCacheFns;
import org.tiqian.layout.WidthIndependentAnnotationCache.WidthIndependentAnnotationKey;
import org.tiqian.test.trace.TestTrace;
import org.tiqian.test.trace.TestTraceRender;
import org.tiqian.test.trace.TracedAssertions;

class CountingTextShaper implements ITextShaper {
    public var shapeCallCount:Int = 0;
    final delegate:ITextShaper;
    public function new(?delegate:ITextShaper) {
        this.delegate = delegate != null ? delegate : new ExplainableStubTextShaper();
    }
    public function shape(input:ShapingInput):ShapingResult {
        shapeCallCount += 1;
        return delegate.shape(input);
    }
}

class WidthIndependentAnnotationCacheTestSupport {
    public static function layoutWithCache(cache:WidthIndependentAnnotationCache, text:String, maxWidth:Float, ?firstLineIndent:Ic):LayoutResult {
        final indent = firstLineIndent != null ? firstLineIndent : Ic.Zero;
        final engine = new ExplainableStubParagraphLayoutEngine(null, null, null, null, null, null, null, null, null, null, null, null, cache);
        return engine.layout(new LayoutInput(
            new TiqianTextContent(text),
            null,
            new ParagraphStyle(null, null, null, indent),
            new LayoutConstraints(maxWidth)
        ));
    }

    public static function annotationKey(input:LayoutInput):WidthIndependentAnnotationKey {
        return WidthIndependentAnnotationCacheFns.toWidthIndependentAnnotationKey(input);
    }

    public static function copyInput(input:LayoutInput, ?content:TiqianTextContent, ?textStyle:TextStyle, ?paragraphStyle:ParagraphStyle, ?constraints:LayoutConstraints, ?decorations:Array<DecorationSpan>, ?rubySpans:Array<RubySpan>, ?inlineBoxes:Array<InlineBoxSpan>):LayoutInput {
        return new LayoutInput(
            content != null ? content : input.content,
            textStyle != null ? textStyle : input.textStyle,
            paragraphStyle != null ? paragraphStyle : input.paragraphStyle,
            constraints != null ? constraints : input.constraints,
            input.profileId,
            decorations != null ? decorations : cast(input.decorations, Array<DecorationSpan>),
            rubySpans != null ? rubySpans : cast(input.rubySpans, Array<RubySpan>),
            inlineBoxes != null ? inlineBoxes : cast(input.inlineBoxes, Array<InlineBoxSpan>),
            input.inlineObjects
        );
    }

    public static function assertEqualsTextRange(expected:TextRange, actual:TextRange, ?message:String):Void {
        final e = TestTraceRender.canonicalNumbers(Std.string(expected));
        final a = TestTraceRender.canonicalNumbers(Std.string(actual));
        final recorder = TestTrace.currentRecorder();
        if (recorder != null) {
            var line = "eq expected=" + e + " actual=" + a;
            if (message != null) line += " msg='" + TestTraceRender.escapeOperand(message) + "'";
            recorder.record(line);
        }
        if (expected.start != actual.start || expected.end != actual.end) TracedAssertions.fail(message == null ? "TextRange mismatch" : message);
    }
}
