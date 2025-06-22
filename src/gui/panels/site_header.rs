use crate::client::ArcMutexOpt;
use crate::client::GuiState;
use crate::client::api;
use egui::TopBottomPanel;

use api::instance::Config;

const HEADER_H: f32 = 64.0;

pub struct SiteHeader<'a> {
    gui_state: &'a mut GuiState,
    config: ArcMutexOpt<Config>,
}

impl<'a> SiteHeader<'a> {
    pub fn new(gui_state: &'a mut GuiState, config: ArcMutexOpt<Config>) -> SiteHeader<'a> {
        Self { gui_state, config }
    }
}

impl SiteHeader<'_> {
    pub fn show(&mut self, ctx: &egui::Context) {
        let name;

        let config_guard = self.config.lock().unwrap();
        if let Some(config) = config_guard.as_ref() {
            name = config.instance().name();
        } else {
            name = "Loading...";
        }

        TopBottomPanel::top("app_header")
            .exact_height(HEADER_H)
            .show_separator_line(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("☰").clicked() {
                        self.gui_state.toggle_sidebar_expand();
                    }

                    ui.heading(name);
                });
            });
    }
}
