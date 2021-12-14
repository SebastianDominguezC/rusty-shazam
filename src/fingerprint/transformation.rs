use rustfft::num_complex::Complex32;
use rustfft::FftPlanner;

fn convert_freq_mag(data: &[f32]) -> Vec<f32> {
    // Convert data to frecuencies in complex form
    let mut frecuencies: Vec<Complex32> = data.iter().map(|f| Complex32::from(f)).collect();

    // Convert data to frecuency domain
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(frecuencies.len());
    fft.process(&mut frecuencies);

    // Get magnitude of frecuencies
    frecuencies.iter().map(|f| f.norm()).collect()
}

pub fn build_spectrum(divisions: i32, mut data: Vec<f32>) -> Option<Vec<Vec<f32>>> {
    let remainder = data.len() as i32 % divisions;

    // Remove data that wont be used
    let _ = data.split_off(data.len() - remainder as usize);
    let remainder = data.len() as i32 % divisions;

    if remainder != 0 {
        println!("Some error happened at build spectrum");
        return None;
    }

    let mut spectrum = vec![];
    let mut i = 0;

    // Loop data by extracts and perform fft
    while i < data.len() {
        let j = (i + divisions as usize) as usize;
        let extract = &data[i..j];
        let mag = convert_freq_mag(extract);
        spectrum.push(mag);
        i = j;
    }
    Some(spectrum)
}
