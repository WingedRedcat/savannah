//! Savannah Web Engine public API.
//!
//! This crate provides the reusable, headless foundation of Savannah. Rendering,
//! DOM, JavaScript, and native windows can be layered on top without changing
//! the core navigation API.

pub mod app;
mod browser;
mod config;
mod engine;
mod error;
pub mod extensions;
mod loading;
pub mod platform;

pub use browser::{
    Bookmark, BrowserError, BrowserState, DownloadRequest, NavigationRequest, PageLoadState,
    TabState,
};
pub use config::EngineConfig;
pub use engine::Engine;
pub use error::{Error, Result};
pub use loading::{LoadedPage, Navigation};
pub use platform::{PlatformCapability, PlatformInfo, WindowBackend};
