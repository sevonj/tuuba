use egui::{Align, CentralPanel, Frame, Image, Label, Layout, RichText};
use egui::{Color32, SidePanel};

use crate::client::ArcMutexOpt;
use crate::client::VideoPlaybackData;
use crate::client::api::types::{VideoDetails, Videos};
use crate::gui::ClientCommand;
use crate::gui::pages::LoadingPage;
use crate::gui::widgets;

use widgets::{VideoPlayer, VideosList, VoteBar};

//use super::LoadingPage;

const AVATAR_SIZE: f32 = 48.;

/// Full post page with comments
pub struct VideoPage<'a> {
    cmd: &'a mut ClientCommand,
    video: &'a ArcMutexOpt<VideoDetails>,
    videos: &'a ArcMutexOpt<Videos>,
    playback: ArcMutexOpt<VideoPlaybackData>,
}

impl<'a> VideoPage<'a> {
    pub fn new(
        cmd: &'a mut ClientCommand,
        video: &'a ArcMutexOpt<VideoDetails>,
        videos: &'a ArcMutexOpt<Videos>,
        playback: ArcMutexOpt<VideoPlaybackData>,
    ) -> VideoPage<'a> {
        Self {
            cmd,
            video,
            videos,
            playback,
        }
    }
}

impl VideoPage<'_> {
    pub fn show(self, ctx: &egui::Context) {
        CentralPanel::default()
            .frame(super::page_frame(&ctx.style()))
            .show(ctx, |ui| {
                SidePanel::right("videos_list")
                    .frame(Frame::side_top_panel(ui.style()).fill(Color32::TRANSPARENT))
                    .resizable(false)
                    .show_separator_line(false)
                    .show_inside(ui, |ui| {
                        let videos_guard = self.videos.as_ref().lock().unwrap();
                        let Some(videos) = videos_guard.as_ref() else {
                            LoadingPage::new().show_inside(ui);
                            return;
                        };
                        ui.add(VideosList::new(videos.data(), self.cmd));
                    });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let corner_radius = ui.visuals().widgets.noninteractive.corner_radius;

                    let name;
                    let desc: Option<&str>;
                    let views;
                    let likes;
                    let dislikes;
                    let timestamp;
                    let account_name;
                    let channel_name;
                    let channel_avatar;

                    let video_guard = self.video.as_ref().lock().unwrap();
                    if let Some(video) = video_guard.as_ref() {
                        name = video.name();
                        desc = video.description();

                        views = video.views();
                        likes = video.likes();
                        dislikes = video.dislikes();
                        timestamp = video.publish_timestamp();
                        account_name = video.account().display_name();
                        channel_name = video.channel().display_name();
                        if !video.channel().avatars().is_empty() {
                            channel_avatar = Some(
                                String::from("https://peertube.wtf")
                                    + video.channel().avatars()[0].path(),
                            )
                        } else {
                            channel_avatar = None;
                        };
                    } else {
                        name = "Loading...";
                        desc = None;

                        views = 0;
                        likes = 0;
                        dislikes = 0;
                        timestamp = "???";
                        channel_name = "???";
                        channel_avatar = None;
                        account_name = "???";
                    }

                    ui.vertical(|ui| {
                        ui.add(VideoPlayer::new(self.playback));

                        ui.add_space(8.);
                        ui.add(Label::new(RichText::new(name).size(28.0).strong()));
                        ui.add_space(8.);

                        ui.horizontal(|ui| {
                            ui.add(Label::new(
                                RichText::new(format!("{views} views ⚫ {timestamp}")).weak(),
                            ));
                            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                        ui.horizontal(|ui| {
                                            let _ = ui.selectable_label(false, "Dislike ⬇");
                                            let _ = ui.selectable_label(true, "Like ⬆");
                                        });
                                        ui.add(VoteBar::new(likes, dislikes));
                                    });
                                });
                            });
                        });

                        ui.separator();

                        ui.horizontal(|ui| {
                            Frame::new().inner_margin(0).show(ui, |ui| {
                                ui.set_width(AVATAR_SIZE);
                                ui.set_height(AVATAR_SIZE);
                                if let Some(avatar) = channel_avatar {
                                    ui.add(Image::new(avatar).corner_radius(corner_radius));
                                }
                            });
                            ui.vertical(|ui| {
                                ui.label(channel_name);
                                ui.label(format!("by {account_name}"));
                            });
                            ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                let _ = ui.selectable_label(false, "Subscribe");
                            });
                        });
                        ui.add(Label::new(RichText::new("date votes etc todo 1234").weak()));

                        if let Some(desc) = desc {
                            ui.add(Label::new(desc));
                        }

                        // License tags other metadata
                    });
                });
            });
    }
}
