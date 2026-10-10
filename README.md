# Savannah Web Engine

Savannah is a reusable Rust browser foundation with a shared core and compile-gated native adapters.

> This repository now includes a Safari-inspired browser shell model (tabs, address/search, back/forward/reload, loading state, bookmarks, downloads, private-mode indicator) and an extension core with explicit permissions and isolated storage.

## Architecture

- **Shared Rust core (`src/browser.rs`)**
  - Browser/tab state model.
  - Address/search parsing.
  - Back/forward/reload history behavior.
  - Bookmarks/download affordances.
  - Private-mode-aware behavior.
- **Network engine (`src/engine.rs`, `src/loading.rs`)**
  - Async HTTP/HTTPS navigation with timeout/cancellation.
- **Extension core (`src/extensions/*`)**
  - JSON manifest parsing/validation.
  - Explicit permission and origin capability checks.
  - Background/content-script metadata hooks.
  - Messaging bus.
  - Isolated storage (`extension_id` + private-mode boundary).
- **Platform adapters (`src/app/*`)**
  - Linux GTK4 + WebKitGTK adapter contract.
  - Windows WinUI 3 + WebView2 adapter contract.
  - macOS SwiftUI + WKWebView adapter contract.
  - Unsupported targets fail with actionable error messages.

## Safari-inspired shell capabilities (core model)

- Tab strip state and active-tab selection.
- Address/search field behavior (URL or search query).
- Back / Forward / Reload affordances.
- Loading / Loaded / Failed page state.
- Favorites/bookmarks affordance (disabled persistence in private mode).
- Downloads affordance with safe URL scheme validation.
- Private browsing mode indicator/state.

## Extension model

Manifest fields:

- `id`, `name`, `version`
- `permissions` (explicit only; no `<all_urls>`)
- `background.script`
- `content_scripts[].matches` and `content_scripts[].script`

Supported permission kinds:

- `tabs`
- `bookmarks`
- `downloads`
- `messaging`
- `storage`
- `origin` (explicit `http(s)://host/` only)

Sample manifest: `/home/runner/work/savannah/savannah/examples/extensions/minimal_extension.json`

## Native UI strategy and prerequisites

Savannah core compiles headlessly in CI. Native adapters are scaffolded and compile-gated:

- **Linux (GTK4 + WebKitGTK)**
  - Typical packages: `libgtk-4-dev`, WebKitGTK development package for your distro.
- **Windows (WinUI 3 + WebView2)**
  - Windows App SDK, WebView2 Runtime, Rust Windows bindings.
- **macOS (SwiftUI + WKWebView)**
  - Xcode command-line tools / Xcode SDK, Swift bridge integration.

Current adapter `launch()` methods return clear "unavailable" errors until each platform crate wires native bindings.

## Security defaults

- Extension permissions are explicit and validated.
- `<all_urls>` origin permission is rejected.
- Origin matching is allow-list based.
- Extensions receive no unrestricted native access API.
- Storage isolation across extension IDs and private/non-private mode.
- Browser navigation/download URLs are restricted to `http/https`.

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

Run:

```bash
cargo run --example headless_navigation -- https://example.com
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

CI runs these checks on Ubuntu, macOS, and Windows.

## Current limitations

- Native GTK4/WinUI3/SwiftUI window implementations are scaffolded, not fully wired.
- Full rendering/DOM/JavaScript engine integration is still future work.
- Extension execution runtime (actual JS host) is not yet implemented; this release focuses on manifest, permissions, messaging, and storage boundaries in the core.

## Future work

1. Implement real platform window/webview bindings behind the existing adapter traits.
2. Bind browser shell state to native UI controls per platform.
3. Add extension runtime host for content/background script execution.
4. Add profile/cookie/cache/download manager persistence.

## License

Licensed under either Apache License, Version 2.0 or MIT at your option.
