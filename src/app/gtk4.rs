use super::{BrowserAppAdapter, PlatformAdapterInfo, PlatformAppError, WebViewRuntime};

#[derive(Default)]
pub struct Gtk4LinuxApp;

impl BrowserAppAdapter for Gtk4LinuxApp {
    fn info(&self) -> PlatformAdapterInfo {
        PlatformAdapterInfo {
            ui_framework: "GTK4",
            webview_runtime: WebViewRuntime::WebKitGtk,
            target: "linux",
        }
    }

    fn launch(&self) -> Result<(), PlatformAppError> {
        #[cfg(target_os = "linux")]
        {
            return Err(PlatformAppError::Unavailable(
                "GTK4 adapter scaffold is present, but native GTK4/WebKitGTK bindings are not wired in this crate yet. Install GTK4 + WebKitGTK development packages and integrate your chosen GTK/WebKit bindings in a platform crate."
                    .to_string(),
            ));
        }

        #[cfg(not(target_os = "linux"))]
        {
            Err(PlatformAppError::Unavailable(
                "GTK4 adapter is only available when compiling for Linux targets.".to_string(),
            ))
        }
    }
}
