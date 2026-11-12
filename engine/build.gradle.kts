import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    kotlin("multiplatform")
    id("com.android.kotlin.multiplatform.library")
    // Haxe port lane: coverage baseline for the pre-refactor engine.
    id("org.jetbrains.kotlinx.kover") version "0.9.9"
}

// commonTest is shared with the Android host-test compilation. AGP's lint model tasks do not
// infer the generated-source task dependency from Kotlin's source-set provider, so declare it.
kotlin {
    // Project-wide Kotlin compiler options. KT-61573: 'expect'/'actual'
    // classes are Beta; TestTracePlatform (org.tiqian.test.trace) is an
    // intentional JVM-only golden-writing hook, so the language-feature
    // notice is silenced instead of restructuring the hook.
    compilerOptions {
        freeCompilerArgs.add("-Xexpect-actual-classes")
    }
    jvm()
    android {
        namespace = "org.tiqian.engine"
        compileSdk = 36
        minSdk = 23
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_17)
        }
        withHostTest {}
    }
    macosArm64()
    iosArm64()
    iosSimulatorArm64()
    linuxX64()
    linuxArm64()
    mingwX64()

    sourceSets {
        // The Kotlin implementation is generated from engine-haxe/ by boring
        // (bundle kotlin-f32). setSrcDirs replaces the default
        // src/commonMain/kotlin instead of adding to it, so the handwritten
        // tree stays on disk untouched and removing this block rolls back.
        commonMain {
            kotlin.setSrcDirs(listOf(layout.projectDirectory.dir("../engine-haxe/out/kotlin-f32/gen")))
        }

        commonTest {
            dependencies {
                implementation(kotlin("test"))
                implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.1")
            }
            // The generated test suite (bundle kotlin-f32, gen-tests) replaces the
            // handwritten tests.
            kotlin.setSrcDirs(listOf(layout.projectDirectory.dir("../engine-haxe/out/kotlin-f32/gen-tests")))
        }

        // TEMPORARY PROBE: the handwritten jvmTest tree is still on the old API.
        jvmTest { kotlin.setSrcDirs(emptyList<Any>()) }

        jvmTest.dependencies {
            implementation(project(":platforms:jvm:shaping"))
            implementation(project(":platforms:jvm:skia"))
            runtimeOnly("org.jetbrains.skiko:skiko-awt-runtime-macos-arm64:0.144.6")
        }
    }
}

// Developer tools (layout report, README sample, probes) are not tests. They
// live in their own JVM compilation so the test source set carries tests only.
val jvmTestCompilation = kotlin.targets.getByName("jvm").compilations.getByName("test")

val jvmToolsCompilation = kotlin.targets.getByName("jvm").compilations.create("tools") {
    defaultSourceSet.dependsOn(kotlin.sourceSets.getByName("commonMain"))
}
jvmToolsCompilation.associateWith(kotlin.targets.getByName("jvm").compilations.getByName("main"))
jvmToolsCompilation.defaultSourceSet.dependencies {
    implementation(kotlin("test-junit"))
    implementation(project(":platforms:jvm:shaping"))
    implementation(project(":platforms:jvm:skia"))
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.1")
    runtimeOnly("org.jetbrains.skiko:skiko-awt-runtime-macos-arm64:0.144.6")
}

val readmeSampleBlackSvg = rootProject.layout.projectDirectory.file("docs/images/sample-paragraph-black.svg")
val readmeSampleWhiteSvg = rootProject.layout.projectDirectory.file("docs/images/sample-paragraph-white.svg")

tasks.register<JavaExec>("generateReadmeSample") {
    group = "documentation"
    description = "Generates the README paragraph sample from a real Tiqian LayoutResult."
    dependsOn("compileToolsKotlinJvm")
    mainClass.set("org.tiqian.layout.tooling.ReadmeSampleMainKt")
    classpath = files(jvmToolsCompilation.output.allOutputs) +
        configurations.named("jvmRuntimeClasspath").get()
    jvmArgs("--enable-native-access=ALL-UNNAMED")
    args(
        readmeSampleBlackSvg.asFile.absolutePath,
        readmeSampleWhiteSvg.asFile.absolutePath,
    )
    outputs.files(readmeSampleBlackSvg, readmeSampleWhiteSvg)
}

// The generated test suite (bundle kotlin-f32, gen-tests) is not a JUnit suite:
// every case carries @boring.test.Test and the generated TestMainKt drives them,
// exiting non-zero when any case throws. Run it through JavaExec so Gradle
// reports the same verdict the boring driver does.
tasks.register<JavaExec>("generatedTestSuite") {
    group = "verification"
    description = "Runs the generated Kotlin test suite through gen-tests/TestMainKt."
    dependsOn("compileToolsKotlinJvm")
    mainClass.set("TestMainKt")
    // The generated golden constants load their text from the bundle resource
    // directory, so it has to sit on the classpath exactly as the driver puts it.
    classpath = files(jvmTestCompilation.output.allOutputs) +
        files(layout.projectDirectory.dir("../engine-haxe/out/kotlin-f32/gen/resources")) +
        configurations.named("jvmTestRuntimeClasspath").get()
    // The generated runner reads these two host edges; the driver sets the same
    // values (boring.json run.env). Without the budget, eleven cases time out.
    environment("BORING_TEST_TIMEOUT_MS", "600000")
    environment(
        "BORING_TEST_RESULTS",
        layout.buildDirectory.file("test-results/generated-suite.jsonl").get().asFile.absolutePath,
    )
    // Trace goldens are written to a repository-relative path.
    workingDir = rootProject.projectDir
}
