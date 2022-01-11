use csv::{Position, Reader};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;

use crate::fingerprint::id::{get_fingerprints, Fingerprint};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SongData {
    author: String,
    name: String,
    duration: String,
    fingerprint: Fingerprint,
}
impl SongData {
    fn new(author: String, name: String, duration: String) -> Self {
        Self {
            author,
            name,
            duration,
            fingerprint: Fingerprint::new(0, 0, 0, 0, 0),
        }
    }

    fn set_fingerprint(&mut self, fingerprint: &Fingerprint) {
        self.fingerprint = fingerprint.clone();
    }
}

struct CvsData {
    ids: Vec<String>,
    records: Reader<File>,
}
impl CvsData {
    fn new(path: &str, start: u32, end: u32) -> Self {
        let file = File::open(path).expect("Could create new CVS data");
        let rds = Reader::from_reader(file);
        Self {
            ids: CvsData::read_ids(path, start, end),
            records: rds,
        }
    }
    fn read_ids(path: &str, start: u32, end: u32) -> Vec<String> {
        let file = File::open(path).expect("Could not read ids");
        let mut rds = Reader::from_reader(file);
        let mut ids = vec![];

        let start: u32 = format!("{}000", start).parse().unwrap();
        let end: u32 = format!("{}999", end).parse().unwrap();

        for r in rds.deserialize() {
            let record: Track = r.unwrap();

            if record.track_id.parse::<u32>().unwrap() > start
                && record.track_id.parse::<u32>().unwrap() < end
            {
                ids.push(record.track_id);
            }
        }
        ids
    }
    fn find_pos_str_id(&self, id: &str) -> Option<usize> {
        let id = id.parse::<u32>().unwrap().to_string();
        let i = self.ids.iter().position(|v| v == &id);
        i
    }
    fn find_data_by_pos(&mut self, pos: usize) -> Option<SongData> {
        let mut iter = self.records.records();
        let data = iter.nth(pos)?;
        let p = Position::new();
        if let Ok(_) = self.records.seek(p) {
            if let Ok(data) = data {
                let name = data.get(2)?;
                let author = data.get(5)?;
                let duration = data.get(22)?;

                return Some(SongData::new(
                    author.to_string(),
                    name.to_string(),
                    duration.to_string(),
                ));
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Track {
    track_id: String,
}

pub fn read_mp3(path: String) -> Option<Vec<f32>> {
    let data = std::fs::read(path);
    if let Ok(data) = data {
        if let Ok((_, samples)) = puremp3::read_mp3(&data[..]) {
            let mut mean: Vec<f32> = vec![];
            for (left, right) in samples {
                let sum = (left + right) as f32;
                let div = sum / 2.0;
                mean.push(div);
            }
            return Some(mean);
        }
        return None;
    }
    None
}

pub fn delete_files() {
    let data = std::fs::read_to_string("./songs/fails.txt").unwrap();
    let mut count = 0;
    for line in data.lines() {
        if let Err(e) = fs::remove_file(line) {
            println!("failed to remove: {}", line);
            println!("{}", e);
        } else {
            count += 1;
            println!("removed: {}", line);
        }
    }
    println!("data deleted: {}", count);
}

pub async fn run() {
    let args: Vec<String> = env::args().collect();

    let num: u32 = args[1].parse().unwrap();

    if num == 0 {
        println!("deleting files");
        delete_files();
    } else {
        main().await;
    }
}

// LOOP EACH FILE, THEN SERACH ID FOR AUTHOR, TITLE, ETC... !!!!
pub async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let num: u32 = args[1].parse().unwrap();
    let start = 15 * (num - 1);
    let end = start + 14;

    let path = format!("./songs/fma_small_{}/", num);
    let paths = fs::read_dir(path).unwrap();
    let mut cvs_data = CvsData::new("songs/raw_tracks.csv", start, end);
    println!("{}", cvs_data.ids.len());

    for path in paths {
        let path = path.unwrap().path();
        if path.to_str().unwrap().contains(".DS_Store") {
            continue;
        }

        let albums;
        if let Ok(albs) = fs::read_dir(path) {
            albums = albs;
        } else {
            continue;
        }

        for song in albums {
            let song = song.unwrap().path();
            let song = song.to_str().unwrap().to_string();

            println!("{}", song);

            let len: usize = song.len();

            let id = &song[len - 10..len - 4];

            let pos = cvs_data.find_pos_str_id(id);

            if let None = pos {
                continue;
            }

            let pos = pos.unwrap();
            let data = cvs_data.find_data_by_pos(pos);

            if let None = data {
                println!("No data was found");
                continue;
            }

            let mut data = data.unwrap();
            let mono = read_mp3(song);

            if let Some(song_freqs) = mono {
                let fingerprints = get_fingerprints(44100, song_freqs);
                match fingerprints {
                    Some(fs) => {
                        for f in fs.iter() {
                            data.set_fingerprint(f);
                            post_data(&data).await;
                        }
                    }
                    None => {
                        println!("No fingerprints can be made...");
                    }
                }
            }
        }
    }

    println!("finito");
}

async fn post_data(data: &SongData) {
    let json = Data::new(
        data.name.clone(),
        data.author.clone(),
        data.duration.clone(),
        data.fingerprint.id1,
        data.fingerprint.id2,
        data.fingerprint.id3,
        data.fingerprint.id4,
        data.fingerprint.id5,
    );

    let uri = "http://sebs-playground.herokuapp.com/api/v1/fingerprints";
    let req = surf::post(uri).body_json(&json);

    // If okay request built...
    if let Ok(r) = req {
        // Make request
        if let Ok(res) = r.await {
            // Parse body and extract data
            if res.status() != 200 {
                // println!("{:?}", json);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    author: String,
    duration: String,
    id1: i32,
    id2: i32,
    id3: i32,
    id4: i32,
    id5: i32,
}

impl Data {
    fn new(
        name: String,
        author: String,
        duration: String,
        id1: i32,
        id2: i32,
        id3: i32,
        id4: i32,
        id5: i32,
    ) -> Self {
        Self {
            name,
            author,
            duration,
            id1,
            id2,
            id3,
            id4,
            id5,
        }
    }
}
