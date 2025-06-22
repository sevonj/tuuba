use std::time::Instant;

use egui::TextureHandle;
use video_rs::Decoder;

pub struct VideoPlaybackData {
    pub decoder: Decoder,
    start_time: Instant,
    frame: usize,
    pub texture_handle: Option<TextureHandle>,
}

impl VideoPlaybackData {
    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn start_mut(&mut self) -> &mut Instant {
        &mut self.start_time
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn frame_mut(&mut self) -> &mut usize {
        &mut self.frame
    }

    pub fn new(url: &str) -> Self {
        let source = url.parse::<video_rs::Url>().unwrap();
        let decoder = Decoder::new(source).expect("failed to create decoder");

        Self {
            decoder,
            start_time: Instant::now(),
            frame: 0,
            texture_handle: None,
        }
    }

    //pub fn update(&mut self) {
    //    self.position = Instant::now() - self.start_time
    //}
}
