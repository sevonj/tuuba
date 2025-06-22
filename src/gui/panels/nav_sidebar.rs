use egui::{ScrollArea, SidePanel};

use crate::gui::{ClientCommand, widgets::NavButton};

const SIDEBAR_W: f32 = 256.0;

pub struct NavSidebar<'a> {
    cmd: &'a mut ClientCommand,
}

impl<'a> NavSidebar<'a> {
    pub fn new(cmd: &'a mut ClientCommand) -> NavSidebar<'a> {
        Self { cmd }
    }
}

impl NavSidebar<'_> {
    pub fn show(self, ctx: &egui::Context) {
        SidePanel::left("nav_sidebar")
            .exact_width(SIDEBAR_W)
            .resizable(false)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    if ui.add(NavButton::new("Home")).clicked() {
                        *self.cmd = ClientCommand::ClosePage;
                    }
                    let _ = ui.add(NavButton::new("Subscriptions"));
                    ui.separator();
                    let _ = ui.add(NavButton::new("Profile"));
                    let _ = ui.add(NavButton::new("Playlists"));
                    let _ = ui.add(NavButton::new("History"));
                    let _ = ui.add(NavButton::new("Watch later"));
                    let _ = ui.add(NavButton::new("Liked videos"));
                    ui.separator();
                    let _ = ui.add(NavButton::new("Chatrooms"));
                    let _ = ui.add(NavButton::new("Channels"));
                    let _ = ui.add(NavButton::new("Videos"));
                    let _ = ui.add(NavButton::new("Upload"));
                    ui.separator();
                    let _ = ui.add(NavButton::new("Settings"));
                    let _ = ui.add(NavButton::new("Instance info"));
                });
            });
    }
}
