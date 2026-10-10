mod linux;
mod macos;
mod windows;

/// Native-window integration points for future platform backends.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowBackend {
    /// No native window backend is linked; suitable for headless use.
    Headless,
    /// A native backend can be added by a platform integration crate.
    FutureNative,
}

/// Capability status for the currently selected platform backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformCapability {
    Supported,
    Placeholder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformInfo {
    pub name: &'static str,
    pub backend: WindowBackend,
    pub capability: PlatformCapability,
}

impl PlatformInfo {
    pub const fn current() -> Self {
        if cfg!(target_os = "linux") {
            linux::platform_info()
        } else if cfg!(target_os = "macos") {
            macos::platform_info()
        } else if cfg!(target_os = "windows") {
            windows::platform_info()
        } else {
            Self {
                name: "Unknown",
                backend: WindowBackend::Headless,
                capability: PlatformCapability::Placeholder,
            }
        }
    }
}
