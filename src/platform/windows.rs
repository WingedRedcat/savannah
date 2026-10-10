use super::{PlatformCapability, PlatformInfo, WindowBackend};

pub(crate) const fn platform_info() -> PlatformInfo {
    PlatformInfo {
        name: "Windows",
        backend: WindowBackend::WinUi3WebView2,
        capability: PlatformCapability::Placeholder,
    }
}
