# Simple Math Library

This is a sample rust library crate with:

* `simple_math` library
* `main` binary (set as default for `cargo run`)
* `cli` binary
* a single integration test with 3 modules

Please note that integration tests does not work for binary crate per [documentation](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates).

We also have:

* `doc/architecture/decisions` directory to keep track of [Architecture Decision Records](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions) created with [ADR Tools](https://github.com/npryce/adr-tools).
* `.adr-dir` file to tell ADR Tools where to put each of decision records into.

## How to run test

```bash
cargo test
```

## How to build everything

```bash
cargo build --all-targets
```

## How to create package's documentation

Also open the document with default web browser.

```bash
cargo doc --document-private-items --open
```
