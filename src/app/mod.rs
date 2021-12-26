use crate::fingerprint::id::get_fingerprints;
use crate::fingerprint::recorder::Recorder;
use hyper::{Body, Client, Method, Request};
use iced::{button, Button, Column, Element, Row, Sandbox, Settings, Text};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    RustyShazam::run(Settings::default())
}

#[derive(Default)]
struct RustyShazam {
    text: String,
    matches: Vec<String>,
    increment_button: button::State,
    decrement_button: button::State,
    recorder: Recorder,
}

impl Sandbox for RustyShazam {
    type Message = Message;

    fn new() -> Self {
        let mut app = Self::default();
        app.text = "Not recording".to_string();
        app
    }

    fn title(&self) -> String {
        String::from("Rusty Shazam")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Play => {
                self.text = "Recording".to_string();
                self.recorder.start_recording();
            }
            Message::Stop => {
                self.recorder.stop_recording();
                self.text = "Not recording".to_string();
                self.matches = vec!["Looking for songs".to_string()];
                let fingerprints = get_fingerprints(2205, self.recorder.flush());
                match fingerprints {
                    Some(fs) => {
                        let mut results = vec![];
                        for f in fs.iter() {
                            let json = format!(
                                "{{\"id1\": {}, \"id2\": {},\"id3\": {}, \"id4\": {}, \"id5\": {}}}",
                                f.id1, f.id2, f.id3, f.id4, f.id5,
                            );

                            let req = Request::builder()
                                .method(Method::GET)
                                .uri("http://sebs-playground.herokuapp.com/api/v1/fingerprints")
                                .header("content-type", "application/json")
                                .body(Body::from(json))
                                .unwrap();

                            let client = Client::new();
                            let res = futures::executor::block_on(client.request(req));
                            if let Ok(res) = res {
                                println!("Processing");
                                let res = futures::executor::block_on(hyper::body::to_bytes(
                                    res.into_body(),
                                ))
                                .unwrap();
                                let d = std::str::from_utf8(&res.to_vec()[..]).unwrap().to_string();
                                let start = d.find("[").unwrap();
                                let end = d.find("]").unwrap();
                                let ex = &d[start..end + 1];
                                let deserialized: Vec<Data> = serde_json::from_str(ex).unwrap();
                                for data in deserialized.iter() {
                                    results.push(data.clone());
                                }
                                println!("Processing 2");
                            } else if let Err(e) = res {
                                println!("{}", e);
                            }
                        }
                        println!("Finished process");
                        let results: Vec<Data> =
                            results.iter().unique().map(|v| v.clone()).collect();
                        self.matches = results
                            .iter()
                            .map(|r| {
                                format!("{} by {}. Duration: {}", r.name, r.author, r.duration)
                            })
                            .collect();
                    }
                    None => self.matches = vec!["No results found".to_string()],
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let songs = self.matches.iter().map(|v| Text::new(v).size(12));
        let mut list = Column::new();
        for song in songs {
            list = list.push(song);
        }

        Column::new()
            .push(Text::new(&self.text).size(24))
            .padding(20)
            .push(
                Row::new()
                    .push(
                        Button::new(&mut self.increment_button, Text::new("Start recording"))
                            .on_press(Message::Play),
                    )
                    .push(
                        Button::new(&mut self.decrement_button, Text::new("Stop recording"))
                            .on_press(Message::Stop),
                    ),
            )
            .padding(20)
            .push(Text::new("Songs found").size(16))
            .push(list)
            .into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Play,
    Stop,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    name: String,
    author: String,
    duration: String,
}
