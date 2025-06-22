use egui::{
    Color32, FontFamily, FontId, Frame, Image, Label, RichText, Sense, TextFormat, UiBuilder,
    Widget, text::LayoutJob,
};
use egui_extras::{Column, TableBuilder};

use crate::{client::api::types::Video, gui::ClientCommand};

const MIN_W: f32 = 400.0;
const ROW_H: f32 = 96.0;

pub struct VideosList<'a> {
    data: &'a Vec<Video>,
    cmd: &'a mut ClientCommand,
}

impl<'a> VideosList<'a> {
    pub fn new(data: &'a Vec<Video>, cmd: &'a mut ClientCommand) -> VideosList<'a> {
        Self { data, cmd }
    }
}

impl Widget for VideosList<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.set_min_width(MIN_W);
        let tablebuilder = TableBuilder::new(ui).column(Column::auto());

        let mut clicked_id = None;

        tablebuilder.body(|mut body| {
            for video in self.data {
                body.row(ROW_H, |mut row| {
                    row.col(|ui| {
                        let resp = ui
                            .scope_builder(
                                UiBuilder::new().sense(Sense::click().union(Sense::hover())),
                                |ui| {
                                    let response = ui.response();
                                    ui.visuals_mut().widgets.inactive.weak_bg_fill =
                                        Color32::TRANSPARENT;
                                    let visuals = ui.style().interact(&response);
                                    let corner_radius =
                                        ui.visuals().widgets.noninteractive.corner_radius;
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    Frame::new()
                                        .fill(visuals.weak_bg_fill)
                                        .inner_margin(4.0)
                                        .corner_radius(corner_radius)
                                        .show(ui, |ui| {
                                            let channel_name = video.channel().display_name();
                                            let views = video.views();
                                            let timestamp = video.publish_timestamp();

                                            ui.set_min_width(MIN_W);
                                            ui.horizontal(|ui| {
                                                Frame::group(ui.style()).inner_margin(0).show(
                                                    ui,
                                                    |ui| {
                                                        ui.set_width(ROW_H * 16.0 / 9.0);
                                                        ui.set_height(ROW_H);
                                                        ui.add(
                                                            Image::new(
                                                                String::from(
                                                                    "https://peertube.wtf",
                                                                ) + video.thumbnail_path(),
                                                            )
                                                            .corner_radius(corner_radius),
                                                        )
                                                    },
                                                );
                                                ui.vertical(|ui: &mut egui::Ui| {
                                                    let mut title_job = LayoutJob::default();
                                                    title_job.append(
                                                        video.name(),
                                                        0.0,
                                                        TextFormat {
                                                            font_id: FontId::new(
                                                                16.0,
                                                                FontFamily::Proportional,
                                                            ),
                                                            color: ui.visuals().strong_text_color(),
                                                            ..Default::default()
                                                        },
                                                    );
                                                    title_job.wrap.max_rows = 2;

                                                    ui.scope(|ui| {
                                                        ui.set_height(40.0);
                                                        ui.add(
                                                            Label::new(title_job).selectable(false),
                                                        );
                                                    });

                                                    ui.add(
                                                        Label::new(channel_name)
                                                            .truncate()
                                                            .selectable(false),
                                                    );
                                                    ui.add(
                                                        Label::new(
                                                            RichText::new(format!(
                                                                "{views} views ⚫ {timestamp}"
                                                            ))
                                                            .weak(),
                                                        )
                                                        .selectable(false),
                                                    );
                                                })
                                            });
                                        });
                                },
                            )
                            .response;

                        if resp.clicked() {
                            println!("clickc {}", video.id());
                            clicked_id = Some(video.id());
                        };
                    });
                });
            }
        });

        if let Some(id) = clicked_id {
            *self.cmd = ClientCommand::OpenVideo { id };
        }

        ui.response()
    }
}
