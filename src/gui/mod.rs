mod pages;
mod panels;
mod style;
mod widgets;

use pages::VideoPage;
use panels::NavSidebar;

use panels::SiteHeader;

use crate::client::Client;
use crate::client::Page;
use crate::gui::panels::NavSidebarCollapsed;

#[derive(Debug)]
enum ClientCommand {
    None,
    ClosePage,
    OpenVideo { id: usize },
}

pub struct Gui<'a> {
    data: &'a mut Client,
}

impl<'a> Gui<'a> {
    pub fn new(data: &'a mut Client) -> Gui<'a> {
        Self { data }
    }
}

impl Gui<'_> {
    pub fn show(&mut self, ctx: &egui::Context) {
        ctx.style_mut(style::apply_style);
        let mut cmd = ClientCommand::None;

        {
            let config = self.data.instance_config().clone();
            SiteHeader::new(&mut self.data.gui_state, config).show(ctx);
            if self.data.gui_state.expand_dong {
                NavSidebar::new(&mut cmd).show(ctx);
            } else {
                NavSidebarCollapsed::new(&mut cmd).show(ctx);
            }

            if let Some(page) = self.data.page() {
                match page {
                    Page::Video { video, videos, .. } => {
                        let playback = self.data.playback.clone();
                        VideoPage::new(&mut cmd, video, videos, playback).show(ctx)
                    }
                }
            }
            //} else {
            //    //let feed = self.data.feed();
            //    //FeedPage::new(&mut cmd, &feed.data().lock().expect("tainted mutex")).show(ctx);
            //};
        }
        self.handle_command(cmd);
    }

    fn handle_command(&mut self, cmd: ClientCommand) {
        match cmd {
            ClientCommand::None => (),
            ClientCommand::ClosePage => self.data.close_page(),
            ClientCommand::OpenVideo { id } => self.data.open_video(id),
        }
    }
}
