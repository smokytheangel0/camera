use crate::log::{Job, LogPipe};
use crate::queue::{Receiver, Sender, VideoUpdate};
pub async fn start(
    video_from_main: Receiver<VideoUpdate>,
    video_to_main: Sender<VideoUpdate>,
    video_compute_log: LogPipe,
) {
    video_compute_log.info("started processing video", Job::VideoCompute);
}
