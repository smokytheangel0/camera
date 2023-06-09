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

pub async fn log_start(queue: Receiver<LogUpdate>) {
    println!("started log storage task !>");
}
pub fn video_start(queue: Receiver<VideoUpdate>) {}
pub fn audio_start(queue: Receiver<AudioUpdate>) {}
