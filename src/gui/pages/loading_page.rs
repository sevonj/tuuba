use egui::CentralPanel;

use crate::gui::widgets;

use widgets::LoadingSpinner;

pub struct LoadingPage;

impl LoadingPage {
    pub fn new() -> Self {
        Self
    }

    //pub fn show(&self, ctx: &egui::Context) {
    //    CentralPanel::default()
    //        .frame(super::page_frame(&ctx.style()))
    //        .show(ctx, |ui| {
    //            ui.add(LoadingSpinner);
    //        });
    //}

    pub fn show_inside(&self, ui: &mut egui::Ui) {
        CentralPanel::default()
            .frame(super::page_frame(ui.style()))
            .show_inside(ui, |ui| {
                ui.add(LoadingSpinner);
            });
    }
}
