// The public entry of `@tiqian/ffi`. Eight engine capabilities stay on the
// Kotlin/JS runtime (./Tiqian-tiqian-ffi-js.mjs); the two line-break and the
// four lowering-helper capabilities are served directly by the single-source
// generated tree (../engine-gen). The export surface — twelve names, in this
// export order — is the compatibility contract pinned by package.test.mts,
// so the served names keep their original positions in the list.

import * as engine from "./Tiqian-tiqian-ffi-js.mjs";
import { liangHyphenate, unicodePunctuationLineBreakClassOf } from "./linebreak-facade.mjs";
import {
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
} from "./loweringhelper-facade.mjs";

const {
  bopomofoParse,
  numberSymbolCohesionUnbreakableRanges,
  fontMetricsResolve,
  fontFallbackResolve,
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
