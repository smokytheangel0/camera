pub mod audio;
pub mod battery;
pub mod bluetooth;
pub mod camera;
pub mod memory;
pub mod storage;

/// This is where we will describe the various
/// kinds of hardware interfaces that we will be
/// using the communicate with both real and mock
/// hardware.
/// there should be a function for each hardware
/// component called 'start' which takes
/// a send handle to its queue(s)
struct Hardware {}
