use std::borrow::Cow;

use egui::{Image, ImageSource, Widget};

pub struct Thumbnail<'a>(ImageSource<'a>);

impl<'a> Thumbnail<'a> {
    pub fn from_uri(uri: Cow<'a, str>) -> Thumbnail<'a> {
        Self(ImageSource::Uri(uri))
    }
}

impl Widget for Thumbnail<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(Image::new(self.0))
    }
}
