use egui::{RichText, Vec2, Widget};


pub struct PersonIdentifier<'a> {
    person: &'a Person,
}

impl<'a> PersonIdentifier<'a> {
    pub fn new(person: &'a Person) -> PersonIdentifier<'a> {
        Self { person }
    }
}

impl Widget for PersonIdentifier<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            let name = &self.person.name;
            let domain = self.person.actor_id.domain().unwrap();

            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.label(RichText::new("@").monospace());
            ui.label(RichText::new(name).monospace().strong());
            ui.label(RichText::new(format!("@{domain}")).monospace());
        })
        .response
    }
}
