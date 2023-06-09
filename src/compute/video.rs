use crate::queue::{Receiver, Sender, VideoUpdate};
pub fn start(video_from_main: Receiver<VideoUpdate>, video_to_main: Sender<VideoUpdate>) {}
