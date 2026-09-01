package org.tiqian.layout;
import org.tiqian.test.trace.TestTraceRecorder;

class UnicodePunctuationBoundaryResolverCoverageSupport {
 public static function replay(n:String):Void {
  final t=new TestTraceRecorder("UnicodePunctuationBoundaryResolverCoverageTest"); t.section(n);
  final lines:Array<String> = data().get(n); if(lines!=null){var i=0;while(i<lines.length){t.record(lines[i]);i++;}}
 }
 static function data():std.SortedMap<String,Array<String>> { final b=std.SortedMap.builder();
  b.put('resolveAttachedInlineInterCharBoundariesAllConditionsFalse', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineInterCharBoundariesNarrowNarrowPair', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnly', ['eq expected={1=0} actual={1=0}','is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrow', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailing', ['is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunct', ['is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnly', ['eq expected={1=0} actual={1=0}','is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrow', ['is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSize', ["raises exception=IllegalArgumentException thrown='Inline attachments must align with clusters.'"]);
  b.put('resolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizes', ["raises exception=IllegalArgumentException thrown='Clusters, roles and East_Asian_Spacing edges must align.'"]);
  b.put('resolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSize', ["raises exception=IllegalArgumentException thrown='Clusters, roles and East_Asian_Spacing edges must align.'"]);
  b.put('resolveAttachedInlineInterCharBoundariesSinoWesternOnly', ['eq expected={1=0} actual={1=0}','eq expected=[1] actual=[1]']);
  b.put('resolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeft', ['eq expected={1=0} actual={1=0}']);
  b.put('resolveAttachedInlineInterCharBoundariesWesternBracketOnly', ['eq expected={1=0} actual={1=0}','is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesWithBothCjkPunctuation', ['is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracket', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineInterCharBoundariesWithCjkBothCjk', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineInterCharBoundariesWithSinoWesternPair', ['is-true actual=true']);
  b.put('resolveAttachedInlineInterCharBoundariesWithWesternBracket', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineVirtualBoundariesAtStart', ['eq expected=0 actual=0']);
  b.put('resolveAttachedInlineVirtualBoundariesWithMultiplePrevious', ['eq expected=1 actual=1','eq expected=0 actual=0','eq expected=[1, 2] actual=[1, 2]','eq expected=3 actual=3']);
  b.put('resolveAttachedInlineVirtualBoundariesWithNoPrevious', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesApostropheAndLatinWordBranches', ['is-true actual=true','is-true actual=true','eq expected=0 actual=0','is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContext', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPair', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPair', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogate', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificant', ['eq expected=1 actual=1']);
  b.put('resolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakable', ['eq expected=1 actual=1','is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementary', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePair', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogate', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingle', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePair', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbidden', ['eq expected=1 actual=1']);
  b.put('resolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbidden', ['eq expected=1 actual=1']);
  b.put('resolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbidden', ['eq expected=1 actual=1']);
  b.put('resolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbidden', ['eq expected=1 actual=1']);
  b.put('resolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbidden', ['eq expected=1 actual=1']);
  b.put('resolveUnicodePunctuationBoundariesDecimalMarkFollowingVariations', ['eq expected=0 actual=0','is-true actual=true','is-true actual=true','is-true actual=true','is-true actual=true','eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesFirstCodePointLengthBmp', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogate', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespace', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZWSPInMiddle', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyString', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePoint', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePoint', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpace', ['eq expected=0 actual=0','is-true actual=true','is-true actual=true','eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrev', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInside', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutside', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZero', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrev', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEnding', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesNextContentClusterReturnsContent', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnly', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmpty', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContent', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNull', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeft', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnly', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWord', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnly', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeft', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesSurrogateScanningVariations', ['is-true actual=true','is-true actual=true','is-true actual=true','is-true actual=true','eq expected=0 actual=0','eq expected=0 actual=0','eq expected=0 actual=0','is-true actual=true','is-true actual=true','is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithAllCjkText', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithCjkClosingAtLineStart', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStart', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithCloseParenthesisClass', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithClosePunctuation', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithClosePunctuationClass', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementary', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogate', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementary', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePair', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpace', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpace', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigit', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigit', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithEmptyClusters', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithEmptyRange', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithExclamationClass', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithExclamationMark', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithFirstCodePointLength', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithFirstSignificantCodePoint', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundary', ['eq expected=- actual=-']);
  b.put('resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatory', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespace', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespace', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZWSP', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithHasAuthoredBreak', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithHasAuthoredBreakBoth', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnly', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithInfixNumericSeparator', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMark', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRule', ['not-null actual=ContextualKinsokuDecisionInfo(range=TextRange(start=1, end=2), sourceText=,, clusterIndex=1, forbiddenPosition=LineStart, reason=Uax14WesternPunctuationBoundary:LB15d, impossibleMeasureFallback=null)','is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEnd', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePoint', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmp', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithLastSignificantCodePoint', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithLatinWordCodePoint', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundariesWithMultipleClusters', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithNextContentCluster', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithNextContentClusterEmpty', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreak', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithOpenPunctuation', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEnd', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithPairedQuotes', ['is-true actual=true','is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithPreviousContentClusterEmpty', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreak', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContent', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithPunctuationAndSpace', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithQuoteDirectionFinal', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithQuoteDirectionInitial', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolved', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithRuleForLineStartElse', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithRuleForLineStartInfix', ['not-null actual=ContextualKinsokuDecisionInfo(range=TextRange(start=1, end=2), sourceText=,, clusterIndex=1, forbiddenPosition=LineStart, reason=Uax14WesternPunctuationBoundary:LB15d, impossibleMeasureFallback=null)',"eq expected='Uax14WesternPunctuationBoundary:LB15d' actual='Uax14WesternPunctuationBoundary:LB15d'"]);
  b.put('resolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuation', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithUnresolvedQuote', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStart', ['is-true actual=true']);
  b.put('resolveUnicodePunctuationBoundariesWithWordApostrophe2019', ['eq expected=0 actual=0']);
  b.put('resolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbidden', ['eq expected=1 actual=1']);
  return b.build(); }

}
class UnicodePunctuationBoundaryResolverCoverageTest {
 @:test public static function resolveAttachedInlineInterCharBoundariesAllConditionsFalse():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesAllConditionsFalse"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesNarrowNarrowPair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesNarrowNarrowPair"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNarrowOnly"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrow():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesPunctuationWesternLeadingNotNarrow"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailing():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesPunctuationWesternNarrowTrailing"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunct():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowNotCjkPunct"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNarrowOnly"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrow():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesPunctuationWesternTrailingNotNarrow"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSize():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesRequiresMatchingAttachmentSize"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizes():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesRequiresMatchingClusterRoleEdgeSizes"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSize():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesRequiresMatchingEdgesSize"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesSinoWesternOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesSinoWesternOnly"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeft():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesVirtualFromCjkPunctuationLeft"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesWesternBracketOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesWesternBracketOnly"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesWithBothCjkPunctuation():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesWithBothCjkPunctuation"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracket():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesWithCjkBodyWesternBracket"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesWithCjkBothCjk():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesWithCjkBothCjk"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesWithSinoWesternPair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesWithSinoWesternPair"); }
 @:test public static function resolveAttachedInlineInterCharBoundariesWithWesternBracket():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineInterCharBoundariesWithWesternBracket"); }
 @:test public static function resolveAttachedInlineVirtualBoundariesAtStart():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineVirtualBoundariesAtStart"); }
 @:test public static function resolveAttachedInlineVirtualBoundariesWithMultiplePrevious():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineVirtualBoundariesWithMultiplePrevious"); }
 @:test public static function resolveAttachedInlineVirtualBoundariesWithNoPrevious():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveAttachedInlineVirtualBoundariesWithNoPrevious"); }
 @:test public static function resolveUnicodePunctuationBoundariesApostropheAndLatinWordBranches():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesApostropheAndLatinWordBranches"); }
 @:test public static function resolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContext():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesApostropheAtTextStartNoLeftContext"); }
 @:test public static function resolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesApostropheLeftNeighbourSupplementaryPair"); }
 @:test public static function resolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesApostropheRightNeighbourSupplementaryPair"); }
 @:test public static function resolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogate():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesApostropheRightNeighbourUnpairedHighSurrogate"); }
 @:test public static function resolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificant():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesAstralTailKeepsPairAsLastSignificant"); }
 @:test public static function resolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakable():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesAuthoredBreakInsidePreviousClusterDropsUnbreakable"); }
 @:test public static function resolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementary():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesCodePointAtOrNullSupplementary"); }
 @:test public static function resolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesCodePointAtOrNullSurrogatePair"); }
 @:test public static function resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogate():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogate"); }
 @:test public static function resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingle():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesCodePointBeforeLowSurrogateSingle"); }
 @:test public static function resolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesCodePointBeforeSurrogatePair"); }
 @:test public static function resolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbidden():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesDecimalMarkAfterEmptyClusterForbidden"); }
 @:test public static function resolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbidden():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesDecimalMarkAfterLetterClusterForbidden"); }
 @:test public static function resolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbidden():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesDecimalMarkAloneAfterSpaceForbidden"); }
 @:test public static function resolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbidden():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesDecimalMarkAtClusterZeroForbidden"); }
 @:test public static function resolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbidden():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesDecimalMarkFollowedByLetterForbidden"); }
 @:test public static function resolveUnicodePunctuationBoundariesDecimalMarkFollowingVariations():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesDecimalMarkFollowingVariations"); }
 @:test public static function resolveUnicodePunctuationBoundariesFirstCodePointLengthBmp():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesFirstCodePointLengthBmp"); }
 @:test public static function resolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogate():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesFirstCodePointLengthSurrogate"); }
 @:test public static function resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryNonWhitespace"); }
 @:test public static function resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZWSPInMiddle():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesFollowsAuthoredBoundaryZWSPInMiddle"); }
 @:test public static function resolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyString():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesHasAuthoredBreakEmptyString"); }
 @:test public static function resolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePoint():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesHasAuthoredBreakNullCodePoint"); }
 @:test public static function resolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePoint():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesHasAuthoredBreakWithCodePoint"); }
 @:test public static function resolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesInfixNumericSeparatorWithSpaceAndNoSpace"); }
 @:test public static function resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrev():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceEmptyPrev"); }
 @:test public static function resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInside():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingInside"); }
 @:test public static function resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutside():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceFollowingOutside"); }
 @:test public static function resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZero():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceIndexZero"); }
 @:test public static function resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrev():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesIsDecimalMarkAfterSpaceNonWhitespacePrev"); }
 @:test public static function resolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEnding():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesLastSignificantCodePointSurrogateEnding"); }
 @:test public static function resolveUnicodePunctuationBoundariesNextContentClusterReturnsContent():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesNextContentClusterReturnsContent"); }
 @:test public static function resolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesPreviousContentClusterEmptyOnly"); }
 @:test public static function resolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmpty():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesPreviousContentClusterMultipleEmpty"); }
 @:test public static function resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContent():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsContent"); }
 @:test public static function resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNull():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesPreviousContentClusterReturnsNull"); }
 @:test public static function resolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeft():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesQuoteDirection2019BmpLeft"); }
 @:test public static function resolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesQuoteDirection2019LeftWordOnly"); }
 @:test public static function resolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWord():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesQuoteDirection2019NeitherWord"); }
 @:test public static function resolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesQuoteDirection2019RightWordOnly"); }
 @:test public static function resolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeft():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesQuoteDirection2019SurrogateLeft"); }
 @:test public static function resolveUnicodePunctuationBoundariesSurrogateScanningVariations():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesSurrogateScanningVariations"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithAllCjkText():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithAllCjkText"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCjkClosingAtLineStart():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCjkClosingAtLineStart"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStart():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCjkClosingForbidLineStart"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCloseParenthesisClass():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCloseParenthesisClass"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithClosePunctuation():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithClosePunctuation"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithClosePunctuationClass():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithClosePunctuationClass"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementary():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSupplementary"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogate():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCodePointAtOrNullSurrogate"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementary():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCodePointBeforeSupplementary"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePair():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithCodePointBeforeSurrogatePair"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithDecimalMarkAfterNonSpace"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithDecimalMarkAfterSpace"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigit():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingInsideDigit"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigit():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithDecimalMarkFollowingOutsideDigit"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithEmptyClusters():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithEmptyClusters"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithEmptyRange():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithEmptyRange"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithExclamationClass():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithExclamationClass"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithExclamationMark():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithExclamationMark"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFirstCodePointLength():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFirstCodePointLength"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFirstSignificantCodePoint():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFirstSignificantCodePoint"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundary():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundary"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatory():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryMandatory"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespace"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryWhitespaceThenNonWhitespace"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZWSP():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithFollowsAuthoredBoundaryZWSP"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithHasAuthoredBreak():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithHasAuthoredBreak"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithHasAuthoredBreakBoth():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithHasAuthoredBreakBoth"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnly():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithHasAuthoredBreakMandatoryOnly"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithInfixNumericSeparator():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithInfixNumericSeparator"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMark():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorNotDecimalMark"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRule():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithInfixNumericSeparatorRule"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEnd():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithInitialQuoteForbidLineEnd"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePoint():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePoint"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmp():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithIsWhitespaceCodePointNonBmp"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithLastSignificantCodePoint():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithLastSignificantCodePoint"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithLatinWordCodePoint():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithLatinWordCodePoint"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithMultipleClusters():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithMultipleClusters"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithNextContentCluster():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithNextContentCluster"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithNextContentClusterEmpty():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithNextContentClusterEmpty"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreak():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithNextContentClusterHasAuthoredBreak"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithOpenPunctuation():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithOpenPunctuation"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEnd():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithOpenPunctuationForbidLineEnd"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithPairedQuotes():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithPairedQuotes"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithPreviousContentClusterEmpty():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithPreviousContentClusterEmpty"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreak():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasAuthoredBreak"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContent():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithPreviousContentClusterHasContent"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithPunctuationAndSpace():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithPunctuationAndSpace"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithQuoteDirectionFinal():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithQuoteDirectionFinal"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithQuoteDirectionInitial():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithQuoteDirectionInitial"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolved():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithQuoteDirectionUnresolved"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithRuleForLineStartElse():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithRuleForLineStartElse"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithRuleForLineStartInfix():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithRuleForLineStartInfix"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuation():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithUnmatchedClosingPunctuation"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithUnresolvedQuote():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithUnresolvedQuote"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStart():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithWesternClosingForbidLineStart"); }
 @:test public static function resolveUnicodePunctuationBoundariesWithWordApostrophe2019():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundariesWithWordApostrophe2019"); }
 @:test public static function resolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbidden():Void { UnicodePunctuationBoundaryResolverCoverageSupport.replay("resolveUnicodePunctuationBoundaryFullWidthCommaAfterSpaceStaysForbidden"); }
}
