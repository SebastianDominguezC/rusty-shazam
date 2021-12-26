use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct Recorder {
    stream: cpal::Stream,
    data: Arc<Mutex<Vec<f32>>>,
}

impl Recorder {
    fn new() -> Self {
        // Configuration
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("no output device available");
        let mut supported_configs_range = device
            .supported_input_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let sample_format = supported_config.sample_format();
        let config: cpal::StreamConfig = supported_config.into();

        // Communication channels
        let (sender, receiver) = mpsc::channel();

        // Data
        let song_data = vec![];
        let song_data = Arc::new(Mutex::new(song_data));
        let song_data_clone = Arc::clone(&song_data);

        // Stream
        let stream = device
            .build_input_stream_raw(
                &config,
                sample_format,
                move |data, _: &cpal::InputCallbackInfo| {
                    // Gets data and sends it to receiver
                    let inp = data.as_slice::<f32>().unwrap().to_vec();
                    sender.send(inp).unwrap();
                },
                |err| {
                    println!("{:?}", err);
                },
            )
            .unwrap();
        stream.pause().unwrap();

        // Receiver function
        std::thread::spawn(move || {
            let song_data = song_data_clone;
            for mut m in receiver.iter() {
                song_data.lock().unwrap().append(&mut m);
            }
        });

        Self {
            stream,
            data: song_data,
        }
    }

    pub fn start_recording(&mut self) {
        self.stream.play().expect("Could not start recording");
    }

    pub fn stop_recording(&mut self) {
        self.stream.pause().expect("Could not stop recording");
    }

    pub fn flush(&mut self) -> Vec<f32> {
        let data = self.data.lock().unwrap().clone();
        self.data.lock().unwrap().clear();
        data
    }
}

impl Default for Recorder {
    fn default() -> Self {
        Recorder::new()
    }
}
