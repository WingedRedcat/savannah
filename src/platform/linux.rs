use super::{PlatformCapability, PlatformInfo, WindowBackend};

pub(crate) const fn platform_info() -> PlatformInfo {
    PlatformInfo {
        name: "Linux",
        backend: WindowBackend::Headless,
        capability: PlatformCapability::Placeholder,
    }
}
