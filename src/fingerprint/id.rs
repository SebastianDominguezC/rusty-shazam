use crate::fingerprint::transformation::build_spectrum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub id2: i32,
    pub id1: i32,
    pub id3: i32,
    pub id4: i32,
    pub id5: i32,
}

impl Fingerprint {
    pub fn new(id1: i32, id2: i32, id3: i32, id4: i32, id5: i32) -> Self {
        Self {
            id1,
            id2,
            id3,
            id4,
            id5,
        }
    }
}

pub fn get_fingerprints(divs: i32, spectrum: Vec<f32>) -> Option<Vec<Fingerprint>> {
    let spectrum = build_spectrum(divs, spectrum);
    if let Some(s) = spectrum {
        let fingerprints = convert_to_fingerprints(s);

        if let Some(fs) = fingerprints {
            return Some(fs);
        }
    }
    None
}

fn convert_to_fingerprints(spectrum: Vec<Vec<f32>>) -> Option<Vec<Fingerprint>> {
    let mut fingerprints = vec![];
    for mag in spectrum.iter() {
        if mag.len() < 2205 {
            return None;
        }
        let r40 = &mag[0..40];
        let r80 = &mag[40..80];
        let r120 = &mag[80..120];
        let r180 = &mag[120..180];
        let r300 = &mag[180..300];

        let r40 = get_highest_freq(r40);
        let r80 = get_highest_freq(r80);
        let r120 = get_highest_freq(r120);
        let r180 = get_highest_freq(r180);
        let r300 = get_highest_freq(r300);

        let fingerprint = Fingerprint::new(r40, r80, r120, r180, r300);
        fingerprints.push(fingerprint);
    }
    Some(fingerprints)
}

fn get_highest_freq(range: &[f32]) -> i32 {
    let mut highest = range[0];
    for val in range {
        if val > &highest {
            highest = *val;
        }
    }
    highest.ceil() as i32
}
