use egui::{Frame, Label, RichText, Sense, UiBuilder, Vec2, Widget};

use super::{CommunityIdenfifier, PersonIdentifier, Thumbnail};

const COMPACT_H: f32 = 96.;
const THUMB_W: f32 = 96.;
const THUMB_H: f32 = 64.;
const AVATAR_SIZE: f32 = 24.;

pub struct PostCompact<'a> {
    post: &'a PostView,
}

impl<'a> PostCompact<'a> {
    pub fn new(post: &'a PostView) -> PostCompact<'a> {
        Self { post }
    }
}

impl Widget for PostCompact<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.scope_builder(
            UiBuilder::new().id_salt("post_cont").sense(Sense::click()),
            |ui| {
                Frame::group(ui.style())
                    .show(ui, |ui| {
                        let avail_w = ui.available_width();
                        ui.set_width(avail_w);
                        ui.set_height(COMPACT_H);
                        ui.set_max_height(COMPACT_H);

                        ui.horizontal(|ui| {
                            Frame::default().inner_margin(0.).show(ui, |ui| {
                                ui.set_width(THUMB_W);
                                ui.set_height(THUMB_H);
                                if let Some(url) = &self.post.post.thumbnail_url {
                                    ui.add(Thumbnail::from_uri(url.as_str().into()))
                                        .on_hover_text(url.as_str());
                                }
                            });

                            ui.vertical(|ui| {
                                ui.add(
                                    Label::new(
                                        RichText::new(&self.post.post.name).heading().strong(),
                                    )
                                    .truncate()
                                    .selectable(false),
                                );
                                if let Some(body) = &self.post.post.body {
                                    ui.add(Label::new(body).truncate().selectable(false));
                                } else {
                                    ui.add(Label::new("").truncate().selectable(false));
                                }

                                ui.horizontal(|ui| {
                                    Frame::group(ui.style()).show(ui, |ui| {
                                        ui.set_width(AVATAR_SIZE);
                                        ui.set_height(AVATAR_SIZE);
                                    });
                                    Frame::group(ui.style()).show(ui, |ui| {
                                        ui.set_width(AVATAR_SIZE);
                                        ui.set_height(AVATAR_SIZE);
                                    });
                                    ui.vertical(|ui| {
                                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                                        ui.add(PersonIdentifier::new(&self.post.creator));
                                        ui.horizontal(|ui| {
                                            ui.monospace("to ");
                                            ui.add(CommunityIdenfifier::new(&self.post.community));
                                        });
                                    });
                                });
                            })
                        })
                    })
                    .response
            },
        )
        .response
    }
}
