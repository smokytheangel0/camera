use crate::hardware::{
    battery::{MainBattery, SecondaryBattery},
    storage::{MainStorage, RemovableStorage},
};

use crate::queue::Receiver;
/// This is where the ui thread will retrieve
/// state from the ui queue and display the
/// state as follows, a full screen blue
/// window will indicate that the program
/// has started and has detected the secondary
/// usb camera. A grey full screen window will
/// indicate that the camera has not been detected
/// and that the program will record from the
/// main (webcam and mic) audio and video sources
/// until it detects a secondary camera.
/// In both of these states, frames are being recorded
/// to on disk storage, even though a removable has
/// not been inserted. A green full screen window
/// will be used to indicate that removable storage
/// was detected and the program has begun recording
/// to that storage device. A full screen green with blue
/// bars will indicate that new storage is being switched
/// to and the user should wait. In the case of recording
/// to a removable storage device, yellow will indicate
/// that the removable is almost full. This should be
/// calculated to give an hour of recording time before
/// the storage device becomes full. An orange full
/// screen window will indicate that the main disk
/// is nearing capacity. This should be calculated to
/// give 24 hours worth of recording space before the
/// main disk is full. A red full screen window will
/// indicate that either the removable storage or the
/// main disk has reached capacity. Holding the space
/// bar will cause the full screen display of video
/// frames copied into the ui queue. These frames are
/// normally discarded to save cycles being used to
/// display them.
use winit;

/// mock frame placeholder
pub struct Frame {}

/// This struct is retrieved from the queue by the
/// UI thread, so that it can display the most current
/// information about the system to the user
/// this should be in the queue page
pub struct ViewUpdate {
    /// this indicates the current status of the system
    status: Status,
    /// this is the most recent frame from the video source
    frame: Frame,
}

/// this carries all of the information
/// the UI thread needs in order to inform
/// the user of the current system state
enum Status {
    /// this indicates that the removable storage
    /// and the secondary camera is disconnected
    MainDiskAndMainCam,
    /// this indicates that the removable storage
    /// is disconnected but the secondary camera
    /// is connected and being used
    MainDiskAndSecondaryCam,
    /// this indicates that the removable storage
    /// is connected and being used, and that the
    /// secondary camera is disconnected
    RemovableDiskAndMainCam,
    /// this indicates that the removable storage
    /// is connected and being used, and that the
    /// secondary camera is connected and being used
    RemovableDiskAndSecondaryCam,
}

/// this carries specific details
/// about the storage and battery
/// so that it can be used by
/// the UI thread to show the
/// state of the system
struct MainDiskAndMainCam {
    main_storage: MainStorage,
    removable_storage: RemovableStorage,
    main_battery: MainBattery,
    secondary_battery: SecondaryBattery,
}

/// this carries specific details
/// about the storage and battery
/// so that it can be used by
/// the UI thread to show the
/// state of the system
struct MainDiskAndSecondaryCam {
    main_storage: MainStorage,
    removable_storage: RemovableStorage,
    main_battery: MainBattery,
    secondary_battery: SecondaryBattery,
}

/// this carries specific details
/// about the storage and battery
/// so that it can be used by
/// the UI thread to show the
/// state of the system
struct RemovableDiskAndMainCam {
    main_storage: MainStorage,
    removable_storage: RemovableStorage,
    main_battery: MainBattery,
    secondary_battery: SecondaryBattery,
} //this scenario is not covered

/// this carries specific details
/// about the storage and battery
/// so that it can be used by
/// the UI thread to show the
/// state of the system
struct RemovableDiskAndSecondaryCam {
    main_storage: MainStorage,
    removable_storage: RemovableStorage,
    main_battery: MainBattery,
    secondary_battery: SecondaryBattery,
}
use crate::log::{Job, LogPipe};
pub async fn start(queue: Receiver<ViewUpdate>, ui_log: LogPipe) {
    ui_log.info("started UI", Job::UI);
}
