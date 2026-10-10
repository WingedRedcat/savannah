use super::{PlatformCapability, PlatformInfo, WindowBackend};

pub(crate) const fn platform_info() -> PlatformInfo {
    PlatformInfo {
        name: "Linux",
        backend: WindowBackend::Gtk4WebKitGtk,
        capability: PlatformCapability::Placeholder,
    }
}
