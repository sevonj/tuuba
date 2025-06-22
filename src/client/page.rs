use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

use crate::client::VideoPlaybackData;

use super::api;

use super::ArcMutexOpt;

pub enum Page {
    Video {
        id: usize,
        playback: ArcMutexOpt<VideoPlaybackData>,
        video: ArcMutexOpt<api::types::VideoDetails>,
        videos: ArcMutexOpt<api::types::Videos>,
    },
}

impl Page {
    pub fn video(id: usize, playback: ArcMutexOpt<VideoPlaybackData>) -> Self {
        Self::Video {
            id,
            playback,
            video: Arc::new(Mutex::new(None)),
            videos: Arc::new(Mutex::new(None)),
        }
    }

    pub fn fetch_data(&mut self, api: api::Api) {
        match self {
            Page::Video {
                id,
                playback,
                video,
                videos,
            } => {
                {
                    let api = api.clone();
                    let id = *id;
                    let playback = playback.clone();
                    let video = video.deref().clone();
                    tokio::spawn(async move {
                        match api::types::VideoDetails::get(api, id).await {
                            Ok(value) => {
                                let url = value.playback_url().to_owned();
                                video.lock().as_mut().unwrap().replace(value);

                                // let url = "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";

                                let value = VideoPlaybackData::new(&url);
                                playback.lock().as_mut().unwrap().replace(value);
                            }
                            Err(e) => println!("{e}"),
                        }
                    });
                }
                {
                    let api = api.clone();
                    let videos = videos.deref().clone();
                    tokio::spawn(async move {
                        match api::types::Videos::get(api).await {
                            Ok(value) => {
                                videos.lock().as_mut().unwrap().replace(value);
                            }
                            Err(e) => println!("{e}"),
                        }
                    });
                }
            }
        }
    }
}
