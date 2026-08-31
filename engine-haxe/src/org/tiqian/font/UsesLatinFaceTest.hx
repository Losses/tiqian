package org.tiqian.font;
import org.tiqian.font.FontRole.*;
import org.tiqian.test.trace.TestTraceRecorder;
import org.tiqian.test.trace.TracedAssertions;
class UsesLatinFaceTest {
 @:test public static function onlyLatinTextUsesLatinFace():Void { new TestTraceRecorder("UsesLatinFaceTest").section("onlyLatinTextUsesLatinFace"); TracedAssertions.assertTrue(FontRoleFns.usesLatinFace(LatinText)); var a=[CjkText,CjkPunctuation,Symbol,Emoji,Unknown];var i=0;while(i<a.length){TracedAssertions.assertFalse(FontRoleFns.usesLatinFace(a[i]));i++;} }
 @:test public static function nameOverloadAgreesWithEnum():Void { new TestTraceRecorder("UsesLatinFaceTest").section("nameOverloadAgreesWithEnum"); var names=["CjkText","CjkPunctuation","LatinText","Symbol","Emoji","Unknown"];var i=0;while(i<names.length){var x=names[i];TracedAssertions.assertEquals(x=="LatinText"?1:0,FontRoleFns.fontRoleNameUsesLatinFace(x)?1:0);i++;} TracedAssertions.assertFalse(FontRoleFns.fontRoleNameUsesLatinFace(null));TracedAssertions.assertFalse(FontRoleFns.fontRoleNameUsesLatinFace("NotARole")); }
}
