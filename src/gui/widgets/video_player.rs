use std::time::Instant;

use egui::{
    Color32, ColorImage, Image, Rect, Sense, TextureOptions, Widget, load::SizedTexture, vec2,
};

use crate::{
    client::{ArcMutexOpt, VideoPlaybackData},
    gui::widgets::LoadingSpinner,
};

pub struct VideoPlayer {
    playback: ArcMutexOpt<VideoPlaybackData>,
}

impl VideoPlayer {
    pub fn new(playback: ArcMutexOpt<VideoPlaybackData>) -> Self {
        Self { playback }
    }
}

impl Widget for VideoPlayer {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let should_advance_frame = self.should_advance_frame();

        {
            let mut playback_guard = self.playback.lock().unwrap();
            if let Some(playback) = playback_guard.as_mut() {
                let (w, h) = playback.decoder.size();

                if playback.texture_handle.is_none() {
                    playback.texture_handle.replace(create_texture(ui, w, h));
                }

                if should_advance_frame {
                    if let Ok((_, frame)) = playback.decoder.decode() {
                        let handle = playback.texture_handle.as_mut().unwrap();

                        let slice = frame.as_slice().unwrap();
                        let image = ColorImage::from_rgb([w as usize, h as usize], slice);
                        let options = TextureOptions::default();

                        handle.set(image, options);
                        *playback.frame_mut() += 1;
                    }
                }
            }
        }

        let aspect_ratio: f32 = 16.0 / 9.0;
        let avail_size = ui.available_rect_before_wrap().size();
        let mut video_size = avail_size;
        if video_size.x / video_size.y > aspect_ratio {
            video_size.x = video_size.y * aspect_ratio;
        } else {
            video_size.y = video_size.x / aspect_ratio;
        }
        egui::Frame::new()
            .fill(Color32::BLACK)
            .show(ui, |ui| {
                let mut min = ui.available_rect_before_wrap().min;
                if video_size.x < avail_size.x {
                    min.x += (avail_size.x - video_size.x) / 2.0;
                }
                let rect = Rect::from_min_size(min, video_size);
                ui.allocate_exact_size(vec2(avail_size.x, video_size.y), Sense::empty());
                if let Some(image) = self.frame_image() {
                    image.paint_at(ui, rect);
                } else {
                    ui.add(LoadingSpinner);
                }
            })
            .response
    }
}

impl VideoPlayer {
    fn should_advance_frame(&self) -> bool {
        let playback_guard = self.playback.lock().unwrap();
        let Some(playback) = playback_guard.as_ref() else {
            return false;
        };
        let time = (Instant::now() - playback.start_time()).as_secs_f64();
        let fps = playback.decoder.frame_rate() as f64;
        let current_frame = (time * fps).floor() as usize;
        if current_frame <= playback.frame() {
            return false;
        }

        true
    }

    fn frame_image(&mut self) -> Option<Image> {
        let mut playback_guard = self.playback.lock().unwrap();
        let playback = playback_guard.as_mut()?;
        let handle = playback.texture_handle.as_mut()?;
        Some(Image::new(SizedTexture::new(
            handle.id(),
            handle.size_vec2(),
        )))
    }
}

fn create_texture(ui: &mut egui::Ui, w: u32, h: u32) -> egui::TextureHandle {
    let handle = ui.ctx().load_texture(
        "screen",
        ColorImage::new([h as usize, w as usize], Color32::BLACK),
        TextureOptions::default(),
    );
    handle
}
