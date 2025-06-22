use egui_extras::install_image_loaders;

use crate::client::Client;
use crate::gui::Gui;

pub struct Tuubapp {
    data: Client,
}

impl Default for Tuubapp {
    fn default() -> Self {
        let mut this = Self {
            data: Client::default(),
        };

        this.data.fetch_instance_config();
        //this.data.reload_feed();

        this
    }
}

impl Tuubapp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        video_rs::init().unwrap();

        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for Tuubapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        install_image_loaders(ctx);
        //if let Some(crate::client::Page::Video(_, Some(playback))) =
        //    self.data.page().lock().as_mut().unwrap().as_mut()
        //{
        //    playback.update();
        //}

        Gui::new(&mut self.data).show(ctx);

        // TODO: repaint on request complete
        ctx.request_repaint();
    }
}
