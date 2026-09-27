// Type surface of the public entry (src/facade.mjs). The eight Kotlin-runtime
// exports reuse the generated declarations; the two line-break and the two
// font exports are declared here beside their JavaScript implementation. The
// export order matches src/facade.mjs and the compatibility contract in
// package.test.mts.

import {
  bopomofoParse,
  numberSymbolCohesionUnbreakableRanges,
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
} from "./Tiqian-tiqian-ffi-js.d.mts";

declare function fontMetricsResolve(requestJson: string): string;
declare function fontFallbackResolve(
  text: string,
  start: number,
  end: number,
  requestJson: string,
): string;
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
