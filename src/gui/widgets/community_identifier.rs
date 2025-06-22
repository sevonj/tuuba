use egui::{RichText, Vec2, Widget};



pub struct CommunityIdenfifier<'a> {
    community: &'a Community,
}

impl<'a> CommunityIdenfifier<'a> {
    pub fn new(community: &'a Community) -> CommunityIdenfifier<'a> {
        Self { community }
    }
}

impl Widget for CommunityIdenfifier<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            let name = &self.community.name;
            let domain = self.community.actor_id.domain().unwrap();

            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.label(RichText::new("!").monospace());
            ui.label(RichText::new(name).monospace().strong());
            ui.label(RichText::new(format!("@{domain}")).monospace());
        })
        .response
    }
}
