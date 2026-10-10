use savannah::{BrowserError, BrowserState, NavigationRequest, PageLoadState};
use url::Url;

#[test]
fn address_field_resolves_urls_and_search_queries() {
    let state = BrowserState::new(false);

    let direct = state.parse_address_input("https://example.com").unwrap();
    assert!(matches!(direct, NavigationRequest::Navigate(_)));

    let host = state.parse_address_input("rust-lang.org").unwrap();
    assert!(matches!(host, NavigationRequest::Navigate(_)));

    let search = state.parse_address_input("rust browser engine").unwrap();
    assert!(matches!(search, NavigationRequest::Search { .. }));
}

#[test]
fn tab_history_supports_back_forward_and_reload() {
    let mut state = BrowserState::new(false);
    let tab = state.active_tab_index();

    state.begin_navigation(tab).unwrap();
    state
        .finish_navigation(tab, Url::parse("https://example.com").unwrap(), None)
        .unwrap();
    state
        .finish_navigation(tab, Url::parse("https://example.com/docs").unwrap(), None)
        .unwrap();

    assert_eq!(state.active_tab().load_state, PageLoadState::Loaded);
    assert!(state.active_tab().can_go_back());
    assert!(!state.active_tab().can_go_forward());

    let previous = state.navigate_back(tab).unwrap().unwrap();
    assert_eq!(previous.as_str(), "https://example.com/");

    let next = state.navigate_forward(tab).unwrap().unwrap();
    assert_eq!(next.as_str(), "https://example.com/docs");

    let reload = state.reload_url(tab).unwrap().unwrap();
    assert_eq!(reload.as_str(), "https://example.com/docs");
}

#[test]
fn private_mode_blocks_bookmark_persistence() {
    let mut private_state = BrowserState::new(true);
    let tab = private_state.active_tab_index();
    private_state
        .finish_navigation(tab, Url::parse("https://example.com").unwrap(), None)
        .unwrap();

    let bookmark = private_state.add_bookmark_for_active_tab().unwrap();
    assert!(bookmark.is_none());
    assert!(private_state.bookmarks().is_empty());
}

#[test]
fn blocks_unsafe_download_urls() {
    let mut state = BrowserState::new(false);
    let result = state.queue_download("file:///tmp/secret");
    assert!(matches!(result, Err(BrowserError::UnsafeDownloadUrl(_))));
}
