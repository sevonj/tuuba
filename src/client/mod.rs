pub mod api;
mod page;
mod playback_data;

use std::sync::{Arc, Mutex};

use api::Api;
pub use page::Page;
pub use playback_data::VideoPlaybackData;

pub type ArcMutexOpt<T> = Arc<Mutex<Option<T>>>;

#[derive(Debug)]
pub struct GuiState {
    pub expand_dong: bool,
}

impl Default for GuiState {
    fn default() -> Self {
        Self { expand_dong: true }
    }
}

impl GuiState {
    pub fn toggle_sidebar_expand(&mut self) {
        self.expand_dong = !self.expand_dong;
    }
}

pub struct Client {
    instance_config: ArcMutexOpt<api::instance::Config>,
    // site: Arc<Mutex<Option<GetSiteResponse>>>,
    // /// Feed is always kept loaded, so it can be returned to.
    // feed: Feed,
    /// Shown instead of feed, if Some.
    page: Option<Page>,
    pub playback: ArcMutexOpt<VideoPlaybackData>,
    pub gui_state: GuiState,
}

impl Default for Client {
    fn default() -> Self {
        let mut this = Self {
            instance_config: Arc::new(Mutex::new(None)),
            page: None,
            playback: Arc::new(Mutex::new(None)),
            gui_state: GuiState::default(),
        };

        this.fetch_instance_config();
        this.open_video(4811);

        this
    }
}

impl Client {
    fn api(&self) -> Api {
        Api::new("https://peertube.wtf")
    }

    pub fn instance_config(&self) -> &ArcMutexOpt<api::instance::Config> {
        &self.instance_config
    }

    //pub fn feed(&self) -> &Feed {
    //    &self.feed
    //}

    pub fn page(&self) -> Option<&Page> {
        self.page.as_ref()
    }

    pub fn fetch_instance_config(&mut self) {
        let config = self.instance_config.clone();
        let api = self.api();
        tokio::spawn(async move {
            match api::instance::Config::get(api).await {
                Ok(value) => {
                    config.lock().as_mut().unwrap().replace(value);
                }
                Err(e) => println!("{e}"),
            }
        });
    }

    pub fn open_video(&mut self, id: usize) {
        self.playback.lock().as_mut().unwrap().take();

        let api = self.api();

        let mut page = Page::video(id, self.playback.clone());
        page.fetch_data(api);
        self.page.replace(page);

        /*tokio::spawn(async move {

        }); */
    }

    pub fn play_open_video(&mut self) {}

    //
    // pub fn reload_feed(&mut self) {
    //     self.feed = Feed::default();
    //     self.feed.fetch_more_posts();
    // }
    //
    pub fn close_page(&mut self) {
        self.page = None;
    }
    //
    // pub fn show_post(&mut self, id: PostId) {
    //     let post_data = PostData::from(id);
    //     self.page = Some(Page::Post(Box::new(post_data)));
    // }
}
