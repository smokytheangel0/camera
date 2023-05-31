use iso8601::DateTime;
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
//use lockfree::queue::Queue;
use whisk::Channel;

/// this is where we store our audio
/// frames and information like timestamp
/// in order to pass the audio through
/// the pipeline (queue)
pub struct AudioUpdate {}

/// this is how we lock a pipeline (queue)
/// to accept only the AudioUpdate type
/// we use this one for Audio Input
pub type AudioIn = Channel<Option<AudioUpdate>>;
/// this is how we lock a pipeline(channel)
/// to accept only the AudioUpdate type
/// we use this one for sending frames
/// to the audio processing thread
pub type ToAudioCompute = Channel<Option<AudioUpdate>>;

/// this is how we lock a pipeline (channel)
/// to accept only the AudioUpdate type
/// we use this one for receiving frames
/// from the audio processing thread
pub type FromAudioCompute = Channel<Option<AudioUpdate>>;

/// this is how we lock a pipeline(queue)
/// to accept only the AudioUpdate type
/// we use this one for Audio Storage
pub type AudioStorage = Channel<Option<AudioUpdate>>;

/// this is where we store our log
/// entries and information like timestamp
/// in order to pass the log entry though
/// the pipeline (queue)
pub struct LogUpdate {}

/// this is how we lock a pipeline (queue)
/// to accept only the LogUpdate type
/// we use this one to move log entries
/// to storage
pub type LogStorage = Channel<Option<LogUpdate>>;

/// this is how we lock a pipeline (queue)
/// to accept only the LogUpdate type
/// we use this one to move log entries
/// to the bluetooth radio for sending
pub type LogOut = Channel<Option<LogUpdate>>;

/// this is where we store our statuses
/// for the UI to display to the user
pub struct ViewUpdate {}

/// this is how we lock a pipeline (queue)
/// to accept only the View Update type
/// we use this one to send information
/// to the UI to show the user on screen
pub type ViewOut = Channel<Option<ViewUpdate>>;

/// this is where we store our video frames
/// and information like timestamp
/// through the pipeline (queue)
pub struct VideoUpdate {}

/// this is how we lock a pipeline (queue)
/// to accept only the Video Update type
/// this one moves video frames from the
/// camera to the main function
pub type VideoIn = Channel<Option<VideoUpdate>>;

/// this is how we lock a pipeline(channel)
/// to accept only the VideoUpdate type
/// we use this one for sending frames
/// to the video processing thread
pub type ToVideoCompute = Channel<Option<VideoUpdate>>;

/// this is how we lock a pipeline (channel)
/// to accept only the VideoUpdate type
/// we use this one for receiving frames
/// from the video processing thread
pub type FromVideoCompute = Channel<Option<VideoUpdate>>;

/// this is how we lock a pipeline (queue)
/// to accept only the Video Update type
/// this one moves video frames from the
/// main function to storage
pub type VideoStorage = Channel<Option<VideoUpdate>>;

#[cfg(test)]
mod tests {
    #[test]
    fn _this() {}
}
