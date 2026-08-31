package org.tiqian.font;

class PreferCjkForAmbiguousPunctuationResolver implements FallbackResolver {
 final cjkFontKey:String;final latinFontKey:String;final symbolFontKey:String;
 public function new(?cjkFontKey:Null<String>,?latinFontKey:Null<String>,?symbolFontKey:Null<String>){this.cjkFontKey=cjkFontKey==null?"cjk-primary":cjkFontKey;this.latinFontKey=latinFontKey==null?"latin-primary":latinFontKey;this.symbolFontKey=symbolFontKey==null?"symbol-fallback":symbolFontKey;}
 public function resolve(text:String,range:org.tiqian.core.TextRange,request:FontRequest):FontDecision { final c = switch(request.role){case CjkText|CjkPunctuation:new FontCandidate(cjkFontKey,request.preferredFamilies.length==0?cjkFontKey:request.preferredFamilies[0],request.role);case LatinText:new FontCandidate(latinFontKey,latinFontKey,request.role);case Symbol|Emoji|Unknown:new FontCandidate(symbolFontKey,symbolFontKey,request.role);}; return new FontDecision(range,c,request.role,"PreferCjkForAmbiguousPunctuationResolver:"+request.role); }
}
