use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nolock::queues::mpsc::jiffy::{queue, Sender};
use nolock::queues::spsc::bounded::BoundedSender;

/// a global that marks whether set_pipe() has been called
const INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
pub struct LogPipe {
    sender: Arc<Sender<LogUpdate>>,
    from_task: bool,
    from_thread: bool,
}

impl LogPipe {
    /// call this function to hook up the logging
    /// queue, a new sender is available in each
    /// call of info(), warn(), error(), or trace()
    pub fn set_pipe(
        log_out_sender: BoundedSender<LogUpdate>,
        log_storage_sender: BoundedSender<LogUpdate>,
    ) -> LogPipe {
        if !INITIALIZED.load(Ordering::SeqCst) {
            if !cfg!(no_std) {
                simple_logger::init()
                    .expect("failed to initialize simple_logger");
            }

            let (mut receiver, sender) = queue::<LogUpdate>();

            std::thread::spawn(move || {
                'read_update: loop {
                    let update = match receiver.try_dequeue() {
                        Ok(update) => update,
                        Err(err) => match err {
                            nolock::queues::DequeueError::Closed => {
                                panic!("mpsc log queue was closed by sender?")
                            }
                            nolock::queues::DequeueError::Empty => {
                                continue 'read_update;
                            }
                        },
                    };

                    if cfg!(no_std) {
                        //this should be replaced by an embedded serial write
                        //println!("{:?}", update);
                    } else {
                        //by using a default user_string length and always padding
                        // the user_string up to the length, we can make our details line up
                        let mut user_string =
                            format!("{} !>", update.user_string);
                        let desired_length = 42;
                        if update.user_string.len() < desired_length {
                            let difference =
                                desired_length - update.user_string.len();
                            let padding = " ";
                            for _i in 0..difference {
                                user_string =
                                    format!("{}{}", user_string, padding);
                            }
                        }

                        //println!("{} job: {:?}, thread: {}, task: {}", user_string, update.job, update.from_thread, update.from_task);
                        match update.level {
                            Level::Error => log::error!(
                                "{} job: {:?}, thread: {}, task: {}",
                                user_string,
                                update.job,
                                update.from_thread,
                                update.from_task
                            ),
                            Level::Warn => log::warn!(
                                "{} job: {:?}, thread: {}, task: {}",
                                user_string,
                                update.job,
                                update.from_thread,
                                update.from_task
                            ),
                            Level::Info => log::info!(
                                "{} job: {:?}, thread: {}, task: {}",
                                user_string,
                                update.job,
                                update.from_thread,
                                update.from_task
                            ),
                        }
                    }

                    // print, clone and push the update onto log out and log storage
                    // on shutdown break the loop so the stuff may be dropped
                    //break 'read_update;
                }
                //drop the receiver, so calls to enqueue
                //return an error
                drop(receiver);
            });

            return LogPipe {
                sender: Arc::new(sender),
                from_task: false,
                from_thread: false,
            };
        } else {
            panic!("you cannot call log::set_pipe() twice");
        }
    }

    /// call this function to log to stdout
    /// when in debug mode, and to disk and
    /// through broadcast when in production
    pub fn info(&self, explanation: &str, job: Job) {
        if !INITIALIZED.load(Ordering::SeqCst) {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    user_string: explanation.into(),
                    level: Level::Info,
                    job,
                    from_task: self.from_task,
                    from_thread: self.from_thread,
                })
                .expect("failed to send info log update to the receiver");
        } else {
            panic!("must call log::set_pipe() before using log::info()");
        }
    }

    /// call this function to log to stdout
    /// when in debug mode, and to disk and
    /// through broadcast when in production
    pub fn warn(&self, condition: &str, job: Job) {
        if !INITIALIZED.load(Ordering::SeqCst) {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    user_string: condition.into(),
                    level: Level::Warn,
                    job,
                    from_task: self.from_task,
                    from_thread: self.from_thread,
                })
                .expect("failed to send warn log update to the receiver");
        } else {
            panic!("must call log::set_pipe() before using log::warn()");
        }
    }

    /// call this function to log to stdout
    /// when in debug mode, and to disk and
    /// through broadcast when in production
    pub fn error(&self, shouldnt_happen: &str, job: Job) {
        if !INITIALIZED.load(Ordering::SeqCst) {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    user_string: shouldnt_happen.into(),
                    level: Level::Error,
                    job,
                    from_task: self.from_task,
                    from_thread: self.from_thread,
                })
                .expect("failed to send warn log update to the receiver");
        } else {
            panic!("must call log::set_pipe() before using log::error()");
        }
    }

    // a convenience function to make
    // thread log use more concise
    pub fn new_thread_log(&mut self) -> LogPipe {
        self.from_thread = true;
        self.clone()
    }

    // a convenience function to make
    // task log use more concise
    pub fn new_task_log(&mut self) -> LogPipe {
        self.from_task = true;
        self.clone()
    }
}
/// this is a single frame
/// of the logger
#[derive(Debug)]
pub struct LogUpdate {
    /// this is where we store the string given to us
    user_string: String,
    /// this is where we store an enum of the level
    level: Level,
    /// this is where we store an enum of where the
    /// message comes from within the program
    job: Job,
    /// this is where we make note of whether
    /// the message comes from a task
    from_task: bool,
    /// this is where we make not of whether
    /// the message comes from a thread
    from_thread: bool,
}

/// the level describes the severity
/// of a log update, meaning with some
/// work the user can hide irrelevant
/// messages like Info
#[derive(Debug)]
enum Level {
    /// designed to walk new users through
    /// this program's execution
    Info,
    /// designed to express the boundaries
    /// of system parts to the developer
    Warn,
    /// designed to express when an error
    /// has been recovered from
    Error,
}

#[derive(Debug)]
///TODO: doc
pub enum Job {
    LogOut,
    LogStorage,
    LogSetup,
    AudioCompute,
    AudioInput,
    AudioStorage,
    AudioSetup,
    VideoCompute,
    VideoInput,
    VideoStorage,
    VideoSetup,
    UI,
    UISetup,
    Main,
    Debug,
}
