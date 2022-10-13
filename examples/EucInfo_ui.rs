use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, Column, container, row, scrollable, text,
    text_input, Text,
};
use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Font, Length, Settings, Subscription};

use BlueToothCommand::bluetooth;
use bluetooth::BlueToothInfo;

pub fn main() -> iced::Result {
    EucInfo::run(Settings::default())
}

struct EucInfo {
    p: Option<bluetooth::Peripheral>,
}

#[derive(Debug, Clone)]
enum Message {
    Connect(String),
    Connected(Option<bluetooth::Peripheral>),
}

impl Application for EucInfo {
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (EucInfo, Command<Message>) {
        (
            EucInfo {
                p: Default::default(),
            },
            Command::perform(async{bluetooth::connect("GotWay_39336").await.ok()}, Message::Connected),
        )
    }

    fn title(&self) -> String {
        String::from("Scan - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        dbg!(&message);
        match message {
            Message::Connect(_name) => {
                return Command::perform(async{bluetooth::connect("GotWay_39336").await.ok()}, Message::Connected);
            }
            Message::Connected(p) => self.p = p,
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text( if self.p.is_some() {"Подключено"} else {"Не подключено"}),
            button(text("Подключиться")).on_press(Message::Connect("GotWay_39336".to_owned()))
        ].spacing(20)
        .into()
    }

}
