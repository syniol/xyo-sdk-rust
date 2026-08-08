# Contributing to XYO Financial Rust SDK

Thank you for contributing to the **XYO Financial Rust SDK** (`xyo-sdk`). This document provides institutional-grade engineering guidelines for contributing to the architecture, development, testing, generation, and maintenance of the SDK.

---

## Table of Contents

1. [Two-Layer Architecture](#1-two-layer-architecture)
   - [Generated Layer (`openapi/`) - Read-Only](#generated-layer-openapi---read-only)
   - [Wrapper Layer (`src/`) - Async Ergonomics & Tokio Integration](#wrapper-layer-src---async-ergonomics--tokio-integration)
2. [Contribution Workflow & Decision Matrix](#2-contribution-workflow--decision-matrix)
   - [Contribution Decision Matrix](#contribution-decision-matrix)
   - [Workflow A: API & Data Model Changes](#workflow-a-api--data-model-changes)
   - [Workflow B: SDK Ergonomics, Helpers & Tests](#workflow-b-sdk-ergonomics-helpers--tests)
3. [Local Code Generation](#3-local-code-generation)
   - [Prerequisites](#prerequisites)
   - [Generation Command](#generation-command)
   - [Generator Workspace Configuration](#generator-workspace-configuration)
4. [Quality Gates & Validation](#4-quality-gates--validation)
   - [1. `cargo check` (Compilation Verification)](#1-cargo-check-compilation-verification)
   - [2. `cargo test` (Unit & Integration Tests)](#2-cargo-test-unit--integration-tests)
   - [3. `cargo clippy --all-targets` (Static Analysis & Linting)](#3-cargo-clippy---all-targets-static-analysis--linting)
5. [Development & Testing Guide](#5-development--testing-guide)
   - [Running the Test Suite](#running-the-test-suite)
   - [WireMock Integration Tests](#wiremock-integration-tests)
   - [Docker Development Environment](#docker-development-environment)
6. [Pull Request & Commit Standards](#6-pull-request--commit-standards)
   - [Conventional Commits](#conventional-commits)
   - [PR Submission Checklist](#pr-submission-checklist)
7. [Release & Versioning Process](#7-release--versioning-process)

---

## 1. Two-Layer Architecture

The XYO Financial Rust SDK is engineered with a strict **Two-Layer Architecture** to decouple raw OpenAPI transport and serialization logic from high-level, idiomatic Rust developer ergonomics.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           Consumer Application                          │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                 Wrapper Layer (`src/` - HAND-CRAFTED)                   │
│  - Crate: `xyo-sdk`                                                     │
│  - Async Ergonomics & Tokio Integration (`tokio`, `reqwest`)            │
│  - High-level Client (`src/client.rs` -> `Client`)                      │
│  - Unified Error Model (`src/error.rs` -> `ClientError`)                │
│  - Curated Public API Exports (`src/lib.rs`)                            │
│  - Clean serialization types & status enums (`EnrichmentStatus`)        │
└────────────────────────────────────┬────────────────────────────────────┘
                                     │ delegates / wraps
                                     ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                 Generated Layer (`openapi/` - READ-ONLY)                │
│  - Workspace Member: `openapi-client` (v2.0.0)                          │
│  - Auto-generated via `@openapitools/openapi-generator-cli`             │
│  - Canonical Source: `xyo-financial/specs` (`openapi.yml`)              │
│  - Low-level APIs: `openapi/src/apis/enrichment_api.rs`                 │
│  - Raw DTO Models: `openapi/src/models/*`                               │
│  - Low-level Configuration: `openapi/src/apis/configuration.rs`         │
│  - STRICTLY READ-ONLY: Never modify manually                            │
└─────────────────────────────────────────────────────────────────────────┘
```

### Generated Layer (`openapi/`) - Read-Only

- **Location**: `openapi/`
- **Workspace Package**: `openapi-client`
- **Origin**: Automatically synthesized from the canonical OpenAPI 3.0/3.1 specification maintained upstream in [`xyo-financial/specs`](https://github.com/xyo-financial/specs).
- **Contents**:
  - `openapi/src/apis/`: Low-level HTTP endpoint bindings (e.g., `enrichment_api.rs`), request dispatchers, and configuration structures (`configuration.rs`).
  - `openapi/src/models/`: Raw data transfer objects (DTOs), serialization and deserialization routines via `serde`.
- **Policy**: **DO NOT edit files in `openapi/` manually.** Any manual modifications will be permanently lost during the next code generation run. All schema, route, and data model alterations must be made upstream in [`xyo-financial/specs`](https://github.com/xyo-financial/specs).

### Wrapper Layer (`src/`) - Async Ergonomics & Tokio Integration

- **Location**: `src/`
- **Crate**: `xyo-sdk`
- **Origin**: Hand-crafted and maintained directly in this repository.
- **Contents**:
  - `src/lib.rs`: The crate entry point exposing a clean, curated public API surface:
    - Re-exports `Client`, `ClientError`, `EnrichmentRequest`, `EnrichmentResponse`, `EnrichTransactionCollectionResponse`, and `EnrichmentStatus`.
  - `src/client.rs`: The primary SDK client implementation:
    - High-level async ergonomics powered by `tokio` and non-blocking I/O.
    - Seamless constructor `Client::new(bearer_token, base_url)` with default production routing (`https://api.xyo.financial`).
    - Async client methods:
      - `enrich_transaction(&self, content, country_code)`: Single transaction enrichment.
      - `enrich_transactions(&self, transactions, api_user)`: Bulk asynchronous batch submission.
      - `get_enrichment_status(&self, id, api_user)`: Polling status endpoint mapping raw responses to idiomatic `EnrichmentStatus` enum variants (`Ready`, `Pending`, `Failed`).
    - Comprehensive error mapping (`map_error`) converting low-level transport errors, JSON deserialization failures, and API status codes into structured errors.
  - `src/error.rs`: Institutional error handling:
    - Implements `ClientError` with `Display` and `std::error::Error` trait implementations.
    - Provides standardized error codes (`code: u16`) and contextual failure messages (`message: String`).
- **Policy**: All SDK usability improvements, async optimizations, builder patterns, helper functions, and ergonomic wrappers belong in this layer.

---

## 2. Contribution Workflow & Decision Matrix

To ensure consistency across the entire multi-language XYO SDK ecosystem, determine the appropriate target repository before proposing any changes.

### Contribution Decision Matrix

| Proposed Change | Destination Repository | Workflow |
| :--- | :--- | :--- |
| **API Endpoints, Routes, HTTP Methods** | [`xyo-financial/specs`](https://github.com/xyo-financial/specs) | Submit PR to OpenAPI specification; once merged, regenerate the SDK locally. |
| **Request / Response Schemas, Field Types, Enums** | [`xyo-financial/specs`](https://github.com/xyo-financial/specs) | Submit PR to OpenAPI specification; once merged, regenerate the SDK locally. |
| **API Error Codes & Status Definitions** | [`xyo-financial/specs`](https://github.com/xyo-financial/specs) | Submit PR to OpenAPI specification; once merged, regenerate the SDK locally. |
| **SDK Async Wrapper Ergonomics & Helpers** | `xyo-financial/sdk-rust` (This Repo) | Submit PR modifying files in `src/`. |
| **Error Handling & Diagnostic Structures** | `xyo-financial/sdk-rust` (This Repo) | Submit PR modifying `src/error.rs` or `src/client.rs`. |
| **Integration & Unit Tests (`wiremock`)** | `xyo-financial/sdk-rust` (This Repo) | Submit PR adding tests to `tests/` or unit tests in `src/`. |
| **Documentation, Examples, Guides** | `xyo-financial/sdk-rust` (This Repo) | Submit PR updating `README.md`, `example/`, or `CONTRIBUTING.md`. |
| **Cargo Workspace & Build Tooling** | `xyo-financial/sdk-rust` (This Repo) | Submit PR updating `Cargo.toml`, `Dockerfile`, or CI workflows. |

---

### Workflow A: API & Data Model Changes

1. Fork and clone [`xyo-financial/specs`](https://github.com/xyo-financial/specs).
2. Propose your changes to `openapi.yml` and submit a Pull Request upstream.
3. Once the PR is reviewed, approved, and merged in `xyo-financial/specs`:
   - Pull the updated `openapi.yml` into your local specs workspace.
   - Follow the [Local Code Generation](#3-local-code-generation) instructions to regenerate the `openapi/` layer.
   - Update `src/client.rs`, `src/error.rs`, and `src/lib.rs` to expose the new functionality with idiomatic async interfaces.
   - Add new tests in `tests/client_test.rs` covering the new behavior.
   - Run all [Quality Gates](#4-quality-gates--validation) and submit a PR to this repository.

### Workflow B: SDK Ergonomics, Helpers & Tests

1. Create a descriptive feature branch from `main`:
   ```bash
   git checkout -b feat/client-retry-policy
   ```
2. Implement your enhancements in `src/` or add test coverage in `tests/`.
3. Verify that all quality gates pass locally (`cargo check`, `cargo test`, `cargo clippy --all-targets`).
4. Submit a Pull Request targeting `main` on `https://github.com/xyo-financial/sdk-rust`.

---

## 3. Local Code Generation

When OpenAPI specifications are updated in `xyo-financial/specs`, regenerate the `openapi/` workspace member using the OpenAPI Generator CLI.

### Prerequisites

- **Node.js**: v18+ with `npx`
- **Java Runtime Environment (JRE)**: Version 11 or higher (required by `@openapitools/openapi-generator-cli`)
- **Upstream Specs**: The `xyo-financial/specs` repository cloned adjacent to `sdks/` (e.g. at `../specs/openapi.yml`).

### Generation Command

From the root directory of the Rust SDK (`/Users/hadi/dev/start-ups/xyo/sdks/rust`):

```bash
npx @openapitools/openapi-generator-cli generate \
  -i ../specs/openapi.yml \
  -g rust \
  -o ./openapi \
  --additional-properties=packageName=xyo-sdk,packageVersion=2.0.0
```

### Generator Workspace Configuration

The root `Cargo.toml` manages `openapi` as a workspace member:

```toml
workspace = { members = ["openapi"], exclude = ["example"] }

[dependencies]
openapi-client = { version = "2.0.0", path = "openapi" }
tokio = { version = "=1.38.0", features = ["rt-multi-thread", "macros"] }
```

After regenerating `openapi/`:
1. Verify that `openapi/Cargo.toml` is intact and `openapi-client` compiles.
2. Run `cargo check` and `cargo test` to ensure compatibility between `src/` and `openapi/`.
3. If new models or APIs were introduced, update the wrapper methods in `src/client.rs` and re-exports in `src/lib.rs`.

---

## 4. Quality Gates & Validation

Every contribution must pass all institutional quality gates before being approved or merged. Continuous Integration (CI) enforces these checks on all Pull Requests and release branches.

```
┌──────────────────────────────────────────────────────────────────────────┐
│                          Mandatory Quality Gates                         │
├───────────────────────────────┬──────────────────────────────────────────┤
│ Gate                          │ Command                                  │
├───────────────────────────────┼──────────────────────────────────────────┤
│ 1. Compilation Verification   │ `cargo check`                            │
│ 2. Unit & Integration Testing │ `cargo test`                             │
│ 3. Static Analysis & Linting  │ `cargo clippy --all-targets`             │
└───────────────────────────────┴──────────────────────────────────────────┘
```

### 1. `cargo check` (Compilation Verification)

Verifies that the entire workspace, including the generated `openapi-client` crate and the `xyo-sdk` wrapper crate, compiles cleanly without warnings or missing dependencies.

```bash
cargo check
```

- **Requirement**: Zero compilation errors.

### 2. `cargo test` (Unit & Integration Tests)

Executes all unit tests in `src/`, doc-tests in public documentation, and comprehensive integration tests in `tests/client_test.rs`.

```bash
cargo test
```

- **Requirement**: All unit tests, doc-tests, and mock integration tests must pass with `0 failed; 0 filtered out`.

### 3. `cargo clippy --all-targets` (Static Analysis & Linting)

Runs Clippy across the entire crate surface, including library code, unit tests, integration test binaries, and examples.

```bash
cargo clippy --all-targets
```

- **Requirement**: Must exit with code `0` and produce zero warnings.

---

## 5. Development & Testing Guide

### Running the Test Suite

The test suite includes:
- **Unit Tests**: Located inline in `src/client.rs` and `src/error.rs`, validating error mapping, serialization, and enum deserialization.
- **Integration Tests**: Located in `tests/client_test.rs`, using [`wiremock`](https://crates.io/crates/wiremock) to simulate real HTTP exchanges against the XYO Financial API.
- **Documentation Tests**: Validating all code examples embedded in Rust doc comments (`//!` and `///`).

To run tests with detailed output:

```bash
cargo test -- --nocapture
```

To run a specific test by name:

```bash
cargo test test_enrich_transaction_success -- --nocapture
```

### WireMock Integration Tests

The integration test suite in `tests/client_test.rs` validates:
- HTTP 200 OK single and bulk transaction enrichment workflows.
- HTTP 400 Bad Request, 401 Unauthorized, 404 Not Found, 422 Unprocessable Entity, and 500 Internal Server Error scenarios.
- Request payload shape verification and `Bearer` token authorization header handling.
- `X-API-User` custom header propagation.
- Polling transitions for `EnrichmentStatus` (`Ready`, `Pending`, `Failed`).
- Network-level transport failures and connection drops mapped to `ClientError`.

When adding new SDK features, always add corresponding WireMock test scenarios in `tests/client_test.rs`.

### Docker Development Environment

A standard `Dockerfile` and `Makefile` are provided for containerized development and CI replication:

```bash
# Build the Docker image
make build

# Launch an interactive shell inside the development container
make ssh
```

---

## 6. Pull Request & Commit Standards

### Conventional Commits

We adhere to the [Conventional Commits](https://www.conventionalcommits.org/) specification for institutional auditability:

- `feat(client)`: Add new public client method or configuration option.
- `fix(error)`: Correct error code mapping or error description.
- `refactor(async)`: Optimize tokio async execution or connection pooling.
- `test(enrichment)`: Add WireMock integration test for bulk endpoints.
- `docs(readme)`: Update architectural documentation or code samples.
- `chore(deps)`: Bump dependencies or OpenAPI generator version.

**Example Commit Message**:
```
feat(client): add retry support for rate-limited requests

Introduce exponential backoff retry handler when receiving HTTP 429
responses from the XYO Financial enrichment endpoint.
```

### PR Submission Checklist

Before submitting your Pull Request, ensure that:

- [ ] Changes adhere to the **Two-Layer Architecture** (`openapi/` untouched manually; edits made in `src/`).
- [ ] `cargo check` compiles cleanly without warnings.
- [ ] `cargo test` passes 100% of unit, doc, and integration tests.
- [ ] `cargo clippy --all-targets` passes with zero linter warnings.
- [ ] Public structs, traits, and functions have comprehensive Rust doc comments (`///`).
- [ ] New features or bug fixes include dedicated test coverage in `tests/client_test.rs`.
- [ ] PR description clearly explains the motivation, changes made, and verification steps executed.

---

## 7. Release & Versioning Process

The XYO Financial Rust SDK follows [Semantic Versioning (SemVer)](https://semver.org/):

1. Create a release branch and submit a PR to `main`.
2. Ensure version numbers are incremented consistently in `Cargo.toml`.
3. Verify that CI runs and all quality gates pass green.
4. Merge the release PR into `main`.
5. Tag the release commit:
   ```bash
   git tag v1.x.x
   git push origin v1.x.x
   ```
6. The CI/CD pipeline triggers automated testing, builds the release artifacts, and publishes the crate to [crates.io](https://crates.io/crates/xyo-sdk).
