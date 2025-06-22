use egui::{Color32, ProgressBar, Widget};

pub struct VoteBar {
    upvotes: usize,
    downvotes: usize,
}

impl VoteBar {
    pub fn new(upvotes: usize, downvotes: usize) -> VoteBar {
        Self { upvotes, downvotes }
    }
}

impl Widget for VoteBar {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let up = self.upvotes;
        let dn = self.downvotes;
        let total = up + dn;
        let value = if total > 0 {
            up as f32 / total as f32
        } else {
            0.5
        };

        ui.add(
            ProgressBar::new(value)
                .desired_height(4.)
                .desired_width(192.)
                .fill(Color32::DARK_GREEN),
        )
    }
}
