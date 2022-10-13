use iced::widget::{button, Column, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Scan::run(Settings::default())
}

struct Scan {
    list: Vec<BlueToothInfo>,
}

struct BlueToothInfo {
    name: String,
    is_connected: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    UpdateList,
}

impl Sandbox for Scan {
    type Message = Message;

    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn title(&self) -> String {
        String::from("Scan - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateList => {

            }
        }
    }

    fn view(&self) -> Element<Message> {
        self.list.iter().fold(Column::new(),
            |c, info| c.push(text(format!("Name: {}; is_connected: {}", info.name, info.is_connected)))
        )
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
