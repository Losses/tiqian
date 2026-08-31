package org.tiqian.core;

enum RichTextRole {
    Background;
    Underline;
    LineThrough;
    Link(target:String);
    TechnicalInline;
    InlineCode;
}
