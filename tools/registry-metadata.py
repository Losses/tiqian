#!/usr/bin/env python3
"""Write the registry metadata tree for one release.

The registry tool accepts exactly one metadata.json per version and platform
(packages/registry/src/registry/Core.hx:280), so every package owns its own
repository layer and the tree path is <owner>/<package>/<version>/<platform>/.

Usage:
  registry-metadata.py --project boring.json --artifacts release-artifacts \
      --version 0.1.0-alpha06 --owner tiqian-cjk --repo tiqian \
      --asset-base https://github.com/tiqian-cjk/tiqian/releases/download/v0.1.0-alpha06 --out tree
"""
import argparse
import base64
import hashlib
import json
import pathlib
import re
import sys

TARGET_PLATFORM = {"rust": "cargo", "ts": "npm", "kotlin": "maven", "swift": "swift", "dart": "pub"}
PLATFORM_DIGEST = {"cargo": "sha256", "pub": "sha256", "swift": "sha256", "npm": "sha512", "maven": "sha1"}
PLATFORM_SUFFIX = {"cargo": ".crate", "pub": ".tar.gz", "swift": ".zip", "npm": ".tgz", "maven": ".jar"}


def load_bundles(path):
    text = pathlib.Path(path).read_text()
    text = re.sub(r"^\s*//.*$", "", text, flags=re.M)
    return json.loads(text)["bundles"]


def artifact_stem(bundle):
    """The name the target writes into the artifact file name.

    A bundle may set -D package-artifact-stem=<stem> in its haxeArgs. Bundles
    without the define use the package name.
    """
    args = bundle.get("haxeArgs") or []
    for index, arg in enumerate(args):
        if arg == "-D" and index + 1 < len(args) and args[index + 1].startswith("package-artifact-stem="):
            return args[index + 1].split("=", 1)[1]
        if arg.startswith("-Dpackage-artifact-stem="):
            return arg.split("=", 1)[1]
    return bundle["package"]["name"]


def registry_name(bundle, platform):
    """The registry repository name one bundle publishes under.

    Every platform under one version shares the directory name, and the
    registry tool rejects a version whose platforms disagree
    (registry/Core.hx validate: "platform disagreement"), so the name has to
    match what the ecosystem itself calls the artifact.

    Maven and SwiftPM name one artifact per build configuration: the Maven POM
    carries groupId <package.name> and artifactId <stem>, and the SwiftPM
    manifest name is the stem, so those two platforms publish under the stem.
    A cargo crate is identified by its package name on every path cargo
    touches: the index file, the resolved dependency name, the tarball, and
    the /cargo/dl/<name>-<version>.crate redirect. A cargo crate therefore
    publishes under the package name, and the stem stays a file-name detail.
    """
    if platform in ("maven", "swift"):
        return artifact_stem(bundle)
    return bundle["package"]["name"]


def cargo_features(bundles, package_name):
    """The cargo features the index has to declare for one package.

    A package shared by several build configurations carries the configuration
    switch as a cargo feature: the Rust target emits a cfg(feature = "f32")
    gate and one crate built from one source has to let the consumer choose.
    Each configuration bundle names the feature it is built and tested with,
    so the union of those values is the feature list to declare.
    """
    features = []
    for bundle in bundles:
        package = bundle.get("package") or {}
        if package.get("name") != package_name:
            continue
        for step in ("build", "run"):
            override = bundle.get(step) or {}
            for key in ("args", "cargoArgs"):
                values = override.get(key) or []
                for index, value in enumerate(values):
                    if value == "--features" and index + 1 < len(values):
                        declared = values[index + 1]
                    elif value.startswith("--features="):
                        declared = value.split("=", 1)[1]
                    else:
                        continue
                    for feature in declared.split(","):
                        feature = feature.strip()
                        if feature and feature not in features:
                            features.append(feature)
    features.sort()
    return features


def digest_of(path, algo, encoding="hex"):
    h = hashlib.new(algo)
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            h.update(block)
    if encoding == "base64":
        # npm integrity is base64; the registry prefixes it with "sha512-".
        return base64.b64encode(h.digest()).decode("ascii")
    return h.hexdigest()


def parse_pubspec(text):
    """Read the flat pubspec the Dart target writes into a JSON object."""
    document = {}
    section = None
    for line in text.splitlines():
        if not line.strip() or line.strip().startswith("#"):
            continue
        if not line.startswith(" "):
            key, _, value = line.partition(":")
            key = key.strip()
            value = value.strip()
            if value:
                document[key] = value
                section = None
            else:
                document[key] = {}
                section = key
        elif section:
            key, _, value = line.strip().partition(":")
            document[section][key.strip()] = value.strip()
    return document


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--project", required=True)
    parser.add_argument("--list-packages", action="store_true")
    parser.add_argument("--artifacts")
    parser.add_argument("--version")
    parser.add_argument("--owner")
    parser.add_argument("--repo")
    parser.add_argument("--asset-base",
                        help="where the published archives are downloaded from; the release asset URL")
    parser.add_argument("--out")
    options = parser.parse_args()

    bundles = load_bundles(options.project)
    if options.list_packages:
        seen = []
        for item in bundles:
            package = item.get("package")
            if package and package["name"] not in seen:
                seen.append(package["name"])
        print("\n".join(seen))
        return

    artifacts = pathlib.Path(options.artifacts)
    out = pathlib.Path(options.out)
    written = []
    bundles = load_bundles(options.project)

    for bundle in bundles:
        package = bundle.get("package")
        if not package:
            continue
        platform = TARGET_PLATFORM.get(bundle["target"])
        if platform is None or platform not in PLATFORM_SUFFIX:
            continue
        stem = artifact_stem(bundle)
        if platform == "cargo" and stem != package["name"]:
            # A precision companion packs the same payload under a file-name
            # stem so that packing both configurations does not collide on
            # disk. The crate it writes is the package crate with a different
            # top-level directory name, so it is not a package of its own and
            # writes no second entry.
            continue
        suffix = PLATFORM_SUFFIX[platform]
        # The artifact carries the stem; the registry repository carries the
        # name the ecosystem resolves the package by.
        artifact_name = stem.replace("-", "_") if platform == "pub" else stem
        candidates = sorted(artifacts.glob(artifact_name + "-*" + suffix))
        if not candidates:
            continue
        archive = candidates[0]
        name = registry_name(bundle, platform)
        dir_name = name.replace("-", "_") if platform == "pub" else name
        repo_dir = out / options.owner / dir_name
        repo_dir.mkdir(parents=True, exist_ok=True)
        readme = repo_dir / "README.md"
        if not readme.exists():
            readme.write_text("# " + name + "\n\nPackage metadata for " + name + ".\n")
        platform_dir = repo_dir / options.version / platform
        platform_dir.mkdir(parents=True, exist_ok=True)
        url = "/".join([
            options.asset_base.rstrip("/"),
            archive.name,
        ])
        metadata = {
            "name": name,
            "version": options.version,
            "license": package.get("license", "MIT"),
            "repository": options.owner + "/" + options.repo,
            "tag": "v" + options.version,
        }
        if platform == "maven":
            # The registry reads a group id and one entry per published file.
            # The tree directory is the artifactId, so the group id stays the
            # package name and one group can carry both precision builds.
            metadata["groupId"] = package["name"]
            metadata["artifacts"] = [
                {"file": entry.name, "url": "/".join([options.asset_base.rstrip("/"), entry.name])}
                for entry in sorted(artifacts.glob(stem + "-*"))
                if entry.suffix in (".jar", ".pom")
            ]
        elif platform == "swift":
            # The registry checks the archive path (registry/Core.hx:513) and
            # writes swift/<scope>/<name>/<version>/Package.swift from this
            # field, so it carries the manifest text rather than a URL.
            import zipfile
            with zipfile.ZipFile(archive) as bundle_zip:
                # Members sit under <package>-<version>/ since the archive
                # carries the single root directory SwiftPM requires.
                manifest = next(n for n in bundle_zip.namelist() if n.endswith("/Package.swift"))
                metadata["packageSwift"] = bundle_zip.read(manifest).decode("utf-8")
            metadata["archive"] = "swift/tiqian/" + name + "/" + options.version + ".zip"
            metadata["sha256"] = digest_of(archive, "sha256")
        else:
            if platform == "pub":
                # Dart identifiers allow lowercase letters, digits and
                # underscores; the packed pubspec uses the same form, and the
                # hosted API serves the parsed pubspec with every version.
                import tarfile
                metadata["name"] = name.replace("-", "_")
                with tarfile.open(archive) as bundle_tar:
                    member = next(m for m in bundle_tar.getmembers() if m.name.endswith("pubspec.yaml"))
                    pubspec_text = bundle_tar.extractfile(member).read().decode("utf-8")
                metadata["pubspec"] = parse_pubspec(pubspec_text)
            metadata["url"] = url
            encoding = "base64" if platform == "npm" else "hex"
            metadata[PLATFORM_DIGEST[platform]] = digest_of(archive, PLATFORM_DIGEST[platform], encoding)
            if platform == "cargo":
                # Cargo resolves a dependency feature set from the index entry,
                # not from the Cargo.toml inside the tarball, so the index has
                # to declare the switch the precision companion is built with.
                metadata["features"] = {feature: [] for feature in cargo_features(bundles, package["name"])}
        (platform_dir / "metadata.json").write_text(json.dumps(metadata, sort_keys=True) + "\n")
        written.append((name, platform, archive.name))

    if not written:
        sys.exit("no pack artifacts matched a bundle with package metadata")
    for row in written:
        print("metadata:", *row)


if __name__ == "__main__":
    main()
