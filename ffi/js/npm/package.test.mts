import assert from "node:assert/strict";
import { readFile, readdir } from "node:fs/promises";
import test from "node:test";

interface PackageManifest {
  name: string;
  license: string;
  engines: { node: string };
  publishConfig: { access: string; tag: string };
  files: string[];
  exports: {
    ".": {
      types: string;
      default: string;
    };
  };
  dependencies?: undefined;
  bin?: undefined;
  scripts: {
    prepack: string;
  };
}

interface SourceMap {
  sources: string[];
  sourcesContent?: string[];
}

const MAPS_WITHOUT_SOURCES: ReadonlySet<string> = new Set([
  "kotlin_org_jetbrains_kotlin_kotlin_dom_api_compat.mjs.map",
]);

interface PrepareParagraphRequest {
  text: string;
  maxWidthPx: number;
  fontFamilies: string[];
  fontSizePx: number;
  lineHeightPx: number;
  locale: string;
  fontWeight: number;
  italic: boolean;
  firstLineIndentIc: number;
  lineLengthGridEnabled: boolean;
  sourceBoundaries: number[];
  textSpans: Array<{
    start: number;
    end: number;
    fontFamilies: string[];
    fontSize: number;
    fontWeight: number;
    italic: boolean;
    baselineShift: number;
  }>;
  inlineBoxes: Array<{
    start: number;
    end: number;
    inlineStart: number;
    inlineEnd: number;
    outerSpacing: string;
  }>;
  lineBreakSpans: Array<{
    start: number;
    end: number;
    policy: string;
  }>;
  inlineObjects: Array<{
    start: number;
    end: number;
    advance: number;
    ascent: number;
    descent: number;
  }>;
  decorations: Array<{
    start: number;
    end: number;
    kind: string;
  }>;
  emphasisDotGapEm: number | null;
  renderEvidenceOverride: boolean | null;
}

interface FfiExports {
  bopomofoParse: (reading: string) => string;
  numberSymbolCohesionUnbreakableRanges: (text: string) => string;
  fontMetricsResolve: (requestJson: string) => string;
  fontFallbackResolve: (text: string, start: number, end: number, requestJson: string) => string;
  liangHyphenate: (word: string, patternsJson: string, exceptionsJson: string, leftMin?: number, rightMin?: number) => string;
  unicodePunctuationLineBreakClassOf: (codePoint: number) => string;
  classifyFontRole: (text: string, start: number, end: number, locale: string) => string;
  classifyFontRoles: (text: string, starts: number[], ends: number[], locale: string) => string[];
  unsupportedInlineShapingProperties: () => string[];
  firstDivergentInlineShapingProperty: (elementValues: string[], paragraphValues: string[]) => string | null;
  precomputeParagraphWithDiagnostics: (
    request: PrepareParagraphRequest,
    zeroAdvanceEpsilonPx: number,
    shapeJson: (p0: string) => string,
    metricsJson: (p0: string) => string,
  ) => string;
  precomputeParagraphWithBrowserMetrics: (
    request: PrepareParagraphRequest,
    zeroAdvanceEpsilonPx: number,
    callbacks: { shapeJson: (p0: string) => string; metricsJson: (p0: string) => string },
  ) => string;
}

test("the manifest ships the generated engine runtime and nothing else", async () => {
  const manifest = JSON.parse(
    await readFile(new URL("./package.json", import.meta.url), "utf8"),
  ) as PackageManifest;

  assert.equal(manifest.name, "@tiqian/ffi");
  assert.equal(manifest.license, "MPL-2.0");
  assert.equal(manifest.engines.node, ">=22");
  assert.deepEqual(manifest.publishConfig, { access: "public", tag: "alpha" });
  assert.deepEqual(manifest.files, ["LICENSE", "README.md", "runtime/", "engine-gen/"]);
  assert.deepEqual(manifest.exports, {
    ".": {
      types: "./runtime/facade.d.mts",
      default: "./runtime/facade.mjs",
    },
  });
  assert.equal(manifest.dependencies, undefined);
  assert.equal(manifest.bin, undefined);
  assert.equal(
    manifest.scripts.prepack,
    "npm run build:runtime && npm test && npm run verify:package",
  );
});

test("the package export surface names all twelve capabilities in order", async () => {
  // Cutover mechanism note: the entry used to be the Kotlin-generated module
  // whose .d.mts this test scanned; it is now the aggregate facade, so the
  // pinned surface is read off the facade's export block. The runtime module
  // namespace cannot carry the order (its keys are sorted by spec), and the
  // order is part of the contract. The pinned list — twelve names and their
  // order — is unchanged.
  const facade = await readFile(new URL("./runtime/facade.mjs", import.meta.url), "utf8");
  const exportBlock = facade.match(/export \{([^}]+)\};/u)?.[1];
  assert.ok(exportBlock, "facade.mjs carries a single export block");

  const exported = exportBlock.split(",").map((name) => name.trim()).filter(Boolean);
  assert.deepEqual(exported, [
    "bopomofoParse",
    "numberSymbolCohesionUnbreakableRanges",
    "fontMetricsResolve",
    "fontFallbackResolve",
    "liangHyphenate",
    "unicodePunctuationLineBreakClassOf",
    "classifyFontRole",
    "classifyFontRoles",
    "unsupportedInlineShapingProperties",
    "firstDivergentInlineShapingProperty",
    "precomputeParagraphWithDiagnostics",
    "precomputeParagraphWithBrowserMetrics",
  ]);
});

test("every engine module ships a source map with embedded sources", async () => {
  const entries = await readdir(new URL("./runtime/", import.meta.url));
  const modules = entries.filter((entry) => entry.endsWith(".mjs"));

  assert.ok(
    modules.length >= 4,
    "the runtime keeps the full module set (engine is a single published module)",
  );
  // Cutover mechanism note: the facade and the line-break wire translation
  // are hand-written entry shims, not engine modules; the embedded-sources
  // contract keeps applying to the Kotlin-generated engine modules.
  const entryShims = new Set(["facade.mjs", "linebreak-facade.mjs"]);
  for (const module of modules) {
    if (entryShims.has(module)) continue;
    const map = `${module}.map`;
    assert.ok(entries.includes(map), `runtime/${module} has no source map`);
    if (MAPS_WITHOUT_SOURCES.has(map)) continue;
    const parsed = JSON.parse(
      await readFile(new URL(`./runtime/${map}`, import.meta.url), "utf8"),
    ) as SourceMap;
    assert.ok(parsed.sources.length > 0, `runtime/${map} has no sources`);
    assert.ok(
      (parsed.sourcesContent ?? []).length >= parsed.sources.length,
      `runtime/${map} does not embed its sources`,
    );
  }
});

test("the engine entry loads from the package exports surface", async () => {
  const ffi = (await import("@tiqian/ffi")) as unknown as FfiExports;

  assert.equal(typeof ffi.bopomofoParse, "function");
  assert.equal(typeof ffi.numberSymbolCohesionUnbreakableRanges, "function");
  assert.equal(typeof ffi.fontMetricsResolve, "function");
  assert.equal(typeof ffi.fontFallbackResolve, "function");
  assert.equal(typeof ffi.liangHyphenate, "function");
  assert.equal(typeof ffi.unicodePunctuationLineBreakClassOf, "function");
  assert.equal(typeof ffi.classifyFontRole, "function");
  assert.equal(typeof ffi.classifyFontRoles, "function");
  assert.equal(typeof ffi.unsupportedInlineShapingProperties, "function");
  assert.equal(typeof ffi.firstDivergentInlineShapingProperty, "function");
  assert.equal(typeof ffi.precomputeParagraphWithDiagnostics, "function");
  assert.equal(typeof ffi.precomputeParagraphWithBrowserMetrics, "function");
  assert.match(import.meta.resolve("@tiqian/ffi"), /facade\.mjs$/u);
});

test("classifyFontRole maps classifier roles to lowering role strings", async () => {
  const ffi = (await import("@tiqian/ffi")) as unknown as FfiExports;

  assert.equal(ffi.classifyFontRole("汉字", 0, 2, "zh-Hans"), "cjk-text");
  assert.equal(ffi.classifyFontRole("，", 0, 1, "zh-Hans"), "cjk-punctuation");
  assert.equal(ffi.classifyFontRole("Hello", 0, 5, "en"), "other");
});

test("classifyFontRoles resolves contextual marks from complete paragraph text", async () => {
  const ffi = (await import("@tiqian/ffi")) as unknown as FfiExports;

  assert.deepEqual(
    ffi.classifyFontRoles("A——B中文……下句", [1, 2, 6, 7], [2, 3, 7, 8], "zh-Hans"),
    ["other", "other", "cjk-punctuation", "cjk-punctuation"],
  );
});

test("classifyFontRole resolves curly quotes through structural pair analysis", async () => {
  const ffi = (await import("@tiqian/ffi")) as unknown as FfiExports;

  assert.equal(ffi.classifyFontRole("word“中文”word", 4, 5, "zh-Hans"), "other");
  assert.equal(ffi.classifyFontRole("word“中文”word", 7, 8, "zh-Hans"), "other");
});

test("unsupportedInlineShapingProperties returns fresh ordered property array", async () => {
  const ffi = (await import("@tiqian/ffi")) as unknown as FfiExports;

  const properties1 = ffi.unsupportedInlineShapingProperties();
  const properties2 = ffi.unsupportedInlineShapingProperties();

  assert.equal(properties1.length, 16);
  assert.equal(properties1[0], "font-feature-settings");
  assert.equal(properties1[1], "font-variation-settings");
  assert.equal(properties1[2], "font-stretch");
  assert.deepEqual(properties1, properties2);
  assert.notEqual(properties1, properties2, "consecutive calls return distinct array instances");
});

test("firstDivergentInlineShapingProperty detects divergence and clamps common prefix", async () => {
  const ffi = (await import("@tiqian/ffi")) as unknown as FfiExports;

  assert.equal(
    ffi.firstDivergentInlineShapingProperty(
      ["normal", "normal", "normal"],
      ["normal", "normal", "normal"],
    ) ?? null,
    null,
  );

  assert.equal(
    ffi.firstDivergentInlineShapingProperty(
      ["normal", "normal", "expanded"],
      ["normal", "normal", "condensed"],
    ),
    "font-stretch",
  );

  assert.equal(
    ffi.firstDivergentInlineShapingProperty(
      ["normal", "normal", "normal", "none"],
      ["normal", "normal", "normal", "auto"],
    ),
    "font-kerning",
  );

  assert.equal(
    ffi.firstDivergentInlineShapingProperty(
      ["normal", "normal"],
      ["normal", "normal", "expanded"],
    ) ?? null,
    null,
  );
  assert.equal(
    ffi.firstDivergentInlineShapingProperty(
      ["normal", "normal", "expanded"],
      ["normal", "normal"],
    ) ?? null,
    null,
  );
});