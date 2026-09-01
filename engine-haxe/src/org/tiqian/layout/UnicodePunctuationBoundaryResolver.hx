package org.tiqian.layout;

import org.tiqian.core.InlineAttachment;
import org.tiqian.core.IntRange;

/**
 * Minimal slice of Kotlin UnicodePunctuationBoundaryResolver.kt (lines 60-100):
 * the attached-inline virtual boundary model and resolver consumed by
 * PunctuationGeometryStage.applyAutoSpacePolicy. The full resolver arrives with
 * the punctuation-boundary lane; the two members below match that file verbatim
 * so the merge keeps a single definition.
 */
@:dataClass class AttachedInlineVirtualBoundary { public final previousClusterIndex:Int; public final attachedClusterRange:IntRange; public final nextClusterIndex:Null<Int>; public function new(previousClusterIndex:Int,attachedClusterRange:IntRange,nextClusterIndex:Null<Int>){this.previousClusterIndex=previousClusterIndex;this.attachedClusterRange=attachedClusterRange;this.nextClusterIndex=nextClusterIndex;} }

class UnicodePunctuationBoundaryResolver {
 public static function resolveAttachedInlineVirtualBoundaries(a:Array<InlineAttachment>):Array<AttachedInlineVirtualBoundary>{final r:Array<AttachedInlineVirtualBoundary>=[];var i=0;while(i<a.length){if(a[i]!=InlineAttachment.Previous){i++;continue;}final s=i;var e=s;while(e+1<a.length&&a[e+1]==InlineAttachment.Previous)e++;if(s>0)r.push(new AttachedInlineVirtualBoundary(s-1,new IntRange(s,e),e+1<a.length?e+1:null));i=e+1;}return r;}
}
