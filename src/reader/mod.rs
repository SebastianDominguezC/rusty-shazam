use csv::{Position, Reader};
use hyper::Client;
use hyper::{Body, Method, Request};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;

use crate::fingerprint::id::{get_fingerprints, Fingerprint};

#[derive(Debug)]
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
    fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let rds = Reader::from_reader(file);
        Self {
            ids: CvsData::read_ids(path),
            records: rds,
        }
    }
    fn read_ids(path: &str) -> Vec<String> {
        let file = File::open(path).unwrap();
        let mut rds = Reader::from_reader(file);
        let mut ids = vec![];
        for r in rds.deserialize() {
            let record: Track = r.unwrap();
            ids.push(record.track_id);
        }
        ids
    }
    fn find_pos_str_id(&self, id: &str) -> Option<usize> {
        let i = self.ids.iter().position(|v| v == id);
        i
    }
    fn find_data_by_pos(&mut self, pos: usize) -> Option<SongData> {
        let mut iter = self.records.records();
        let data = iter.nth(pos).unwrap();
        let p = Position::new();
        self.records.seek(p).unwrap();

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
                let div = sum / 2 as f32;
                mean.push(div);
            }
            return Some(mean);
        }
        return None;
    }
    None
}

// LOOP EACH FILE, THEN SERACH ID FOR AUTHOR, TITLE, ETC... !!!!
pub async fn main() {
    let paths = fs::read_dir("./songs/fma_small/").unwrap();
    let mut cvs_data = CvsData::new("songs/raw_tracks.csv");
    println!("{}", cvs_data.ids.len());

    for path in paths {
        let path = path.unwrap().path();
        if path.to_str().unwrap().contains(".DS_Store") {
            continue;
        }
        let albums = fs::read_dir(path).unwrap();
        for song in albums {
            let song = song.unwrap().path();
            let song = song.to_str().unwrap().to_string();
            let len: usize = song.len();
            let id = &song[len - 10..len - 4];
            let pos = cvs_data.find_pos_str_id(id);
            if let None = pos {
                continue;
            }
            let pos = pos.unwrap();
            println!("{} -> {}: {}", pos, id, song);
            let data = cvs_data.find_data_by_pos(pos);

            if let None = data {
                continue;
            }
            std::thread::spawn(async move || {
                let mut data = data.unwrap();
                let mono = read_mp3(song);
                if let Some(song_freqs) = mono {
                    let fingerprints = get_fingerprints(44100, song_freqs);
                    match fingerprints {
                        Some(fs) => {
                            for f in fs.iter() {
                                println!("Runnin");
                                data.set_fingerprint(f);
                                post_data(&data).await;
                            }
                        }
                        None => {
                            println!("No fingerprints can be made...");
                        }
                    }
                }
            });
        }
    }
}
async fn post_data(data: &SongData) {
    let json = format!(
        "{{
        \"name\": \"{}\",
        \"author\": \"{}\",
        \"duration\": \"{}\",
        \"id1\": {},
        \"id2\": {},
        \"id3\": {},
        \"id4\": {},
        \"id5\": {}
    }}",
        data.name,
        data.author,
        data.duration,
        data.fingerprint.id1,
        data.fingerprint.id2,
        data.fingerprint.id3,
        data.fingerprint.id4,
        data.fingerprint.id5,
    );

    let req = Request::builder()
        .method(Method::POST)
        .uri("https://sebs-playground.herokuapp.com/api/v1/fingerprints")
        .header("content-type", "application/json")
        .body(Body::from(json))
        .unwrap();

    let client = Client::new();

    let res = client.request(req).await;
    let res = res.unwrap();
    println!("{}", res.status());
}
