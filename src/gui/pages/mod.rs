//mod feed_page;
mod loading_page;
mod video_page;

use egui::Frame;
//pub use feed_page::FeedPage;
pub use loading_page::LoadingPage;
pub use video_page::VideoPage;

use crate::gui::style::ColorScheme;

fn page_frame(style: &egui::Style) -> Frame {
    let color_scheme = ColorScheme::default();
    Frame::central_panel(style).fill(color_scheme.bg2)
}
