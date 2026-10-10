use super::{PlatformCapability, PlatformInfo, WindowBackend};

pub(crate) const fn platform_info() -> PlatformInfo {
    PlatformInfo {
        name: "macOS",
        backend: WindowBackend::SwiftUiWkWebView,
        capability: PlatformCapability::Placeholder,
    }
}
