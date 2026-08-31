package org.tiqian.font;
class CjkFontRoleClassifier implements FontRoleClassifier {
 public function new() {}
 public function classify(text:String,range:org.tiqian.core.TextRange,?context:Null<FontRoleContext>):FontRole { final c=text.charCodeAt(range.start); if((c>=0x3105&&c<=0x312F)||(c>=0x31A0&&c<=0x31BF)||(c>=0x3400&&c<=0x4DBF)||(c>=0x4E00&&c<=0x9FFF)||(c>=0xF900&&c<=0xFAFF)||(c>=0x20000&&c<=0x3134F))return CjkText; if(c==0x2018||c==0x2019||c==0x201C||c==0x201D){var l=range.start>0?text.charCodeAt(range.start-1):null;var r=range.end<text.length?text.charCodeAt(range.end):null;return isLatin(l)&&isLatin(r)?LatinText:CjkPunctuation;} if(c>=0x20&&c<=0x7E||(c>=0xC0&&c<=0x24F))return LatinText; if(c>=0x3000&&c<=0x303F||c==0x2014||c==0x2026||c==0x2027||c==0x22EF||c==0x30FB||c==0x2E3A||c==0x00B7||c==0x2022||c>=0xFF01&&c<=0xFF5E)return CjkPunctuation; if(UnicodeEmojiPresentationData.contains(c))return Emoji; if(c==0x2764)return Symbol; return Unknown; }
 static function isLatin(c:Null<Int>):Bool return c!=null&&(c>=0x20&&c<=0x7E||c>=0xC0&&c<=0x24F);
}
