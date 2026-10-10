use std::time::Duration;

use savannah::{Engine, EngineConfig, Error, PlatformCapability, PlatformInfo, WindowBackend};

#[test]
fn reports_a_supported_compile_target() {
    let info = PlatformInfo::current();
    assert!(!info.name.is_empty());
    assert_eq!(info.capability, PlatformCapability::Placeholder);
    if cfg!(target_os = "linux") {
        assert_eq!(info.backend, WindowBackend::Gtk4WebKitGtk);
    } else if cfg!(target_os = "macos") {
        assert_eq!(info.backend, WindowBackend::SwiftUiWkWebView);
    } else if cfg!(target_os = "windows") {
        assert_eq!(info.backend, WindowBackend::WinUi3WebView2);
    } else {
        assert_eq!(info.backend, WindowBackend::Headless);
    }
}

#[test]
fn rejects_invalid_configuration() {
    let config = EngineConfig {
        max_response_bytes: 0,
        ..EngineConfig::default()
    };
    assert!(matches!(
        Engine::new(config),
        Err(Error::InvalidConfiguration(_))
    ));
}

#[test]
fn validates_navigation_schemes() {
    let engine = Engine::new(EngineConfig::default()).unwrap();
    assert!(matches!(
        engine.navigation("file:///tmp/index.html"),
        Err(Error::UnsupportedScheme(_))
    ));
    assert!(engine.navigation("https://example.com").is_ok());
}

#[tokio::test]
async fn navigation_cancellation_is_honored() {
    let engine = Engine::new(EngineConfig::default()).unwrap();
    let navigation = engine.navigation("https://example.com").unwrap();
    navigation.cancel();
    let result = navigation.run().await;
    assert!(matches!(result, Err(Error::Cancelled)));
}

#[tokio::test]
async fn navigation_times_out_for_slow_server() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;
        use tokio::io::AsyncWriteExt;
        let _ = stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok")
            .await;
    });

    let config = EngineConfig {
        navigation_timeout: Duration::from_millis(20),
        ..EngineConfig::default()
    };
    let engine = Engine::new(config).unwrap();

    let result = engine.navigate(&format!("http://{}", address)).await;
    assert!(matches!(result, Err(Error::Timeout(_))));
}
