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
/// we should create an impl for each hardware
/// component that says 'start' and takes
/// a send handle to its queue(s)
struct Hardware {}
