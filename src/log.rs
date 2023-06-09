use crate::queue::Sender as AsyncSender;
/// these functions should format the incoming string with time, and !>
///
/// they should use a global receiver and should feed the
/// println! macro on std and log out and log storage pipelines
///
///
/// this file should also include the sender functions
use nolock::queues::mpsc::jiffy::{queue, Receiver, Sender};
use std::mem::MaybeUninit;
use std::sync::Arc;

/// the hard coded size of the queue
/// 60 fps for 60 seconds (1 minute)
const QUEUE_SIZE: usize = 60 * 60;

/// the memory location where our logging
/// facilities can find the receiving end
/// of the log queue, must be initialized
/// with set_pipe() in main
const RECEIVER: MaybeUninit<Receiver<LogUpdate>> = MaybeUninit::<Receiver<LogUpdate>>::uninit();

/// the memory location where the user
/// logging functions can find the mother
/// send side of the queue, to be cloned for
/// each new use, the clone will be discarded
/// at the end of the function
const SENDER: MaybeUninit<Arc<Sender<LogUpdate>>> = MaybeUninit::<Arc<Sender<LogUpdate>>>::uninit();
// TODO: must remember to decrement the arc on final shutdown

/// a value that marks whether set_pipe() has been called
static mut INITIALIZED: bool = false;

/// call this function to hook up the logging
/// queue, a new sender is available in each
/// call of info(), warn(), error(), or trace()
#[must_use]
pub fn set_pipe(
    log_out_sender: AsyncSender<LogUpdate>,
    log_storage_sender: AsyncSender<LogUpdate>,
) {
    // INITIALIZED always has a value
    if !unsafe { INITIALIZED } {
        let (receiver, sender) = queue::<LogUpdate>();
        unsafe {
            // the receiver is meant to be
            // used by a single consumer
            // this is only GLOBAL because
            // the queue() function from
            // nolock returns both sides
            RECEIVER.write(receiver);

            // the sender is meant to be cloned
            // so that multiple producers may
            // queue updates through it, it
            // is global to simplify log
            // function arguments
            SENDER.write(Arc::new(sender));
            INITIALIZED = true;
        }

        // dequeue elements from the receiver as they are sent
        loop {
            // we can assume the RECEIVER has been initialized
            let update;
            unsafe {
                update = RECEIVER
                    .assume_init()
                    .try_dequeue()
                    .expect("failed to pop LogUpdate from the pipe");
            }
            // print, clone and push the update onto log out and log storage
            // on shutdown break the loop so the stuff may be dropped
        }
        unsafe {
            INITIALIZED = false;
            SENDER.assume_init_drop();
            RECEIVER.assume_init_drop();
        }
    } else {
        panic!("you cannot call log::set_pipe() twice");
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

/// call this function to log to stdout
/// when in debug mode, and to disk and
/// through broadcast when in production
pub fn info(explanation: &str) {
    // INITIALIZED always has a value
    if !unsafe { INITIALIZED } {
        let local_sender = unsafe { SENDER.assume_init().clone() };
        local_sender.enqueue(LogUpdate {
            user_string: explanation.into(),
            level: Level::Info,
        }).expect("failed to send info log update to the receiver");
    } else {
        panic!("must call log::set_pipe() before using log::info()");
    }
}

/// call this function to log to stdout
/// when in debug mode, and to disk and
/// through broadcast when in production
pub fn warn(condition: &str) {
    // INITIALIZED always has a value
    if !unsafe { INITIALIZED } {
        let local_sender = unsafe { SENDER.assume_init().clone() };
        local_sender.enqueue(LogUpdate {
            user_string: condition.into(),
            level: Level::Warn,
        }).expect("failed to send warn log update to the receiver");
    } else {
        panic!("must call log::set_pipe() before using log::warn()");
    }
}

/// call this function to log to stdout
/// when in debug mode, and to disk and
/// through broadcast when in production
pub fn error(shouldnt_happen: &str) {
    // INITIALIZED always has a value
    if !unsafe { INITIALIZED } {
        let local_sender = unsafe { SENDER.assume_init().clone() };
        local_sender.enqueue(LogUpdate {
            user_string: shouldnt_happen.into(),
            level: Level::Error,
        }).expect("failed to send warn log update to the receiver");
    } else {
        panic!("must call log::set_pipe() before using log::error()");
    }
}
