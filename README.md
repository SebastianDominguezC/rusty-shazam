# Rusty-Shazam

Rusty Shazam is a shazam-like app built with Rust

It has a custom fingerprinting algorithm that uses the Fast Fourier Transform.

## Get started

To run the app:

```
cargo run
```

## Dependencies

- iced - for GUI
- cpal - recording
- rustfft - FFT
- hyper, tokio, futures - for all async and server requests

## Server

The songs can be accesses through my custom server, which connects to the DB that contains all fingerprints (around 900 fingerprints of 30 song samples)
