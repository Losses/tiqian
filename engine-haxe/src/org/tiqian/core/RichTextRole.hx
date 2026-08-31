package org.tiqian.core;

import org.tiqian.core.RichTextRole.Background;
import org.tiqian.core.RichTextRole.Underline;
import org.tiqian.core.RichTextRole.LineThrough;
import org.tiqian.core.RichTextRole.Link;
import org.tiqian.core.RichTextRole.TechnicalInline;
import org.tiqian.core.RichTextRole.InlineCode;

@:sealed
interface RichTextRole {}

class Background implements RichTextRole { public static final instance:Background = new Background(); private function new() {} }
class Underline implements RichTextRole { public static final instance:Underline = new Underline(); private function new() {} }
class LineThrough implements RichTextRole { public static final instance:LineThrough = new LineThrough(); private function new() {} }
@:dataClass class Link implements RichTextRole { public final target:String; public function new(target:String) this.target = target; }
class TechnicalInline implements RichTextRole { public static final instance:TechnicalInline = new TechnicalInline(); private function new() {} }
class InlineCode implements RichTextRole { public static final instance:InlineCode = new InlineCode(); private function new() {} }
