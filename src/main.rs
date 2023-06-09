mod compute;
mod hardware;
mod log;
mod queue;
mod ui;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    // set up logging input
    println!("creating new logout queue !>");
    let (log_out_queue_receiver, log_out_queue) = queue::LogOut::new();

    println!("creating new log storage queue !>");
    let (log_storage_queue_receiver, log_storage_queue) = queue::LogStorage::new();

    println!("creating logging thread !>");
    let log_thread = thread::spawn(move || {
        println!("logging thread started !>");
        let tasks = pasts::Executor::default();
        println!("starting the log out task !>");
        tasks.spawn_boxed(async move {
            println!("log out task started !>");
            hardware::bluetooth::start(log_out_queue_receiver);
        });
        println!("starting the log storage task !>");
        tasks.spawn_boxed(async move {
            println!("log storage task started !>");
            hardware::storage::log_start(log_storage_queue_receiver);
        });
    });
    println!("setting up proper logging facilities !>");
    log::set_pipe(log_out_queue, log_storage_queue);

    log::info("creating new audio in queue !>");
    let (audio_in_queue, audio_in_queue_sender) = queue::AudioIn::new();

    log::info("creating new input queue for audio compute !>");
    let (audio_compute_out_receiver, audio_compute_out) = queue::ToAudioCompute::new();

    log::info("creating new output queue for audio compute !>");
    let (audio_compute_in, audio_compute_in_sender) = queue::FromAudioCompute::new();

    log::info("creating new audio storage queue !>");
    let (audio_storage_queue_receiver, audio_storage_queue) = queue::AudioStorage::new();

    log::info("creating audio thread !>");
    let audio_thread = thread::spawn(move || {
        log::info("audio thread started !>");
        let tasks = pasts::Executor::default();
        log::info("starting the microphone task !>");
        tasks.spawn_boxed(async move {
            log::info("microphone task started !>");
            hardware::audio::start(audio_in_queue_sender);
        });
        log::info("starting the audio compute task !>");
        tasks.spawn_boxed(async move {
            log::info("audio compute task started !>");
            compute::audio::start(audio_compute_out_receiver, audio_compute_in_sender);
        });
        log::info("starting the audio storage task !>");
        tasks.spawn_boxed(async move {
            log::info("audio storage task started !>");
            hardware::storage::audio_start(audio_storage_queue_receiver);
        });
    });

    log::info("creating new video in queue !>");
    let (mut video_in_queue, video_in_queue_sender) = queue::VideoIn::new();

    log::info("creating new input queue for video compute !>");
    let (video_compute_out_receiver, video_compute_out) = queue::ToVideoCompute::new();

    log::info("creating new output queue for video compute !>");
    let (video_compute_in, video_compute_in_sender) = queue::FromVideoCompute::new();

    log::info("creating new video storage queue !>");
    let (video_storage_queue_receiver, video_storage_queue) = queue::VideoStorage::new();

    log::info("creating video thread !>");
    let video_thread = thread::spawn(move || {
        log::info("video thread started !>");
        let tasks = pasts::Executor::default();
        log::info("starting camera task !>");
        tasks.spawn_boxed(async move {
            log::info("camera task started !>");
            hardware::camera::start(video_in_queue_sender);
        });

        log::info("starting the video compute task !>");
        tasks.spawn_boxed(async move {
            log::info("video compute task started !>");
            compute::video::start(video_compute_out_receiver, video_compute_in_sender);
        });

        log::info("starting the video storage task !>");
        tasks.spawn_boxed(async move {
            log::info("video storage task started !>");
            hardware::storage::video_start(video_storage_queue_receiver);
        });
    });

    // Last set up the ui queue for the user
    log::info("creating new input queue for the UI !>");
    let (view_out_queue_receiver, view_out_queue) = queue::ViewOut::new();

    log::info("creating the UI thread !>");
    thread::spawn(move || {
        log::info("UI thread started !>");
        let tasks = pasts::Executor::default();
        log::info("starting the UI task !>");
        tasks.spawn_boxed(async move {
            log::info("UI task started !>");
            ui::start(view_out_queue_receiver);
        });
    });
    // Begin popping new frames from the in pipelines
    // processing them
    // then pushing them onto the storage pipelines
    sleep(Duration::from_millis(1000));

    if let Ok(_frame) = video_in_queue.try_dequeue() {
        //this is really nice it just magicked the VideoUpdate out of the Some
        log::info("recieved a video frame in main!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _this() {}
}
