// Type surface of the public entry (src/facade.mjs). The eight Kotlin-runtime
// exports reuse the generated declarations; the two line-break and the four
// lowering-helper exports are declared here beside their JavaScript
// implementations. The export order matches src/facade.mjs and the
// compatibility contract in package.test.mts.

import {
  bopomofoParse,
  numberSymbolCohesionUnbreakableRanges,
  fontMetricsResolve,
  fontFallbackResolve,
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
declare function classifyFontRole(text: string, start: number, end: number, locale: string): string;
declare function classifyFontRoles(
  text: string,
  starts: number[],
  ends: number[],
  locale: string,
): string[];
declare function unsupportedInlineShapingProperties(): string[];
declare function firstDivergentInlineShapingProperty(
  elementValues: string[],
  paragraphValues: string[],
): string | null;

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
