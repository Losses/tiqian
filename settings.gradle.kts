pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "1.0.0"
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.PREFER_SETTINGS)
    repositories {
        google()
        mavenCentral()
        // Kotlin/JS downloads a Node toolchain for webpack; under
        // FAIL_ON_PROJECT_REPOS the download source must be declared here. These
        // exclusive-content ivy repos serve ONLY org.nodejs:node / com.yarnpkg:yarn.
        exclusiveContent {
            forRepository {
                ivy("https://nodejs.org/dist/") {
                    name = "Node Distributions"
                    patternLayout { artifact("v[revision]/[artifact](-v[revision]-[classifier]).[ext]") }
                    metadataSources { artifact() }
                    content { includeModule("org.nodejs", "node") }
                }
            }
            filter { includeGroup("org.nodejs") }
        }
        exclusiveContent {
            forRepository {
                ivy("https://github.com/yarnpkg/yarn/releases/download") {
                    name = "Yarn Distributions"
                    patternLayout { artifact("v[revision]/[artifact](-v[revision]).[ext]") }
                    metadataSources { artifact() }
                    content { includeModule("com.yarnpkg", "yarn") }
                }
            }
            filter { includeGroup("com.yarnpkg") }
        }
    }
}

rootProject.name = "tiqian"

// ============================================================================
// Legacy deprecation (Phase 1): ":engine" is the Legacy hand-written Kotlin
// engine, retained ONLY as the Kotlin-platform consumption surface while the
// engine-haxe migration completes. engine-haxe/ is the SOLE ACTIVE SOURCE;
// boring generates Kotlin / Swift / Dart / JS / Rust from it. Do not add new
// code to ":engine"; migrate to engine-haxe.
// ============================================================================
logger.warn("tiqian deprecation: :engine is the Legacy Kotlin engine - migrate to engine-haxe (boring-generated multi-target)")

include(
    ":engine", // Legacy - migrate to engine-haxe
    ":platforms:jvm:shaping",
    ":platforms:jvm:skia",
    ":platforms:android:shaping",
    ":platforms:android:native-font",
    ":platforms:android:rendering",
    ":platforms:android:view",
    ":platforms:android:benchmark",
    ":platforms:apple:shaping",
    ":platforms:apple:frontend",
    ":platforms:apple:frontend:coretext-render",
    ":platforms:compose:compose",
    ":platforms:compose:material3",
    ":ffi:js",
    ":demo",
    ":demo:android",
    ":demo:font-diagnostics",
)
