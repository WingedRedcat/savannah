use savannah::extensions::{ExtensionManager, ExtensionManifest, ExtensionPermission};
use url::Url;

fn sample_manifest(id: &str, with_storage: bool) -> String {
    let storage = if with_storage {
        r#", {"kind":"storage"}"#
    } else {
        ""
    };

    format!(
        r#"{{
            "id":"{id}",
            "name":"{id}",
            "version":"1.0.0",
            "permissions":[
                {{"kind":"messaging"}}{storage},
                {{"kind":"origin","value":"https://example.com/"}}
            ],
            "background":{{"script":"background.js"}},
            "content_scripts":[{{"matches":["https://example.com/"],"script":"content.js"}}]
        }}"#
    )
}

#[test]
fn parses_and_validates_manifest_permissions() {
    let manifest = ExtensionManifest::parse(&sample_manifest("ext.alpha", true)).unwrap();
    assert!(manifest
        .permissions
        .contains(&ExtensionPermission::Messaging));
}

#[test]
fn rejects_overbroad_origin_permission() {
    let json = r#"{
        "id":"ext.bad",
        "name":"Bad",
        "version":"1.0.0",
        "permissions":[{"kind":"origin","value":"<all_urls>"}],
        "content_scripts":[{"matches":["https://example.com/"],"script":"content.js"}],
        "background":{"script":"background.js"}
    }"#;

    assert!(ExtensionManifest::parse(json).is_err());
}

#[test]
fn messaging_requires_permission_and_routes_payload() {
    let mut manager = ExtensionManager::new();
    manager
        .register_manifest_json(&sample_manifest("ext.sender", true))
        .unwrap();
    manager
        .register_manifest_json(&sample_manifest("ext.receiver", true))
        .unwrap();

    manager
        .send_message("ext.sender", "ext.receiver", "ping", "hello")
        .unwrap();
    let messages = manager.take_messages("ext.receiver");

    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].payload, "hello");
}

#[test]
fn storage_isolated_by_extension_and_private_mode() {
    let mut manager = ExtensionManager::new();
    manager
        .register_manifest_json(&sample_manifest("ext.storage", true))
        .unwrap();

    manager
        .storage_set("ext.storage", "token", "normal".to_string(), false)
        .unwrap();
    manager
        .storage_set("ext.storage", "token", "private".to_string(), true)
        .unwrap();

    let normal = manager
        .storage_get("ext.storage", "token", false)
        .unwrap()
        .unwrap();
    let private = manager
        .storage_get("ext.storage", "token", true)
        .unwrap()
        .unwrap();

    assert_eq!(normal, "normal");
    assert_eq!(private, "private");
}

#[test]
fn origin_validation_requires_explicit_match() {
    let mut manager = ExtensionManager::new();
    manager
        .register_manifest_json(&sample_manifest("ext.origin", false))
        .unwrap();

    let allowed = manager
        .can_access_origin(
            "ext.origin",
            &Url::parse("https://example.com/docs").unwrap(),
        )
        .unwrap();
    let denied = manager
        .can_access_origin("ext.origin", &Url::parse("https://example.org").unwrap())
        .unwrap();

    assert!(allowed);
    assert!(!denied);
}
