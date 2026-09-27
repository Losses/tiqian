// The public entry of `@tiqian/ffi`. Two engine capabilities stay on the
// Kotlin/JS runtime (./Tiqian-tiqian-ffi-js.mjs); the line-break, clreq, font
// and lowering-helper capabilities are served directly by the single-source
// generated tree (./engine-gen). The export surface — twelve names, in this
// export order — is the compatibility contract pinned by package.test.mts, so
// the generated-tree names keep their original positions in the list.

import * as engine from "./Tiqian-tiqian-ffi-js.mjs";
import { liangHyphenate, unicodePunctuationLineBreakClassOf } from "./linebreak-facade.mjs";
import { bopomofoParse, numberSymbolCohesionUnbreakableRanges } from "./clreq-facade.mjs";
import { fontMetricsResolve, fontFallbackResolve } from "./font-facade.mjs";
import {
  classifyFontRole,
  classifyFontRoles,
  unsupportedInlineShapingProperties,
  firstDivergentInlineShapingProperty,
} from "./loweringhelper-facade.mjs";

const {
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
