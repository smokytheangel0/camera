use crate::log::{Job, LogPipe};
use crate::queue::{Receiver, Sender, VideoUpdate};

/// this function receives video frames from
/// the camera function, and processes the
/// frames in order to increase the signal
/// to noise ratio and make movement
/// more perceptable
pub async fn start(
    video_from_main: Receiver<VideoUpdate>,
    video_to_main: Sender<VideoUpdate>,
    video_compute_log: LogPipe,
) {
    video_compute_log.info("started processing video", Job::VideoCompute);
}
