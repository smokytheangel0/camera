use crate::log::{Job, LogPipe};
use crate::queue::{AudioUpdate, Sender};

/// this function sets up and begins streaming data from
/// a USB or analogue microphone, currently making use
/// of the linus audio server layer, it then sends each
/// audio frame down the queue to the audio processing
/// functions
pub async fn start(queue: Sender<AudioUpdate>, microphone_log: LogPipe) {
    microphone_log.info("started audio input", Job::AudioInput);
}
