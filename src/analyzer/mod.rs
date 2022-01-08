pub mod recorder;

use crate::app::Message;
use crate::fingerprint::id::get_fingerprints;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub struct Analyzer {}

impl Analyzer {
    pub async fn analyze(data: Vec<f32>) -> Message {
        // Get fingerprints
        let fingerprints = get_fingerprints(22050, data);

        match fingerprints {
            Some(fs) => {
                // Declare results
                let mut results = vec![];

                // Loop every fingerprint
                for f in fs.iter() {
                    println!("Finding fingerprint");
                    // Build ID and request
                    let ids = Id::new(f.id1, f.id2, f.id3, f.id4, f.id5);
                    let uri = "http://sebs-playground.herokuapp.com/api/v1/fingerprints";
                    let req = surf::get(uri).body_json(&ids);

                    // If okay request built...
                    if let Ok(r) = req {
                        // Make request
                        if let Ok(mut res) = r.await {
                            // Parse body and extract data
                            let d = res.body_string().await.unwrap();
                            let start = d.find("[").unwrap();
                            let end = d.find("]").unwrap();
                            let ex = &d[start..end + 1];

                            // Parse results into a vector of songs
                            let deserialized: Vec<Song> = serde_json::from_str(ex).unwrap();
                            for data in deserialized.iter() {
                                results.push(data.clone());
                            }
                        }
                    }
                }

                // Get unique songs
                let results: Vec<Song> = results.iter().unique().map(|v| v.clone()).collect();

                // Map into a format
                let results: Vec<String> = results
                    .iter()
                    .map(|r| format!("{} by {}. Duration: {}", r.name, r.author, r.duration))
                    .collect();

                // Return message for app update
                return Message::Analyze(results);
            }
            None => {
                return Message::Analyze(vec!["No results found".to_string()]);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Song {
    name: String,
    author: String,
    duration: String,
}

#[derive(Deserialize, Serialize)]
struct Id {
    id1: i32,
    id2: i32,
    id3: i32,
    id4: i32,
    id5: i32,
}

impl Id {
    fn new(id1: i32, id2: i32, id3: i32, id4: i32, id5: i32) -> Self {
        Self {
            id1,
            id2,
            id3,
            id4,
            id5,
        }
    }
}
