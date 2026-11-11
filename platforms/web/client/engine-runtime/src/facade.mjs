// The public entry of `@tiqian/ffi`. Every capability is served directly by
// the single-source generated tree (./engine-gen): the line-break, clreq,
// font and lowering-helper capabilities since their cutover waves, and the
// two precompute capabilities since this wave. The export surface — twelve
// names, in this export order — is the compatibility contract pinned by
// package.test.mts, so the generated-tree names keep their original
// positions in the list.

import { liangHyphenate, unicodePunctuationLineBreakClassOf } from "./linebreak-facade.mjs";
import { bopomofoParse, numberSymbolCohesionUnbreakableRanges } from "./clreq-facade.mjs";
import { fontMetricsResolve, fontFallbackResolve } from "./font-facade.mjs";
import {
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
} from "./loweringhelper-facade.mjs";
import {
  precomputeParagraphWithDiagnostics,
  precomputeParagraphWithBrowserMetrics,
} from "./precompute-facade.mjs";

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
