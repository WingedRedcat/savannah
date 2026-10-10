use std::fmt;

use url::Url;

/// One browser tab in the shared shell model.
#[derive(Clone, Debug)]
pub struct TabState {
    pub id: u64,
    pub title: String,
    pub load_state: PageLoadState,
    pub current_url: Option<Url>,
    history: Vec<Url>,
    history_index: Option<usize>,
}

impl TabState {
    fn new(id: u64) -> Self {
        Self {
            id,
            title: "New Tab".to_string(),
            load_state: PageLoadState::Idle,
            current_url: None,
            history: Vec::new(),
            history_index: None,
        }
    }

    pub fn can_go_back(&self) -> bool {
        matches!(self.history_index, Some(index) if index > 0)
    }

    pub fn can_go_forward(&self) -> bool {
        matches!(self.history_index, Some(index) if index + 1 < self.history.len())
    }

    fn push_history(&mut self, url: Url) {
        if let Some(index) = self.history_index {
            self.history.truncate(index + 1);
        }
        self.history.push(url.clone());
        self.history_index = Some(self.history.len() - 1);
        self.current_url = Some(url);
    }

    fn navigate_history(&mut self, direction: HistoryDirection) -> Option<Url> {
        let current = self.history_index?;
        let next = match direction {
            HistoryDirection::Back if current > 0 => current - 1,
            HistoryDirection::Forward if current + 1 < self.history.len() => current + 1,
            _ => return None,
        };
        let url = self.history.get(next)?.clone();
        self.history_index = Some(next);
        self.current_url = Some(url.clone());
        Some(url)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageLoadState {
    Idle,
    Loading,
    Loaded,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HistoryDirection {
    Back,
    Forward,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bookmark {
    pub title: String,
    pub url: Url,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DownloadRequest {
    pub url: Url,
    pub private_mode: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NavigationRequest {
    Navigate(Url),
    Search { query: String, url: Url },
}

#[derive(Clone, Debug)]
pub struct BrowserState {
    tabs: Vec<TabState>,
    active_tab: usize,
    next_tab_id: u64,
    bookmarks: Vec<Bookmark>,
    downloads: Vec<DownloadRequest>,
    private_mode: bool,
    search_url: Url,
}

impl BrowserState {
    pub fn new(private_mode: bool) -> Self {
        let mut state = Self {
            tabs: Vec::new(),
            active_tab: 0,
            next_tab_id: 1,
            bookmarks: Vec::new(),
            downloads: Vec::new(),
            private_mode,
            search_url: Url::parse("https://duckduckgo.com/")
                .expect("default search endpoint is valid"),
        };
        state.open_tab();
        state
    }

    pub fn private_mode(&self) -> bool {
        self.private_mode
    }

    pub fn tabs(&self) -> &[TabState] {
        &self.tabs
    }

    pub fn active_tab_index(&self) -> usize {
        self.active_tab
    }

    pub fn active_tab(&self) -> &TabState {
        &self.tabs[self.active_tab]
    }

    pub fn open_tab(&mut self) -> u64 {
        let id = self.next_tab_id;
        self.next_tab_id += 1;
        self.tabs.push(TabState::new(id));
        self.active_tab = self.tabs.len() - 1;
        id
    }

    pub fn close_tab(&mut self, tab_index: usize) -> Result<(), BrowserError> {
        if self.tabs.len() == 1 {
            return Err(BrowserError::CannotCloseLastTab);
        }
        let _ = self
            .tabs
            .get(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        self.tabs.remove(tab_index);
        self.active_tab = self.active_tab.min(self.tabs.len() - 1);
        Ok(())
    }

    pub fn activate_tab(&mut self, tab_index: usize) -> Result<(), BrowserError> {
        let _ = self
            .tabs
            .get(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        self.active_tab = tab_index;
        Ok(())
    }

    pub fn parse_address_input(&self, input: &str) -> Result<NavigationRequest, BrowserError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(BrowserError::EmptyAddressInput);
        }

        if let Ok(url) = Url::parse(trimmed) {
            return validate_browsable_url(url).map(NavigationRequest::Navigate);
        }

        if looks_like_host(trimmed) {
            let url = Url::parse(&format!("https://{trimmed}"))
                .map_err(|error| BrowserError::InvalidInput(error.to_string()))?;
            return Ok(NavigationRequest::Navigate(url));
        }

        let mut url = self.search_url.clone();
        url.query_pairs_mut().append_pair("q", trimmed);
        Ok(NavigationRequest::Search {
            query: trimmed.to_string(),
            url,
        })
    }

    pub fn begin_navigation(&mut self, tab_index: usize) -> Result<(), BrowserError> {
        let tab = self
            .tabs
            .get_mut(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        tab.load_state = PageLoadState::Loading;
        Ok(())
    }

    pub fn finish_navigation(
        &mut self,
        tab_index: usize,
        url: Url,
        title: Option<String>,
    ) -> Result<(), BrowserError> {
        validate_browsable_url(url.clone())?;
        let tab = self
            .tabs
            .get_mut(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        tab.push_history(url.clone());
        tab.load_state = PageLoadState::Loaded;
        tab.title = title.unwrap_or_else(|| default_title(&url));
        Ok(())
    }

    pub fn fail_navigation(&mut self, tab_index: usize) -> Result<(), BrowserError> {
        let tab = self
            .tabs
            .get_mut(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        tab.load_state = PageLoadState::Failed;
        Ok(())
    }

    pub fn navigate_back(&mut self, tab_index: usize) -> Result<Option<Url>, BrowserError> {
        let tab = self
            .tabs
            .get_mut(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        Ok(tab.navigate_history(HistoryDirection::Back))
    }

    pub fn navigate_forward(&mut self, tab_index: usize) -> Result<Option<Url>, BrowserError> {
        let tab = self
            .tabs
            .get_mut(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        Ok(tab.navigate_history(HistoryDirection::Forward))
    }

    pub fn reload_url(&self, tab_index: usize) -> Result<Option<Url>, BrowserError> {
        let tab = self
            .tabs
            .get(tab_index)
            .ok_or(BrowserError::UnknownTab(tab_index))?;
        Ok(tab.current_url.clone())
    }

    pub fn add_bookmark_for_active_tab(&mut self) -> Result<Option<Bookmark>, BrowserError> {
        if self.private_mode {
            return Ok(None);
        }

        let tab = self.active_tab();
        let url = tab
            .current_url
            .clone()
            .ok_or(BrowserError::MissingCurrentPage)?;
        let bookmark = Bookmark {
            title: tab.title.clone(),
            url,
        };
        self.bookmarks.push(bookmark.clone());
        Ok(Some(bookmark))
    }

    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    pub fn queue_download(&mut self, url_input: &str) -> Result<DownloadRequest, BrowserError> {
        let url =
            Url::parse(url_input).map_err(|error| BrowserError::InvalidInput(error.to_string()))?;
        validate_download_url(url.clone())?;

        let request = DownloadRequest {
            url,
            private_mode: self.private_mode,
        };
        self.downloads.push(request.clone());
        Ok(request)
    }

    pub fn downloads(&self) -> &[DownloadRequest] {
        &self.downloads
    }
}

fn looks_like_host(input: &str) -> bool {
    !input.contains(' ') && input.contains('.') && !input.contains('/')
}

fn validate_browsable_url(url: Url) -> Result<Url, BrowserError> {
    match url.scheme() {
        "http" | "https" => Ok(url),
        scheme => Err(BrowserError::BlockedScheme(scheme.to_string())),
    }
}

fn validate_download_url(url: Url) -> Result<Url, BrowserError> {
    match url.scheme() {
        "http" | "https" => Ok(url),
        scheme => Err(BrowserError::UnsafeDownloadUrl(scheme.to_string())),
    }
}

fn default_title(url: &Url) -> String {
    url.host_str()
        .map(str::to_string)
        .filter(|host| !host.is_empty())
        .unwrap_or_else(|| "Untitled".to_string())
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BrowserError {
    EmptyAddressInput,
    InvalidInput(String),
    BlockedScheme(String),
    UnsafeDownloadUrl(String),
    UnknownTab(usize),
    MissingCurrentPage,
    CannotCloseLastTab,
}

impl fmt::Display for BrowserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrowserError::EmptyAddressInput => write!(f, "address/search field cannot be empty"),
            BrowserError::InvalidInput(message) => write!(f, "invalid address input: {message}"),
            BrowserError::BlockedScheme(scheme) => {
                write!(f, "blocked non-web scheme for navigation: {scheme}")
            }
            BrowserError::UnsafeDownloadUrl(scheme) => {
                write!(f, "blocked unsafe download URL scheme: {scheme}")
            }
            BrowserError::UnknownTab(index) => write!(f, "unknown tab index: {index}"),
            BrowserError::MissingCurrentPage => write!(f, "active tab has no current page"),
            BrowserError::CannotCloseLastTab => write!(f, "cannot close the last tab"),
        }
    }
}

impl std::error::Error for BrowserError {}
