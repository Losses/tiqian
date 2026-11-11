// Frozen projection of platforms/web/client/core/schema/assembly-record.schema.json
// revision 1. The generator tools/schema/generate_ts.py was deleted in commit
// 5d366296 together with tools/schema/, so this file cannot be regenerated.
// Treat it as a frozen artifact and edit it by hand only in the same commit
// that changes the schema.
//
// Manual sync criterion: LAYOUT_REQUEST_FIELDS must equal
// surfaces.workerJson.fields in the same order; ASSEMBLY_RECORD_REVISION must
// equal the schema revision; the AssemblyRecordRequest typedef must match the
// schema scalar and table types. A wire-shape change bumps the schema revision
// and updates all three by hand. See platforms/web/client/core/schema/FROZEN.md.

export const LAYOUT_REQUEST_FIELDS = Object.freeze([
  "text",
  "maxWidthPx",
  "fontFamilies",
  "fontSizePx",
  "lineHeightPx",
  "locale",
  "fontWeight",
  "italic",
  "firstLineIndentIc",
  "sourceBoundaries",
  "textSpans",
  "inlineBoxes",
  "lineBreakSpans",
  "inlineObjects",
] as const);

export const ASSEMBLY_RECORD_REVISION = 1;

/**
 * @typedef {Object} AssemblyRecordRequest
 * @property {string} text
 * @property {number} maxWidthPx
 * @property {string[]} fontFamilies
 * @property {number} fontSizePx
 * @property {number} lineHeightPx
 * @property {string} locale
 * @property {number} fontWeight
 * @property {boolean} italic
 * @property {number} firstLineIndentIc
 * @property {number[]} sourceBoundaries
 * @property {Array.<{start: number, end: number, fontSizePx: number, fontWeight: number, italic: boolean, baselineShift: number, fontFamilies: string[]}>} textSpans
 * @property {Array.<{start: number, end: number, inlineStart: number, inlineEnd: number, outerSpacing: string}>} inlineBoxes
 * @property {Array.<{start: number, end: number, policy: string}>} lineBreakSpans
 * @property {Array.<{start: number, end: number, advance: number, ascent: number, descent: number}>} inlineObjects
 */
