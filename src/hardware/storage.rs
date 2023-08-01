use crate::log::{Job, LogPipe};
use crate::queue::{AudioUpdate, LogUpdate, Receiver, VideoUpdate};

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
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

/// this is used to communicate to the started storage loop
/// that is on a seperate thread
static mut SHUTDOWN: AtomicBool = AtomicBool::new(false);

/// this function opens a file on either the main storage
/// usb storage or both, and appends each LogUpdate that
/// comes down the pipe, to the file that was created
/// TODO: EXTRACT SIDE EFFECTS
pub async fn log_start(
    mut queue: Receiver<LogUpdate>,
    log_storage_log: LogPipe,
) {
    log_storage_log.info("started log storage", Job::LogStorage);

    use std::fs::OpenOptions;
    let mut file;
    file = match OpenOptions::new()
        .read(false)
        .append(true)
        .create(true)
        .open("main_log.txt")
    {
        Ok(file) => file,
        Err(err) => panic!("unable to open or create file because: {:?}", err),
    };
    log_storage_log.info("created or opened file", Job::LogStorage);

    use std::thread::sleep;
    use std::time::{Duration, SystemTime};
    'store_log_update: loop {
        match queue.try_dequeue() {
            Ok(update) => {
                //we will start with newline delimited text
                //noria is also a good option, and has webui
                let line_buffer = format!(
                    "{} :: {} :: {} :: from {{thread: {}, task: {}}}\n",
                    update.timestamp,
                    pad_job_string(&update.job),
                    pad_user_string(&update.user_string),
                    update.thread_name,
                    update.from_task
                );

                match file.write_all(line_buffer.as_bytes()) {
                    Ok(_) => {} //fall through to ,
                    Err(err) => panic!(
                        "could not appened new log update with open file, {:?}",
                        err
                    ),
                };
                match file.flush() {
                    Ok(_) => {}, //fall through to sync_all
                    Err(err) => panic!("there was an error flushing the file buffers to disk: {:?}", err)
                };
                match file.sync_all() {
                    Ok(_) => {}, //fall through to shutdown check
                    Err(err) => panic!("there was a error syncing all os metadata to disk: {:?}", err)
                };
            }
            Err(_) => {} //fall through to shutdown check
        }
        sleep(Duration::new(0, 1000));
        if unsafe { SHUTDOWN.load(Ordering::SeqCst) } {
            break 'store_log_update;
        } else {
            continue 'store_log_update;
        }
    }
}

pub fn log_stop() {
    unsafe { SHUTDOWN.store(true, Ordering::SeqCst) }
}

use std::format;
use std::string::String;
fn pad_user_string(user_string: &str) -> String {
    let mut padded_string = format!("{}", user_string);
    let desired_length = 45;
    if padded_string.len() < desired_length {
        let difference = desired_length - padded_string.len();
        let padding = " ";
        for _i in 0..difference {
            padded_string = format!("{}{}", padded_string, padding);
        }
    }
    padded_string
}

fn pad_job_string(job: &Job) -> String {
    let mut padded_string = format!("{:?}", job);
    let desired_length = 15;
    if padded_string.len() < desired_length {
        let difference = desired_length - padded_string.len();
        let padding = " ";
        for _i in 0..difference {
            padded_string = format!("{}{}", padded_string, padding);
        }
    }
    padded_string
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
