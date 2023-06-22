use crate::log::{Job, LogPipe};
use crate::queue::{AudioUpdate, LogUpdate, Receiver, VideoUpdate};
/// This is where we will retrieve frames in order
/// from the video and audio queues, and begin a file
/// or continue a file for each type. The events which will
/// determine whether to begin a new file or to continue
/// an existing one are as follows: new mass storage insertion,
/// mass storage capacity, no mass storage available.
/// A sled Db will be appended to as necessary to keep
/// on disk a record of which mass storage serial number
/// which file was saved on, and the completion or error
/// of that file's progress. An option will allow backing up
/// each audio and video chunk to disk or cloud, besides
/// saving it to removable storage.
/// Any article saved on the MainDisk, will be encrypted
/// including the source code.

/// TODO: split into two tasks main and removable,
/// writes to these can happen in parallel

/// This enum describes which state that your
/// main storage is in, this storage is
/// attached to the laptop internally
pub enum MainStorage {
    /// indicates the device has sufficient capacity
    HasCapacity,
    /// indicates the storage device will become full in less than 24 hours
    NearFull(
        /// number of seconds until full, at current average write rate
        u64,
    ),
    /// indicates the device has filled its capacity
    Full,
}

/// This enum describes which state your
/// removable storage is in, this storage
/// is attached to the laptop's USB port
pub enum RemovableStorage {
    /// indicates the device has sufficient capacity
    HasCapacity,
    /// the storage device will become full in less than an hour
    NearFull(
        /// number of seconds until full, at current average write rate
        u64,
    ),
    /// indicates the device has filled its capacity
    Full,
}

/// this function opens a file on either the main storage
/// usb storage or both, and appends each LogUpdate that
/// comes down the pipe, to the file that was created
pub async fn log_start(queue: Receiver<LogUpdate>, log_storage_log: LogPipe) {
    log_storage_log.info("started log storage", Job::LogStorage);
}

/// this function opens a file on either the main storage
/// (if no removable exists), or by default to the usb
/// storage, allowing the user to view the videos on
/// a PlayStation 3
pub async fn video_start(
    queue: Receiver<VideoUpdate>,
    video_storage_log: LogPipe,
) {
    video_storage_log.info("started video storage", Job::VideoStorage);
}

/// this function opens a file on the main storage
/// in order to keep potentially illegal audio recordings
/// seperate from the video files to be used in legal
/// proceedings, audio files should only be used for
/// intelligence gathering rather than for capturing
/// disruptive activity directly
pub async fn audio_start(
    queue: Receiver<AudioUpdate>,
    audio_storage_log: LogPipe,
) {
    audio_storage_log.info("started audio storage", Job::AudioStorage);
}
