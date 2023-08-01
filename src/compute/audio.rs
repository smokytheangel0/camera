use crate::log::{Job, LogPipe};
use crate::queue::{AudioUpdate, Receiver, Sender};
use pasts::prelude::*;

use std::format;
/// this function is where we perform our audio
/// post processing to gain more human voice signal
/// frma amidst the background noise
pub async fn start(
    mut audio_from_microphone: Receiver<AudioUpdate>,
    mut audio_to_storage: Sender<AudioUpdate>,
    audio_compute_log: LogPipe,
) {
    audio_compute_log.info("started audio processing", Job::AudioCompute);
    for update in audio_from_microphone.try_dequeue() {
        audio_compute_log.info("successfully received frame from microphone task", Job::AudioCompute);
        audio_compute_log.info(&format!("audio compute update {{timestamp: {:?}, name: {:?} }}", update.timestamp, update.name), Job::AudioCompute);
    }
}
