use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nolock::queues::mpsc::jiffy::{queue, Sender};
use nolock::queues::spsc::unbounded::UnboundedSender;

use cfg_if::cfg_if;

#[cfg(feature = "std")]
use std::format;
#[cfg(feature = "std")]
use std::string::String;

/// a global that marks whether set_pipe() has been called
static mut INITIALIZED: AtomicBool = AtomicBool::new(false);

/// a global that marks whether the system is headed for a shutdown
static mut SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
/// this structure is used to hold important
/// logging state, so that it may be used
/// correctly and freely from the main program
pub struct LogPipe {
    /// an Arc'd Sender allows
    /// us to clone the send side
    /// of the pipe for use with
    /// multiple senders
    sender: Arc<Sender<LogUpdate>>,
    /// this indicates the log message
    /// came from inside an async task
    from_task: bool,
    /// this indicates the log message
    /// came from inside another thread
    /// besides main
    from_thread: bool,
    // refrain from holding job state in
    // here, it is global to the entire
    // program
}

impl LogPipe {
    /// call this function to hook up the logging
    /// queue, a new sender is available in each
    /// call of info(), warn(), error(), or trace()
    pub fn set_pipe(
        mut log_out_sender: crate::queue::Sender<LogUpdate>,
        mut log_storage_sender: crate::queue::Sender<LogUpdate>,
    ) -> LogPipe {
        if unsafe { !INITIALIZED.load(Ordering::SeqCst) } {
            /*
            {
                #[cfg(feature = "std")]
                simple_logger::init()
                    .expect("failed to initialize simple_logger");

            }
            */

            let (mut receiver, sender) = queue::<LogUpdate>();

            let _ = std::thread::Builder::new().name("log listener".into()).spawn(move || {
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

                    #[allow(unused_variables)]
                    let padded_string = LogPipe::pad_user_string(&update.user_string);
                    //println!("{} job: {:?}, thread: {}, task: {}", user_string, update.job, update.from_thread, update.from_task);

                    match update.job {
                        Job::LogOut => {
                            cfg_if! {
                                if #[cfg(all(feature = "logout", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "logout")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::LogStorage => {
                            cfg_if! {
                                if #[cfg(all(feature = "logstorage", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "logstorage")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::LogSetup => {
                            cfg_if! {
                                if #[cfg(all(feature = "logsetup", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "logsetup")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::AudioCompute => {
                            cfg_if! {
                                if #[cfg(all(feature = "audiocompute", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "audiocompute")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::AudioInput => {
                            cfg_if! {
                                if #[cfg(all(feature = "audioinput", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "audioinput")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::AudioStorage => {
                            cfg_if! {
                                if #[cfg(all(feature = "audiostorage", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "audiostorage")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::AudioSetup => {
                            cfg_if! {
                                if #[cfg(all(feature = "audiosetup", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "audiosetup")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::VideoCompute => {
                            cfg_if! {
                                if #[cfg(all(feature = "videocompute", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "videocompute")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::VideoInput => {
                            cfg_if! {
                                if #[cfg(all(feature = "videoinput", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "videoinput")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::VideoStorage => {
                            cfg_if! {
                                if #[cfg(all(feature = "videostorage", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "videostorage")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::VideoSetup => {
                            cfg_if! {
                                if #[cfg(all(feature = "videosetup", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "videosetup")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::UI => {
                            cfg_if! {
                                if #[cfg(all(feature = "ui", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "ui")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::UISetup => {
                            cfg_if! {
                                if #[cfg(all(feature = "uisetup", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "uisetup")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::Main => {
                            cfg_if! {
                                if #[cfg(all(feature = "main", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "main")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                        Job::Debug => {
                            cfg_if! {
                                if #[cfg(all(feature = "debug", feature = "std"))] {
                                    LogPipe::use_simple_logger(&update, &padded_string);
                                } else if #[cfg(feature = "debug")] {
                                    LogPipe::embedded_write(&update);
                                }

                            }
                            log_storage_sender.enqueue(update.clone()).expect("failed to send LogUpdate to log_storage");
                            log_out_sender.enqueue(update).expect("failed to send LogUpdate to log_out");
                        }
                    }
                    
                    if unsafe{SHUTDOWN.load(Ordering::SeqCst)} {break 'read_update;} else {continue 'read_update;}
                } // 'read_update
                // drop the receiver, so it cleans up
                // and marks itself as closed so further
                // use returns a handleable error
                unsafe{INITIALIZED.store(false, Ordering::SeqCst);}
                drop(receiver);
            }); // log listener thread;
            unsafe {
                INITIALIZED.store(true, Ordering::SeqCst);
            }
            LogPipe {
                sender: Arc::new(sender),
                from_task: false,
                from_thread: false,
            }
        } else {
            panic!("you cannot call log::set_pipe() twice");
        }
    }

    /// call this function to log to stdout
    /// when in debug mode, and to disk and
    /// through broadcast when in production
    pub fn info(&self, explanation: &str, job: Job) {
        if unsafe { INITIALIZED.load(Ordering::SeqCst) } {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    timestamp: crate::get_timestamp(),
                    user_string: explanation.into(),
                    level: Level::Info,
                    job: job,
                    from_task: self.from_task,
                    thread_name: std::thread::current()
                        .name()
                        .unwrap_or("no name returned")
                        .into(),
                })
                .expect("failed to send info log update to the receiver");
        } else {
            panic!("must call log::set_pipe() before using log::info()");
        }
    }

    /// call this function to log to stdout
    /// when in debug mode, and to disk and
    /// through broadcast when in production
    pub fn debug(&self, explanation: &str) {
        if unsafe { INITIALIZED.load(Ordering::SeqCst) } {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    timestamp: crate::get_timestamp(),
                    user_string: explanation.into(),
                    level: Level::Info,
                    job: Job::Debug,
                    from_task: self.from_task,
                    thread_name: std::thread::current()
                        .name()
                        .unwrap_or("no name returned")
                        .into(),
                })
                .expect("failed to send debug log update to the receiver");
        } else {
            panic!("must call log::set_pipe() before using log::debug()");
        }
    }

    /// call this function to log to stdout
    /// when in debug mode, and to disk and
    /// through broadcast when in production
    pub fn warn(&self, condition: &str, job: Job) {
        if unsafe { INITIALIZED.load(Ordering::SeqCst) } {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    timestamp: crate::get_timestamp(),
                    user_string: condition.into(),
                    level: Level::Warn,
                    job,
                    from_task: self.from_task,
                    thread_name: std::thread::current()
                        .name()
                        .unwrap_or("no name returned")
                        .into(),
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
        if unsafe { INITIALIZED.load(Ordering::SeqCst) } {
            self.sender
                .clone()
                .enqueue(LogUpdate {
                    timestamp: crate::get_timestamp(),
                    user_string: shouldnt_happen.into(),
                    level: Level::Error,
                    job,
                    from_task: self.from_task,
                    thread_name: std::thread::current()
                        .name()
                        .unwrap_or("no name returned")
                        .into(),
                })
                .expect("failed to send warn log update to the receiver");
        } else {
            panic!("must call log::set_pipe() before using log::error()");
        }
    }

    /// this function creates a new log sender
    /// and marks it as coming from inside a thread
    pub fn new_thread_log(&mut self) -> LogPipe {
        self.from_thread = true;
        self.clone()
    }

    /// this function creates a new log sender
    /// and marks it as coming from inside an
    /// asynchronous task
    pub fn new_task_log(&mut self) -> LogPipe {
        self.from_task = true;
        self.clone()
    }

    /// this function pads user log strings so that
    /// they display well as print statements
    fn pad_user_string(user_string: &str) -> String {
        let mut padded_string = format!("{} !>", user_string);
        let desired_length = 42;
        if padded_string.len() < desired_length {
            let difference = desired_length - padded_string.len();
            let padding = " ";
            for _i in 0..difference {
                padded_string = format!("{}{}", padded_string, padding);
            }
        }
        padded_string
    }

    /// THIS IS CURRENTLY A NO OP
    fn use_simple_logger(update: &LogUpdate, padded_string: &str) {
        /*
        match update.level {
            Level::Error => log::error!(
                "{} job: {:?}, thread: {}, task: {}",
                padded_string,
                update.job,
                update.from_thread,
                update.from_task
            ),
            Level::Warn => log::warn!(
                "{} job: {:?}, thread: {}, task: {}",
                padded_string,
                update.job,
                update.from_thread,
                update.from_task
            ),
            Level::Info => log::info!(
                "{} job: {:?}, thread: {}, task: {}",
                padded_string,
                update.job,
                update.from_thread,
                update.from_task
            ),
        }
        */
    }

    /// this function will be used to print
    /// across UART or similar embedded systems
    fn embedded_write(update: &LogUpdate) {
        unimplemented!()
    }

    /// this function shuts down the log receivers correctly
    pub fn shutdown() {
        unsafe {
            SHUTDOWN.store(true, Ordering::SeqCst);
        }
    }
}

/// this is a single frame
/// of the logger
#[derive(Debug, Clone)]
pub struct LogUpdate {
    /// this is where we store the string given to us
    pub user_string: String,
    /// this is where we store an enum of the level
    level: Level,
    /// this is where we store an enum of where the
    /// message comes from within the program
    pub job: Job,
    /// this is where we make note of whether
    /// the message comes from a task
    pub from_task: bool,
    /// this is where we make not of whether
    /// the message comes from a thread
    pub thread_name: String,
    /// this is where we store the log's
    /// timestamp for use when storing the file
    pub timestamp: u64,
}

/// the level describes the severity
/// of a log update, meaning with some
/// work the user can hide irrelevant
/// messages like Info
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
/// Distinct Jobs allow us to sort
/// our logs based on the area we are
/// working in, besides just the typical
/// log levels
pub enum Job {
    /// this indicates the log message
    /// came from somewhere in our
    /// bluetooth logging functions
    LogOut,
    /// this indicates the log message
    /// came from somewhere in our
    /// Log Storage functions
    LogStorage,
    /// this indicates the message occured
    /// during setup of the logging aparatus
    /// in the main function
    LogSetup,
    /// this indicates the message came from
    /// the audio post processing functions
    AudioCompute,
    /// this inidcates the message came from
    /// the microphone capture functions
    AudioInput,
    /// this indicates the message came from
    /// the audio storage functinos
    AudioStorage,
    /// this inidcates the message occured
    /// during setup of the audio aparatus
    /// in the main function
    AudioSetup,
    /// this indicates the message came from
    /// the video post processing functions
    VideoCompute,
    /// this indicates the message came from
    /// the camera capture functions
    VideoInput,
    /// this indicates the message came from
    /// the video storage functions
    VideoStorage,
    /// this indicates the message occured
    /// during setup of the video aparatus
    /// in the main function
    VideoSetup,
    /// this indicates the message came
    /// from the UI functions
    UI,
    /// this indicates the messsage occured
    /// during setup of the UI aparatus
    /// in the main function
    UISetup,
    /// this indicates the message came from
    /// the main function where all of the
    /// functions are tied together as
    /// a program
    Main,
    /// this indicates the message is temporarily
    /// used to help figure out a bug
    Debug,
}
