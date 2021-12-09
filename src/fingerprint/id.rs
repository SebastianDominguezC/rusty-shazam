use crate::fingerprint::transformation::build_spectrum;

pub fn get_fingerprints(divs: i32, spectrum: Vec<f32>) -> Option<Vec<String>> {
    let spectrum = build_spectrum(divs, spectrum);
    if let Some(s) = spectrum {
        let fingerprints = convert_to_fingerprints(s);

        if let Some(fs) = fingerprints {
            return Some(fs);
        }
    }
    None
}

fn convert_to_fingerprints(spectrum: Vec<Vec<f32>>) -> Option<Vec<String>> {
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

        let fingerprint = format!("{:X} {:X} {:X} {:X} {:X}", r40, r80, r120, r180, r300);
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
