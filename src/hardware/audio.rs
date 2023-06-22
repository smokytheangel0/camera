use crate::log::{Job, LogPipe};
use crate::queue::{AudioUpdate, Sender};
pub async fn start(queue: Sender<AudioUpdate>, microphone_log: LogPipe) {
    microphone_log.info("started audio input", Job::AudioInput);
}
