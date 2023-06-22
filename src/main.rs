mod compute;
mod hardware;
mod log;
mod queue;
mod ui;
use crate::log::{Job, LogPipe};

use core::time::Duration;

use std::thread;
use std::thread::sleep;

/*
       YAY THREADFUL LOGGING
*/
static DEBUG_LOGGING: bool = false;

fn main() {
    if DEBUG_LOGGING {
        println!("creating new log out queue !>");
    }
    let (log_out_queue_receiver, log_out_queue) = queue::LogOut::new();

    if DEBUG_LOGGING {
        println!("creating new log storage queue !>");
    }
    let (log_storage_queue_receiver, log_storage_queue) =
        queue::LogStorage::new();

    if DEBUG_LOGGING {
        println!("setting up proper logging facilities !>");
    }
    let mut log = LogPipe::set_pipe(log_out_queue, log_storage_queue);

    log.info("creating logging thread", Job::LogSetup);
    let mut logging_thread_log = log.new_thread_log();
    let logging_thread = thread::spawn(move || {
        logging_thread_log.info("logging thread started", Job::LogSetup);

        let tasks = pasts::Executor::default();
        let log_out_log = logging_thread_log.new_task_log();
        let log_storage_log = logging_thread_log.new_task_log();

        tasks.clone().block_on(async move {
            tasks.spawn_boxed(async {
                log_out_log.info("log out task started", Job::LogOut);
                hardware::bluetooth::start(log_out_queue_receiver, log_out_log)
                    .await;
            });
            tasks.spawn_boxed(async {
                log_storage_log
                    .info("log storage task started", Job::LogStorage);
                hardware::storage::log_start(
                    log_storage_queue_receiver,
                    log_storage_log,
                )
                .await;
            });
        });
    });

    log.info("creating new audio in queue", Job::AudioSetup);
    let (audio_in_queue, audio_in_queue_sender) = queue::AudioIn::new();

    log.info(
        "creating new input queue for audio compute",
        Job::AudioSetup,
    );
    let (audio_compute_out_receiver, audio_compute_out) =
        queue::ToAudioCompute::new();

    log.info(
        "creating new output queue for audio compute",
        Job::AudioSetup,
    );
    let (audio_compute_in, audio_compute_in_sender) =
        queue::FromAudioCompute::new();

    log.info("creating new audio storage queue", Job::AudioSetup);
    let (audio_storage_queue_receiver, audio_storage_queue) =
        queue::AudioStorage::new();

    log.info("creating audio thread", Job::AudioSetup);
    let mut audio_thread_log = log.new_thread_log();
    let audio_thread = thread::spawn(move || {
        audio_thread_log.info("audio thread started", Job::AudioSetup);

        let tasks = pasts::Executor::default();
        let microphone_log = audio_thread_log.new_task_log();
        let audio_compute_log = audio_thread_log.new_task_log();
        let audio_storage_log = audio_thread_log.new_task_log();

        tasks.clone().block_on(async move {
            tasks.spawn_boxed(async {
                microphone_log.info("microphone task started", Job::AudioSetup);
                hardware::audio::start(audio_in_queue_sender, microphone_log)
                    .await;
            });
            tasks.spawn_boxed(async {
                audio_compute_log
                    .info("audio compute task started", Job::AudioSetup);
                compute::audio::start(
                    audio_compute_out_receiver,
                    audio_compute_in_sender,
                    audio_compute_log,
                )
                .await;
            });
            tasks.spawn_boxed(async {
                audio_storage_log
                    .info("audio storage task started", Job::AudioSetup);
                hardware::storage::audio_start(
                    audio_storage_queue_receiver,
                    audio_storage_log,
                )
                .await;
            });
        });
    });

    log.info("creating new video in queue", Job::VideoSetup);
    let (mut video_in_queue, video_in_queue_sender) = queue::VideoIn::new();

    log.info(
        "creating new input queue for video compute",
        Job::VideoSetup,
    );
    let (video_compute_out_receiver, video_compute_out) =
        queue::ToVideoCompute::new();

    log.info(
        "creating new output queue for video compute",
        Job::VideoSetup,
    );
    let (video_compute_in, video_compute_in_sender) =
        queue::FromVideoCompute::new();

    log.info("creating new video storage queue", Job::VideoSetup);
    let (video_storage_queue_receiver, video_storage_queue) =
        queue::VideoStorage::new();

    log.info("creating video thread", Job::VideoSetup);
    let mut video_thread_log = log.new_thread_log();
    let video_thread = thread::spawn(move || {
        video_thread_log.info("video thread started", Job::VideoSetup);

        let tasks = pasts::Executor::default();
        let camera_log = video_thread_log.new_task_log();
        let video_compute_log = video_thread_log.new_task_log();
        let video_storage_log = video_thread_log.new_task_log();

        tasks.clone().block_on(async move {
            tasks.spawn_boxed(async {
                camera_log.info("camera task started", Job::VideoSetup);
                hardware::camera::start(video_in_queue_sender, camera_log)
                    .await;
            });

            tasks.spawn_boxed(async {
                video_compute_log
                    .info("video compute task started", Job::VideoSetup);
                compute::video::start(
                    video_compute_out_receiver,
                    video_compute_in_sender,
                    video_compute_log,
                )
                .await;
            });

            tasks.spawn_boxed(async {
                video_storage_log
                    .info("video storage task started", Job::VideoSetup);
                hardware::storage::video_start(
                    video_storage_queue_receiver,
                    video_storage_log,
                )
                .await;
            });
        });
    });

    // Last set up the ui queue for the user
    log.info("creating new input queue for the UI", Job::UISetup);
    let (view_out_queue_receiver, view_out_queue) = queue::ViewOut::new();

    log.info("creating the UI thread", Job::UISetup);
    let mut ui_thread_log = log.new_thread_log();
    thread::spawn( move || {
        ui_thread_log.info("UI thread started", Job::UISetup);

        let tasks = pasts::Executor::default();
        let ui_log = ui_thread_log.new_task_log();

        tasks.clone().block_on(async move {
            tasks.spawn_boxed(async {
                ui_log.info("UI task started", Job::UISetup);
                ui::start(view_out_queue_receiver, ui_log).await;
            });
        });
    });
    // Begin popping new frames from the in pipelines
    // processing them
    // then pushing them onto the storage pipelines
    sleep(Duration::from_millis(1000));
    log.info("got past sleep", Job::Debug);

    /*
        SOMEHOW THIS ALL WORKS!
    */
    loop {
        if let Ok(_frame) = video_in_queue.try_dequeue() {
            //this is really nice it just magicked the VideoUpdate out of the Some
            log.info("recieved a video frame in main!", Job::Main);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _this() {}
}
