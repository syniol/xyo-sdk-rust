# Changelog

All notable changes to the XYO Financial SDK for Rust will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.9] - 2026-08-07

### 🚀 Version Bump
- **xyo-sdk**: Incremented version from `1.1.8` to `1.1.9` in `Cargo.toml`.
- **xyo-http**: Incremented version from `1.1.8` to `1.1.9` in `xyo-http/Cargo.toml`.
- **Dependencies**: Updated `xyo-http` workspace dependency to `1.1.9` in root `Cargo.toml` and updated `xyo-sdk` dependency to `1.1.9` in `example/Cargo.toml`.
- **Lockfiles**: Updated `Cargo.lock` and `example/Cargo.lock` to reflect version `1.1.9`.

### 🔄 Repository Migration
- **GitHub URL**: Updated repository URLs from `https://github.com/syniol/xyo-sdk-rust` to `https://github.com/xyo-financial/sdk-rust` in `Cargo.toml`, `xyo-http/Cargo.toml`, and `README.md`.
- **CI Badges & Assets**: Updated workflow status badges and mascot image links in `README.md` to point to the new organization repository.

## [1.1.8] - 2026-07-20

### 🚀 Version Bump
- **xyo-sdk**: Incremented version from `1.1.7` to `1.1.8` in `Cargo.toml`.
- **xyo-http**: Incremented version from `1.1.7` to `1.1.8` in `xyo-http/Cargo.toml`.
- **Dependencies**: Updated `xyo-http` workspace dependency to `1.1.8` in root `Cargo.toml` and updated `xyo-sdk` dependency to `1.1.8` in `example/Cargo.toml`.
- **Lockfiles**: Updated `Cargo.lock` and `example/Cargo.lock` to reflect version `1.1.8`.

### 📄 Licensing
- **Relicensed to BSD 3-Clause**: Changed project license to the `BSD 3-Clause License` (`BSD-3-Clause`).
- **LICENSE**: Replaced license text in `LICENSE` file with BSD 3-Clause License terms for Syniol Limited.
- **Package Manifests**: Updated SPDX `license` metadata field to `"BSD-3-Clause"` in `Cargo.toml` and `xyo-http/Cargo.toml`.
- **Documentation**: Updated license references in `README.md` and `xyo-http/README.md`.

### ⚙️ CI/CD & Workflows
- **Publish Workflow**: Refined release tag push filter pattern to `v[0-9]+.[0-9]+.[0-9]+` in `.github/workflows/crates_xyo_http_publish.yml`.
