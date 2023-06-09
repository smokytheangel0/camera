use crate::queue::{Sender, VideoUpdate};
/// This is where we will retrieve video
/// from either a USB source or a mock source. The timestamp
/// and the metadata important for making audio and video
/// adjustments will be packaged along with the frame in
/// either an AudioFrame struct or a VideoFrame struct

// in order to access the usb devices the code must run as root
// we will probably be operating on MJPEG images without converting
// them to raw, or requesting them from the device in raw.
// this will allow us alot more Memory space for queues
// and hopefully they will cycle quick enough to not
// create a storage deficit (more frames being produced per
// second than are being stored per second)

// this struct describes the camera that is mounted
// above the laptop screen, and attached internally
pub struct X102baCamera {
    /// this value was hardcoded
    /// DO NOT MUTATE
    vendor_id: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    product_id: u64,
    /// this is the handle to the
    /// device, given to us by the
    /// operating system
    dev_id: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    best_resolution: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    best_fps: u64,
}

/// this method returns a struct
/// with data about the camera
impl X102baCamera {
    fn new(dev_id: u64) -> X102baCamera {
        X102baCamera {
            vendor_id: 3034,
            product_id: 22019,
            dev_id,
            best_resolution: 1280 * 720,
            best_fps: 30,
        }
    }
}

/// this struct describes the USB
/// camera that says 3.6mm on the lens
pub struct WideBandCamera {
    /// this value was hardcoded
    /// DO NOT MUTATE
    vendor_id: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    product_id: u64,
    /// this is the handle to the
    /// device, given to us by the
    /// operating system
    dev_id: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    best_resolution: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    best_fps: u64,
    //focal_length: "1/2.5''"
}

/// this method returns a struct
/// with data about the camera
impl WideBandCamera {
    fn new(dev_id: u64) -> WideBandCamera {
        WideBandCamera {
            vendor_id: 3804,
            product_id: 12416,
            dev_id,
            best_resolution: 2592 * 1933,
            best_fps: 30,
        }
    }
}

/// this struct describes the USB
/// camera that has no markings
/// but has small wires on the front
/// and a light sensor (black tube)
pub struct IRSensitiveCamera {
    /// this value was hardcoded
    /// DO NOT MUTATE
    vendor_id: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    product_id: u64,
    /// this is the handle to the
    /// device, given to us by the
    /// operating system
    dev_id: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    best_resolution: u64,
    /// this value was hardcoded
    /// DO NOT MUTATE
    best_fps: u64,
}

/// this method returns a struct
/// with data about the camera
impl IRSensitiveCamera {
    fn new(dev_id: u64) -> IRSensitiveCamera {
        IRSensitiveCamera {
            vendor_id: 3141,
            product_id: 25449,
            best_resolution: 1920 * 1080,
            best_fps: 30,
            dev_id,
        }
    }
}
use log::info;
pub fn start(mut queue: Sender<VideoUpdate>) {
    for _ in 1..=10 {
        let new_mock_frame = VideoUpdate {};
        queue.enqueue(new_mock_frame);
        info!("just sent a frame from camera to main!");
    }
}
