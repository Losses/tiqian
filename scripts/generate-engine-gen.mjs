#!/usr/bin/env node
// Generates the Rust engine tree that used to be committed:
//   platforms/web/server/precompute/engine-gen  (bundle engine-rust-f64)
//
// The directory is a build product and is gitignored. Run the script inside
// `nix develop` (the flake provides the boring driver):
//   node scripts/generate-engine-gen.mjs
// The Rust build path runs this before `cargo build -p tiqian`.
//
// The JavaScript engine-gen tree is still committed. The ts bundle does not
// emit the production modules the JS facade imports (the emitter classifies
// them as test-only because their only Haxe referencers are test modules);
// see the follow-up task for that gap.

import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const outRoot = join(repoRoot, "engine-haxe", "out");
const rustGen = join(repoRoot, "platforms/web/server/precompute/engine-gen");

function runDriver() {
  try {
    execFileSync("boring", ["gen", "engine-rust-f64", "--project", "boring.json"], {
      cwd: repoRoot,
      stdio: "inherit",
    });
  } catch (error) {
    console.error(
      "generate-engine-gen: the boring driver failed or is missing. " +
        "Run this script inside `nix develop` (the flake provides `boring`).",
    );
    throw error;
  }
}

const generated = join(outRoot, "engine-rust-f64", "gen");
runDriver();
if (!existsSync(generated)) {
  throw new Error(`generate-engine-gen: expected driver output at ${generated}`);
}
rmSync(rustGen, { recursive: true, force: true });
mkdirSync(rustGen, { recursive: true });
cpSync(generated, rustGen, { recursive: true });
const tests = join(outRoot, "engine-rust-f64", "gen-tests");
if (existsSync(tests)) {
  cpSync(tests, rustGen, { recursive: true });
}
console.log(`generate-engine-gen: wrote ${rustGen}`);
