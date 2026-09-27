package org.tiqian.protocol;

enum DecodeResult {
    TOk(data:TableData);
    TErr(issue:String);
}

/** Linear string interner: replay strings keep refs 0..n-1, new texts append. */
