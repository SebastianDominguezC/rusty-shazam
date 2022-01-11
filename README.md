# Rusty-Shazam

Rusty-Shazam is a shazam-like app built with Rust.

It has a custom fingerprinting algorithm that uses the Fast Fourier Transform.

You can record and stop the recording. Once the recording stops, it tries to hash the recording and look for any matching fingerprints.

Some concepts used for the project:

- Fast Fourier Transform
- Audio Recording
- Data processing in Rust
- Futures in Rust
- Asynchronous data processing
- REST server
- MongoDB Database
- GUI in Rust

## Songs

Songs were taken from [this free dataset](https://www.kaggle.com/imsparsh/fma-free-music-archive-small-medium).

The small 8GB data set was used, with around 8_000 song samples of 30 seconds each.

Data was analyzed to an output of around 202_000 unique hashes for moments in each song.

Some songs could not be propperly analyzed, resulting in around only 7_000 songs in the fingerprint database.

## Example

You can try out this app by seeing the Get Started section!

You can also see the video of the app working [here](./archive/showcase.mp4)

## Future Improvements

### Server Side

- Removing silences from beggining and ending of songs
- Giving a propper error range for each ID in a fingerprint

### Client Side

- Probability weighing algorithm for all results returned
- Better filtering and limiting of results
- Better UI when recording (animations)
- Better UI when rendering results

## Get started

Download the code and then in the root directory:

Run the app unoptimized:

```
cargo run
```

Run the app with optimizations:

```
cargo run --release
```

## Dependencies

- iced - for GUI
- cpal - recording
- rustfft - FFT
- surf, async-std - for all async and server requests

## Server

The songs can be accesses through my custom server, which connects to the DB that contains all fingerprints (around 900 fingerprints of 30 song samples)
