#!/usr/bin/env node

import { spawnSync, type SpawnSyncReturns } from "node:child_process";
import { copyFile, mkdir } from "node:fs/promises";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);

const repositoryRoot: string = fileURLToPath(new URL("../../..", import.meta.url));
const isWindows: boolean = process.platform === "win32";
const gradleWrapper: string = fileURLToPath(new URL(
  isWindows ? "../../../gradlew.bat" : "../../../gradlew",
  import.meta.url,
));
const gradleArguments: readonly string[] = [
  ":ffi:js:clean",
  ":ffi:js:assembleNpmPackage",
  "--no-build-cache",
];
// WindowsBatchWrapperViaComSpec: .bat files are cmd scripts rather than native
// executables. Invoke the wrapper through ComSpec while keeping Unix on the
// directly executable Gradle wrapper.
const command: string = isWindows
  ? (process.env.ComSpec ?? process.env.COMSPEC ?? "cmd.exe")
  : gradleWrapper;
const commandArguments: readonly string[] = isWindows
  ? ["/d", "/c", "call", gradleWrapper, ...gradleArguments]
  : gradleArguments;
const result: SpawnSyncReturns<Buffer> = spawnSync(command, commandArguments, {
  cwd: repositoryRoot,
  stdio: "inherit",
});

if (result.error) throw result.error;

// The Gradle Sync task owns runtime/ and wipes anything it does not carry, so
// the generated-tree and facade artifacts land after it. The single-source
// TypeScript tree under engine-gen/ is compiled with
// rewriteRelativeImportExtensions (P6 precedent: relative .ts specifiers are
// rewritten to .js in the emitted JavaScript), because the generated modules
// import interfaces as value imports, which Node type stripping cannot link.
if (result.status === 0) {
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
    const runtimeDirectory = fileURLToPath(new URL("./runtime/", import.meta.url));
    await mkdir(runtimeDirectory, { recursive: true });
    for (const facadeFile of ["facade.mjs", "facade.d.mts", "linebreak-facade.mjs", "font-facade.mjs"]) {
      await copyFile(fileURLToPath(new URL(`./src/${facadeFile}`, import.meta.url)), runtimeDirectory + facadeFile);
    }
  }
}
process.exitCode = process.exitCode ?? result.status ?? 1;