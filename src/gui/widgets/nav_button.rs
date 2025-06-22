use egui::{Button, Color32, RichText, Widget, vec2};

const BUTTON_H: f32 = 32.0;

pub struct NavButton<'a> {
    text: &'a str,
}

impl<'a> NavButton<'a> {
    pub fn new(text: &'a str) -> NavButton<'a> {
        Self { text }
    }
}

impl Widget for NavButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

        let min_size = vec2(ui.available_width(), BUTTON_H);
        let text = RichText::new(self.text).size(18.0);
        let button = Button::new(text).min_size(min_size).corner_radius(6.0);

        ui.add(button)
    }
}
