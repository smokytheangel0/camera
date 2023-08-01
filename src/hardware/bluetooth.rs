use crate::log::{Job, LogPipe};
/// In order to enable light logging
/// through bluetooth, we will need
/// to host as non discoverable,
/// use encryption if possible
/// send basic status through bluetooth
/// to feature phone
use crate::queue::{LogUpdate, Receiver};

use std::sync::atomic::{AtomicBool, Ordering};

/// public shutdown atomic for stopping the
/// bluetooth functionality
static mut SHUTDOWN: AtomicBool = AtomicBool::new(false);

/// this function starts a loop which empties the
/// queue of LogUpdates, formats them, and if the
/// correct bluetooth device is near by, sends Warn
/// and Error variants as a message to the user's device
pub async fn start(queue: Receiver<LogUpdate>, log_out_log: LogPipe) {
    log_out_log.info("started bluetooth logger", Job::LogOut);
    'send_log_update: loop {
        unimplemented!()
    }
}

pub fn stop() {
    unsafe { SHUTDOWN.store(true, Ordering::SeqCst) }
}
