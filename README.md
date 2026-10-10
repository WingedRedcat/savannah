# Savannah Web Engine

Savannah is a reusable Rust foundation for building fast, cross-platform web browsers and browser-powered applications on Linux, macOS, and Windows.

> Savannah is currently an engine foundation, not a complete browser. HTML/CSS layout, JavaScript execution, GPU rendering, and native windows are intentionally exposed as future integration points.

## Features

- Async, non-blocking navigation and resource loading.
- Typed configuration and errors.
- Request timeouts and cancellation.
- HTTP and HTTPS URL validation.
- Platform capability reporting for Linux, macOS, and Windows.
- A small public API suitable for future browser shells.
- No display server or GUI dependency in the core crate, making CI and headless use straightforward.

## Quick start

```rust
use savannah::{Engine, EngineConfig};

#[tokio::main]
async fn main() -> Result<(), savannah::Error> {
    let engine = Engine::new(EngineConfig::default())?;
    let page = engine.navigate("https://example.com").await?;
    println!("{} bytes from {}", page.body.len(), page.url);
    engine.shutdown().await;
    Ok(())
}
```

Run the example with:

```bash
cargo run --example headless_navigation -- https://example.com
```

## Architecture

- `Engine` owns shared runtime services and the HTTP client.
- `Navigation` models one cancellable load operation.
- `platform` modules (`linux`, `macos`, `windows`) report compile-time capabilities and explicit placeholder behavior for native backends.
- Future crates can implement rendering, DOM, JavaScript, storage, and window backends against these stable boundaries.

The loading path uses `reqwest` with Rustls and HTTP/2, and never performs network I/O on the caller's synchronous thread. Keep parsing and rendering in separate pipeline stages so network completion can be streamed into future consumers.

## Dependency choices

- `tokio`: stable async runtime for non-blocking execution and timeout control.
- `reqwest` (`rustls-tls`, `http2`): mature HTTP client with cross-platform TLS and protocol support.
- `url`: robust URL parsing and scheme validation.
- `tokio-util`: cancellation token support for explicit navigation cancellation.
- `thiserror`: typed, maintainable error definitions.

These dependencies are conservative and widely used in production Rust projects.

## Supported targets

The core crate is designed to compile on:

- Linux (`x86_64`, `aarch64`)
- macOS (`x86_64`, `aarch64`)
- Windows (`x86_64`, `aarch64` where the Rust toolchain supports it)

Native windowing and GPU backends are not included yet. The platform module provides explicit capability reporting and is safe to use in headless applications.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Linux/macOS/Windows CI is configured in GitHub Actions and avoids GUI/display requirements.

## Current limitations

- No DOM, CSS, JavaScript runtime, or rendering pipeline yet.
- No browser storage/cookie/profile system yet.
- Native window backends are placeholders that intentionally report capability status.

## Roadmap

1. Streaming response and resource scheduling.
2. HTML tokenizer, DOM, and CSS parsing crates.
3. Incremental layout and paint trees.
4. JavaScript runtime integration behind a feature flag.
5. GPU compositor and native window backends.
6. Browser profile, cache, cookies, permissions, and developer tooling.

## License

Licensed under either Apache License, Version 2.0 or MIT at your option.
