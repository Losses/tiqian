// Type surface of the public entry (src/facade.mjs). The eight Kotlin-runtime
// exports reuse the generated declarations; the line-break and clreq exports
// are declared here beside their JavaScript implementations. The export order
// matches src/facade.mjs and the compatibility contract in package.test.mts.

import {
  fontMetricsResolve,
  fontFallbackResolve,
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
} from "./Tiqian-tiqian-ffi-js.d.mts";

declare function bopomofoParse(reading: string): string;
declare function numberSymbolCohesionUnbreakableRanges(text: string): string;

declare function liangHyphenate(
  word: string,
  patternsJson: string,
  exceptionsJson: string,
  leftMin?: number,
  rightMin?: number,
): string;
declare function unicodePunctuationLineBreakClassOf(codePoint: number): string;

export {
  bopomofoParse,
  numberSymbolCohesionUnbreakableRanges,
  fontMetricsResolve,
  fontFallbackResolve,
  liangHyphenate,
  unicodePunctuationLineBreakClassOf,
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
};
