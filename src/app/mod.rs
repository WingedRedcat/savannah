mod gtk4;
mod swiftui;
mod winui3;

pub use gtk4::Gtk4LinuxApp;
pub use swiftui::SwiftUiMacApp;
pub use winui3::WinUi3WindowsApp;

/// High-level browser shell controls for native adapters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrowserCommand {
    Back,
    Forward,
    Reload,
    NewTab,
    OpenDownloads,
    TogglePrivateMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebViewRuntime {
    WebKitGtk,
    WebView2,
    WkWebView,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformAdapterInfo {
    pub ui_framework: &'static str,
    pub webview_runtime: WebViewRuntime,
    pub target: &'static str,
}

pub trait BrowserAppAdapter {
    fn info(&self) -> PlatformAdapterInfo;
    fn launch(&self) -> Result<(), PlatformAppError>;
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformAppError {
    #[error("platform adapter unavailable: {0}")]
    Unavailable(String),
}
