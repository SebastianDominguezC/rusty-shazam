use crate::fingerprint::id::convert_to_fingerprint;
use crate::fingerprint::recorder::Recorder;
use crate::fingerprint::transformation::build_spectrum;
use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    recorder: Recorder,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.recorder.start_recording();
            }
            Message::DecrementPressed => {
                self.value -= 1;
                self.recorder.stop_recording();
                let spectrum = build_spectrum(2205, self.recorder.flush());
                match spectrum {
                    Some(s) => match convert_to_fingerprint(s) {
                        Some(fingerprints) => {
                            println!("{:#?}", fingerprints);
                        }
                        None => {
                            println!("No fingerprint can be made from this audio")
                        }
                    },
                    None => {
                        println!("No fingerprint can be made from this audio")
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
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Stop recording"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
