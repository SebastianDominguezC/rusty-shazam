use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use cpal::{Data, Sample, SampleFormat};
use rustfft::num_complex::Complex32;
use rustfft::FftPlanner;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");
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

    println!("{:?}", supported_config);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    println!("{:?}", config);

    let (s, r) = mpsc::channel();

    let song_data = vec![];
    let song_data = Arc::new(Mutex::new(song_data));
    let song_data_clone = Arc::clone(&song_data);

    std::thread::spawn(move || {
        let song_data = song_data_clone;
        for mut m in r.iter() {
            song_data.lock().unwrap().append(&mut m);
        }
    });

    let stream = device
        .build_input_stream_raw(
            &config,
            sample_format,
            move |data, _: &cpal::InputCallbackInfo| {
                let inp = data.as_slice::<f32>().unwrap().to_vec();
                s.send(inp).unwrap();
            },
            |err| {
                println!("{:?}", err);
            },
        )
        .unwrap();

    let time = 50;
    let divisions = 200;

    stream.play().unwrap();

    let mut data = vec![];
    for _ in 0..divisions {
        let mut sd = vec![];
        for data in song_data.lock().unwrap().iter() {
            sd.push(Complex32::from(data.clone()));
        }
        println!("{:?}", sd.len());
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(sd.len());
        fft.process(&mut sd);
        let mut nd = vec![];
        for data in sd.iter() {
            nd.push(data.norm());
        }
        song_data.lock().unwrap().clear();
        std::thread::sleep_ms(time);
    }
    println!("{}", data.len());

    stream.pause().unwrap();
}
