/// In order to enable light logging
/// through bluetooth, we will need
/// to host as non discoverable,
/// use encryption if possible
/// send basic status through bluetooth
/// to feature phone
use crate::queue::{LogUpdate, Receiver};
pub async fn start(queue: Receiver<LogUpdate>) {
    println!("have started bluetooth task !>");
}
