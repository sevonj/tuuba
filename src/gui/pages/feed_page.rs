use egui::CentralPanel;
use egui::TopBottomPanel;

use crate::gui::ClientCommand;
use crate::gui::widgets;

use widgets::PostCompact;

pub struct FeedPage<'a> {
    cmd: &'a mut ClientCommand,
    //feed: &'a FeedData,
}

impl<'a> FeedPage<'a> {
    pub fn new(cmd: &'a mut ClientCommand) -> FeedPage<'a> {
        Self { cmd }
    }
}

impl FeedPage<'_> {
    pub fn show(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("postview_top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("back").clicked() {
                    *self.cmd = Some(ClientCommand::ClosePage);
                }
            });
        });

        CentralPanel::default()
            .frame(super::page_frame(&ctx.style()))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    egui::ScrollArea::vertical()
                        .show(ui, |ui| {
                            //for post in &self.feed.posts {
                            //    if ui.add(PostCompact::new(post)).clicked() {
                            //        *self.cmd = Some(ClientCommand::ShowPost(post.post.id));
                            //    };
                            //}
                        })
                        .inner
                });
            });
    }
}
