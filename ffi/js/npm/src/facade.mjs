// The public entry of `@tiqian/ffi`. Eight engine capabilities stay on the
// Kotlin/JS runtime (./Tiqian-tiqian-ffi-js.mjs); the line-break and clreq
// capabilities are served directly by the single-source generated tree
// (../engine-gen). The export surface — twelve names, in this export order —
// is the compatibility contract pinned by package.test.mts, so the four
// generated-tree names keep their original positions in the list.

import * as engine from "./Tiqian-tiqian-ffi-js.mjs";
import { liangHyphenate, unicodePunctuationLineBreakClassOf } from "./linebreak-facade.mjs";
import { bopomofoParse, numberSymbolCohesionUnbreakableRanges } from "./clreq-facade.mjs";

const {
  fontMetricsResolve,
  fontFallbackResolve,
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
} = engine;

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
