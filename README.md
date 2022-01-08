# Rusty-Shazam

Rusty-Shazam is a shazam-like app built with Rust.

It has a custom fingerprinting algorithm that uses the Fast Fourier Transform.

You can record and stop the recording. Once the recording stops, it tries to hash the recording and look for any matching fingerprints.

Some concepts used for the project:

- Fast Fourier Transform
- Recording
- Data processing in Rust
- Futures in Rust
- Asynchronous data processing
- REST server
- MongoDB Database

## Get started

To run the app:

```
cargo run
```

## Dependencies

- iced - for GUI
- cpal - recording
- rustfft - FFT
- surf, async-std - for all async and server requests

## Server

The songs can be accesses through my custom server, which connects to the DB that contains all fingerprints (around 900 fingerprints of 30 song samples)
