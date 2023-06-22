use crate::log::{Job, LogPipe};
use crate::queue::{AudioUpdate, Receiver, Sender};
use pasts::prelude::*;
pub async fn start(
    mut audio_from_main: Receiver<AudioUpdate>,
    mut audio_to_main: Sender<AudioUpdate>,
    audio_compute_log: LogPipe,
) {
    audio_compute_log.info("started audio processing", Job::AudioCompute);
}
