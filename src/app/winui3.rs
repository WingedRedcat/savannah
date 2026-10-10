use super::{BrowserAppAdapter, PlatformAdapterInfo, PlatformAppError, WebViewRuntime};

#[derive(Default)]
pub struct WinUi3WindowsApp;

impl BrowserAppAdapter for WinUi3WindowsApp {
    fn info(&self) -> PlatformAdapterInfo {
        PlatformAdapterInfo {
            ui_framework: "WinUI 3",
            webview_runtime: WebViewRuntime::WebView2,
            target: "windows",
        }
    }

    fn launch(&self) -> Result<(), PlatformAppError> {
        #[cfg(target_os = "windows")]
        {
            return Err(PlatformAppError::Unavailable(
                "WinUI 3 adapter scaffold is present, but Rust/WinRT + WebView2 wiring is not linked in this crate yet. Install Windows App SDK + WebView2 runtime and implement the adapter in a Windows-specific crate."
                    .to_string(),
            ));
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err(PlatformAppError::Unavailable(
                "WinUI 3 adapter is only available when compiling for Windows targets.".to_string(),
            ))
        }
    }
}
