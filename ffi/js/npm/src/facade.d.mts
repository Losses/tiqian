// Type surface of the public entry (src/facade.mjs). The ten Kotlin-runtime
// exports reuse the generated declarations; the two line-break exports are
// declared here beside their JavaScript implementation. The export order
// matches src/facade.mjs and the compatibility contract in package.test.mts.

import {
  bopomofoParse,
  numberSymbolCohesionUnbreakableRanges,
  fontMetricsResolve,
  fontFallbackResolve,
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
} from "./Tiqian-tiqian-ffi-js.d.mts";

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
