use crate::fingerprint::id::get_fingerprints;
use crate::fingerprint::recorder::Recorder;
use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    RustyShazam::run(Settings::default())
}

#[derive(Default)]
struct RustyShazam {
    text: String,
    increment_button: button::State,
    decrement_button: button::State,
    recorder: Recorder,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for RustyShazam {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rusty Shazam")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.text = "Recording".to_string();
                self.recorder.start_recording();
            }
            Message::DecrementPressed => {
                self.recorder.stop_recording();
                self.text = "Not recording".to_string();
                let fingerprints = get_fingerprints(2205, self.recorder.flush());
                match fingerprints {
                    Some(fs) => {
                        println!("{:#?}", fs);
                    }
                    None => {
                        println!("No fingerprints can be made...");
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .push(
                Button::new(&mut self.increment_button, Text::new("Start recording"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(&self.text).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Stop recording"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
