/// This is where we define the queues used to channel
/// our video and audio data to processing, and both
/// from processing to storage in their repective devices
/// We use lockfree queues in order to make sure that
/// all frames are stored in such a way that two threads
/// can access seperate sides of the queue, and store
/// real time data, so that lags in processing and storing
/// the data, do not influence whether frames are dropped
/// Storing our audio and video frames this way allows us
/// keep away from race conditions, without locking the
/// queue when its in use. This can be thought of as
/// a pipeline with a sender and a receiver.
pub use nolock::queues::spsc::bounded::async_queue as new_bounded_async_queue;
pub use nolock::queues::spsc::bounded::AsyncBoundedReceiver as Receiver;
pub use nolock::queues::spsc::bounded::AsyncBoundedSender as Sender;
/// this is where we store our audio
/// frames and information like timestamp
/// in order to pass the audio through
/// the pipeline (queue)
pub struct AudioUpdate {}
/// one minute worth of frames
const AUDIO_QUEUE_SIZE: usize = 3600;

/// this is how we lock a pipeline (queue)
/// to accept only the AudioUpdate type
/// we use this one for Audio Input
pub struct AudioIn {}
impl AudioIn {
    pub fn new() -> (Receiver<AudioUpdate>, Sender<AudioUpdate>) {
        new_bounded_async_queue(AUDIO_QUEUE_SIZE)
    }
}
/// this is how we lock a pipeline(channel)
/// to accept only the AudioUpdate type
/// we use this one for sending frames
/// to the audio processing thread
pub struct ToAudioCompute {}
impl ToAudioCompute {
    pub fn new() -> (Receiver<AudioUpdate>, Sender<AudioUpdate>) {
        new_bounded_async_queue(AUDIO_QUEUE_SIZE)
    }
}

/// this is how we lock a pipeline (channel)
/// to accept only the AudioUpdate type
/// we use this one for receiving frames
/// from the audio processing thread
pub struct FromAudioCompute {}
impl FromAudioCompute {
    pub fn new() -> (Receiver<AudioUpdate>, Sender<AudioUpdate>) {
        new_bounded_async_queue(AUDIO_QUEUE_SIZE)
    }
}

/// this is how we lock a pipeline(queue)
/// to accept only the AudioUpdate type
/// we use this one for Audio Storage
pub struct AudioStorage {}
impl AudioStorage {
    pub fn new() -> (Receiver<AudioUpdate>, Sender<AudioUpdate>) {
        new_bounded_async_queue(AUDIO_QUEUE_SIZE)
    }
}

/// this is where we store our log
/// entries and information like timestamp
/// in order to pass the log entry though
/// the pipeline (queue)
pub use crate::log::LogUpdate;
///one minute worth of frames
const LOG_QUEUE_SIZE: usize = 60 * 60;
/// this is how we lock a pipeline (queue)
/// to accept only the LogUpdate type
/// we use this one to move log entries
/// to storage
pub struct LogStorage {}
impl LogStorage {
    pub fn new() -> (Receiver<LogUpdate>, Sender<LogUpdate>) {
        new_bounded_async_queue(LOG_QUEUE_SIZE)
    }
}

/// this is how we lock a pipeline (queue)
/// to accept only the LogUpdate type
/// we use this one to move log entries
/// to the bluetooth radio for sending
pub struct LogOut {}
impl LogOut {
    pub fn new() -> (Receiver<LogUpdate>, Sender<LogUpdate>) {
        new_bounded_async_queue(LOG_QUEUE_SIZE)
    }
}

/// this is where we store our statuses
/// for the UI to display to the user
use crate::ui::ViewUpdate;
/// one minute worth of frames
const VIEW_QUEUE_SIZE: usize = 60 * 60;

/// this is how we lock a pipeline (queue)
/// to accept only the View Update type
/// we use this one to send information
/// to the UI to show the user on screen
pub struct ViewOut {}
impl ViewOut {
    pub fn new() -> (Receiver<ViewUpdate>, Sender<ViewUpdate>) {
        new_bounded_async_queue(VIEW_QUEUE_SIZE)
    }
}

/// this is where we store our video frames
/// and information like timestamp
/// through the pipeline (queue)
pub struct VideoUpdate {}
/// one minute worth of frames
const VIDEO_QUEUE_SIZE: usize = 3600;

/// this is how we lock a pipeline (queue)
/// to accept only the Video Update type
/// this one moves video frames from the
/// camera to the main function
pub struct VideoIn {}
impl VideoIn {
    pub fn new() -> (Receiver<VideoUpdate>, Sender<VideoUpdate>) {
        new_bounded_async_queue(VIDEO_QUEUE_SIZE)
    }
}

/// this is how we lock a pipeline(channel)
/// to accept only the VideoUpdate type
/// we use this one for sending frames
/// to the video processing thread
pub struct ToVideoCompute {}
impl ToVideoCompute {
    pub fn new() -> (Receiver<VideoUpdate>, Sender<VideoUpdate>) {
        new_bounded_async_queue(VIDEO_QUEUE_SIZE)
    }
}

/// this is how we lock a pipeline (channel)
/// to accept only the VideoUpdate type
/// we use this one for receiving frames
/// from the video processing thread
pub struct FromVideoCompute {}
impl FromVideoCompute {
    pub fn new() -> (Receiver<VideoUpdate>, Sender<VideoUpdate>) {
        new_bounded_async_queue(VIDEO_QUEUE_SIZE)
    }
}

/// this is how we lock a pipeline (queue)
/// to accept only the Video Update type
/// this one moves video frames from the
/// main function to storage
pub struct VideoStorage {}
impl VideoStorage {
    pub fn new() -> (Receiver<VideoUpdate>, Sender<VideoUpdate>) {
        new_bounded_async_queue(VIDEO_QUEUE_SIZE)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _this() {}
}
