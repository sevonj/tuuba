
use egui::SidePanel;

pub struct InfoSidebar<'a> {
    site_view: &'a SiteView,
}

impl<'a> InfoSidebar<'a> {
    pub fn new(site_view: &'a SiteView) -> InfoSidebar<'a> {
        Self { site_view }
    }
}

impl InfoSidebar<'_> {
    pub fn show(self, ctx: &egui::Context) {
        SidePanel::right("info_sidebar").show(ctx, |ui| {
            ui.heading(&self.site_view.site.name);

            if let Some(description) = &self.site_view.site.description {
                ui.separator();
                ui.label(description);
            }

            if let Some(text) = &self.site_view.site.sidebar {
                ui.separator();
                ui.label(text);
            }
        });
    }
}
