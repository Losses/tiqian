import org.gradle.api.tasks.Sync

plugins {
    kotlin("multiplatform")
}

kotlin {
    js {
        // The published `@tiqian/ffi` package (`ffi/js/npm`) serves every
        // capability from the generated TypeScript tree under engine-gen/;
        // this target keeps the module shell without protocol sources.
        outputModuleName.set("Tiqian-tiqian-ffi-js")
        nodejs()
        useEsModules()
        binaries.executable()
        // Ship the @JsExport surface with TypeScript definitions next to the
        // executable module (ADR 0053 A4); source maps embed their sources by
        // default, so the published .map files stand alone.
        generateTypeScriptDefinitions()
    }
}

tasks.register<Sync>("assembleNpmPackage") {
    group = "distribution"
    description = "Builds the @tiqian/ffi ESM package runtime."
    // The Kotlin/JS runtime no longer contributes to the published package:
    // since the precompute cutover every capability is served by the
    // generated TypeScript tree, and the facade artifacts are copied in
    // afterwards by npm/build-runtime.ts. The Sync still owns runtime/ and
    // wipes anything it does not carry.
    into(layout.projectDirectory.dir("npm/runtime"))
}
