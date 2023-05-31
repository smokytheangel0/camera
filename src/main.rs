mod compute;
mod hardware;
mod queue;
mod ui;
use std::thread::sleep;
use std::time::Duration;
use whisk::Channel;

fn main() {
    let task = pasts::Executor::default();

    let log_out_queue = queue::LogOut::new();
    let log_out_queue_receiver = log_out_queue.clone();
    task.spawn_boxed(async {
        hardware::bluetooth::start(log_out_queue_receiver);
    });

    let log_storage_queue = queue::LogStorage::new();
    let log_storage_queue_receiver = log_storage_queue.clone();
    task.spawn_boxed(async {
        hardware::storage::log_start(log_storage_queue_receiver);
    });

    let audio_in_queue = queue::AudioIn::new();
    let audio_in_queue_sender = audio_in_queue.clone();
    task.spawn_boxed(async {
        hardware::audio::start(audio_in_queue_sender);
    });

    let audio_compute_out = queue::ToAudioCompute::new();
    let audio_compute_out_receiver = audio_compute_out.clone();

    let audio_compute_in = queue::FromAudioCompute::new();
    let audio_compute_in_sender = audio_compute_in.clone();

    task.spawn_boxed(async {
        compute::audio::start(audio_compute_out_receiver, audio_compute_in_sender);
    });

    let audio_storage_queue = queue::AudioStorage::new();
    let audio_storage_queue_receiver = audio_storage_queue.clone();
    task.spawn_boxed(async {
        hardware::storage::audio_start(audio_storage_queue_receiver);
    });
    let video_in_queue = queue::VideoIn::new();
    let video_in_queue_sender = video_in_queue.clone();
    task.spawn_boxed(async {
        hardware::camera::start(video_in_queue_sender);
    });

    let video_compute_out = queue::ToVideoCompute::new();
    let video_compute_out_receiver = video_compute_out.clone();

    let video_compute_in = queue::FromVideoCompute::new();
    let video_compute_in_sender = video_compute_in.clone();

    task.spawn_boxed(async {
        compute::video::start(video_compute_out_receiver, video_compute_in_sender);
    });

    let video_storage_queue = queue::VideoStorage::new();
    let video_storage_queue_receiver = video_storage_queue.clone();
    task.spawn_boxed(async {
        hardware::storage::video_start(video_storage_queue_receiver);
    });

    // Last set up the ui queue for the user
    let view_out_queue = queue::ViewOut::new();
    let view_out_queue_receiver = view_out_queue.clone();
    task.spawn_boxed(async {
        ui::start(view_out_queue_receiver);
    });

    // Begin popping new frames from the in pipelines
    // processing them
    // then pushing them onto the storage pipelines
    sleep(Duration::from_millis(1000));

    if let Some(_frame) = video_in_queue.poll_next() {
        //this is really nice it just magicked the VideoUpdate out of the Some
        println!("recieved a video frame in main!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _this() {}
}
