#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { copyFile, mkdir, rm } from "node:fs/promises";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);

// The Kotlin/JS runtime no longer contributes to the published package:
// since the precompute cutover every capability is served by the generated
// TypeScript tree under engine-gen/. runtime/ is a pure build product, so it
// is wiped directly here instead of routing through a Gradle Sync task that
// configured the whole Kotlin MPP graph (":engine" plus its Android and
// Kotlin/Native targets) only to empty one directory. Dropping the Gradle
// round-trip removes the JDK/Android/Native toolchain requirement from
// build:runtime.
const engineGenDirectory = fileURLToPath(new URL("./engine-gen/", import.meta.url));
if (!existsSync(engineGenDirectory)) {
  const repoRoot = fileURLToPath(new URL("../../../../", import.meta.url));
  const generator = fileURLToPath(new URL("../../../../scripts/generate-engine-gen.mjs", import.meta.url));
  console.log("build:runtime: engine-gen/ is missing, generating it from the ts bundles");
  const generate = spawnSync(process.execPath, [generator, "ts"], { cwd: repoRoot, stdio: "inherit" });
  if (generate.error) throw generate.error;
  if (generate.status !== 0) process.exit(generate.status ?? 1);
}

const runtimeDirectory = fileURLToPath(new URL("./runtime/", import.meta.url));
await rm(runtimeDirectory, { recursive: true, force: true });
await mkdir(runtimeDirectory, { recursive: true });

// The single-source TypeScript tree under engine-gen/ is compiled with
// rewriteRelativeImportExtensions (P6 precedent: relative .ts specifiers are
// rewritten to .js in the emitted JavaScript), because the generated modules
// import interfaces as value imports, which Node type stripping cannot link.
// npm workspaces hoist typescript to the repository root, so the CLI is
// located through module resolution instead of a fixed relative path.
const typescriptBinary = require.resolve("typescript/lib/typescript.js").replace(
  /lib\/typescript\.js$/u,
  "bin/tsc",
);
const tsc = spawnSync(process.execPath, [typescriptBinary, "-p", "tsconfig.engine-gen.json"], {
  cwd: fileURLToPath(new URL("./", import.meta.url)),
  stdio: "inherit",
});
if (tsc.error) throw tsc.error;
if (tsc.status !== 0) {
  process.exitCode = tsc.status ?? 1;
} else {
  for (const facadeFile of ["facade.mjs", "facade.d.mts", "linebreak-facade.mjs", "clreq-facade.mjs", "font-facade.mjs", "loweringhelper-facade.mjs", "precompute-facade.mjs"]) {
    await copyFile(fileURLToPath(new URL(`./src/${facadeFile}`, import.meta.url)), runtimeDirectory + facadeFile);
  }
}
