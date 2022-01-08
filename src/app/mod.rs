use crate::analyzer::recorder::Recorder;
use crate::analyzer::Analyzer;

use iced::{
    button, executor, Application, Button, Clipboard, Column, Command, Element, Row, Settings, Text,
};
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

impl Application for RustyShazam {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut app = Self::default();
        app.text = "Not recording".to_string();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rusty Shazam")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Play => {
                self.text = "Recording".to_string();
                self.recorder.start_recording();
            }
            Message::Stop => {
                self.recorder.stop_recording();
                self.text = "Not recording".to_string();
                self.matches = vec!["Looking for songs".to_string()];
                return Command::from(Analyzer::analyze(self.recorder.flush()));
            }
            Message::Analyze(data) => {
                self.matches = data;
            }
        }
        Command::none()
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

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Stop,
    Analyze(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Data {
    name: String,
    author: String,
    duration: String,
}
