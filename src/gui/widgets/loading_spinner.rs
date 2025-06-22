use egui::{Frame, Widget};

pub struct LoadingSpinner;

impl Widget for LoadingSpinner {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        Frame::new()
            .show(ui, |ui| {
                Frame::default().show(ui, |ui| {
                    ui.label("loading");
                })
            })
            .response
    }
}
