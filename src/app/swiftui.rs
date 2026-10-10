use super::{BrowserAppAdapter, PlatformAdapterInfo, PlatformAppError, WebViewRuntime};

#[derive(Default)]
pub struct SwiftUiMacApp;

impl BrowserAppAdapter for SwiftUiMacApp {
    fn info(&self) -> PlatformAdapterInfo {
        PlatformAdapterInfo {
            ui_framework: "SwiftUI",
            webview_runtime: WebViewRuntime::WkWebView,
            target: "macos",
        }
    }

    fn launch(&self) -> Result<(), PlatformAppError> {
        #[cfg(target_os = "macos")]
        {
            return Err(PlatformAppError::Unavailable(
                "SwiftUI adapter scaffold is present, but SwiftUI/WKWebView bridging is not compiled in this crate yet. Install Xcode command-line tools and add a macOS bridge crate for SwiftUI integration."
                    .to_string(),
            ));
        }

        #[cfg(not(target_os = "macos"))]
        {
            Err(PlatformAppError::Unavailable(
                "SwiftUI adapter is only available when compiling for macOS targets.".to_string(),
            ))
        }
    }
}
