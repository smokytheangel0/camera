//#![no_std]
use crate::log::LogPipe;
// WE SHOULD DEFINITELY USE MPSC FOR THE ENDS HERE,
// SO THAT WE CAN THREAD A BUNCH OF HANDLES TO CALLBACKS
// VALGRIND SAYS SO
use crate::queue::{AudioUpdate, Sender};

#[cfg(feature = "std")]
extern crate std;

use core::sync::atomic::AtomicBool;
static mut INITIALIZED: AtomicBool = AtomicBool::new(false);

static mut MICROPHONE_LOG: MaybeUninit<LogPipe> = MaybeUninit::<LogPipe>::uninit();

use core::mem::MaybeUninit;
static mut TO_AUDIO_COMPUTE: MaybeUninit<Sender<AudioUpdate>> =
    MaybeUninit::<Sender<AudioUpdate>>::uninit();

use std::string::String;
static mut DEVICE_NAME: MaybeUninit<String> = MaybeUninit::<String>::uninit();


/// this function sets up and begins streaming data from
/// a USB or analogue microphone, currently making use
/// of the linux audio server layer, it then sends each
/// audio frame down the queue to the audio processing
/// functions
pub async fn start(
    to_audio_compute: Sender<AudioUpdate>,
    microphone_log: LogPipe,
) {
    use crate::log::Job;
    unsafe {
        MICROPHONE_LOG.write(microphone_log);
    }
    unsafe {
        TO_AUDIO_COMPUTE.write(to_audio_compute);
    }
    let this_microphone_log;
    unsafe {
        
        this_microphone_log = MICROPHONE_LOG
            .assume_init_read()
            .new_task_log();
    }
    this_microphone_log.info("started audio input", Job::AudioInput);

    match has_side_effects::get_device() {
        Some(device) => {
            has_side_effects::use_stream(device);
            this_microphone_log.info("fell through use_stream to busy loop", Job::AudioInput);
            loop{}
            // this should not terminate (might if the device is unplugged)
        }
        None => panic!("failed to get an input device from cpal"),
    };
}

mod has_side_effects {
    use std::format;

    use cpal::traits::{DeviceTrait, HostTrait};
    use cpal::Device;

    use super::{DEVICE_NAME, INITIALIZED};

    pub fn get_device() -> Option<Device> {
        use core::sync::atomic::Ordering;
        use crate::log::Job;

        let this_microphone_log;
        unsafe {
            this_microphone_log = super::MICROPHONE_LOG
                .assume_init_read()
                .new_task_log();
        }

        let host = cpal::default_host();

        match host.default_input_device() {
            Some(device) => {
                this_microphone_log.info(
                    &format!(
                        "got device: {}",
                        &device.name().expect("failed to retrieve device name")
                    ),
                    Job::AudioInput,
                );
                unsafe {
                    super::DEVICE_NAME.write(device.name().unwrap());
                }
                unsafe{INITIALIZED.store(true, Ordering::SeqCst);}
                return Some(device);
            }
            None => {
                this_microphone_log
                    .debug(&format!("no audio input device found",));
                return None;
            }
        }
    }

    pub fn use_stream(device: Device) {
        // CPAL/examples/feedback.rs works
        // so it might be a good place to figure out
        // how to get this mic pushing frames
        use crate::log::Job;
        use cpal::traits::StreamTrait;
        use std::time::Duration;

        let mut this_microphone_log;
        unsafe {
            this_microphone_log = super::MICROPHONE_LOG
                .assume_init_read()
                .new_task_log();
        }

        let microphone_stream_log = this_microphone_log.new_task_log();
        let microphone_error_log = this_microphone_log.new_task_log();
        let config: cpal::StreamConfig = match device.default_input_config() {
            Ok(config) => config,
            Err(err) => panic!("could not configure input stream"),
        }
        .into();
        this_microphone_log
            .info("got default input stream config", Job::AudioInput);

        let input_stream =
            match device.build_input_stream(&config, got_data, got_err, None) {
                Ok(stream) => stream, //block on call backs
                Err(err) => panic!("building the input stream returned {err}"),
            };

        this_microphone_log.info("built audio input stream", Job::AudioInput);

        match input_stream.play() {
            //must busy loop to keep from dropping stream
            //should probably use a shutdown atomic
            Ok(_) => {
                this_microphone_log
                    .info("started audio input stream", Job::AudioInput);
                loop {
                    std::thread::sleep(Duration::new(0, 1000));
                }
            }
            Err(err) => panic!("there was an error starting the stream: {err}"),
        }
    }

    pub fn got_data(data: &[f32], _: &cpal::InputCallbackInfo) {
        use crate::log::Job;
        use crate::queue::AudioUpdate;
        use core::sync::atomic::Ordering;

        let this_microphone_log;
        unsafe {
            this_microphone_log = super::MICROPHONE_LOG
                .assume_init_read()
                .new_task_log();
        }
        this_microphone_log.info("got audio input frame", Job::AudioInput);

        //now we need to pass the input frame to audio compute, using an AudioUpdate
        if unsafe{INITIALIZED.load(Ordering::SeqCst)} {
            let update = AudioUpdate {
                timestamp: crate::get_timestamp(),
                data: data.to_vec(),
                name: unsafe { DEVICE_NAME.assume_init_read() },
            };
            this_microphone_log
                .info("packed AudioUpdate with new frame", Job::AudioInput);
    
            unsafe {
                match super::TO_AUDIO_COMPUTE.assume_init_read().enqueue(update) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("failed to enqueue the new update: {:?}", err)
                    }
                };
            }
            this_microphone_log
                .info("pushed new audio update to audio compute", Job::AudioInput);    
        }
    }

    pub fn got_err(err: cpal::StreamError) {
        panic!("the audio input stream had an error: {err}")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn _this() {}
}

/*
        macro_rules! input_stream {
            ($sample:ty, |$x:ident| $convert:expr) => {
                device.build_input_stream(
                    &config,
                    move |data: &[$sample], _: &InputCallbackInfo| {
                        for &$x in data {
                            let s = $convert;
                            microphone_stream_log.info(
                                &format!("got an audio frame!: {:?}", s),
                                Job::AudioInput,
                            );

                        }
                    },
                    |err| panic!("failed to build the correct input stream: {err}"),
                    None,
                )
            };
        }

        let _stream = match format {
            SampleFormat::F32 => input_stream!(f32, |x| x as f64),
            SampleFormat::I16 => input_stream!(i16, |x| x as f64 / i16::MAX as f64),
            SampleFormat::U16 => input_stream!(u16, |x| x as f64 - u16::MAX as f64 / 2.0),
            SampleFormat::I8 => input_stream!(i8, |x| x as f64 / i8::MAX as f64),
            SampleFormat::I32 => input_stream!(i32, |x| x as f64 / i32::MAX as f64),
            SampleFormat::I64 => input_stream!(i64, |x| x as f64 / i64::MAX as f64),
            SampleFormat::U8 => input_stream!(u8, |x| x as f64 - u8::MAX as f64 / 2.0),
            SampleFormat::U32 => input_stream!(u32, |x| x as f64 - u32::MAX as f64 / 2.0),
            SampleFormat::U64 => input_stream!(u64, |x| x as f64 - u64::MAX as f64 / 2.0),
            SampleFormat::F64 => input_stream!(f64, |x| x),
            _ => panic!("Unsupported sample format"),
        }.expect("failed to match the input audio sample format");

*/
