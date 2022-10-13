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

// use lazy_static::lazy_static;
use std::collections::BTreeMap;

use BlueToothCommand::bluetooth;
use bluetooth::BlueToothInfo;

pub fn main() -> iced::Result {
    Scan::run(Settings::default())
}

struct Scan {
    list: BTreeMap<String, BlueToothInfo>,
}

#[derive(Debug, Clone)]
enum Message {
    UpdateList,
    UpdatedList(Vec<BlueToothInfo>),
}

impl Application for Scan {
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Scan, Command<Message>) {
        (
            Scan {
                list: Default::default(),
            },
            Command::perform(async{bluetooth::get_list().await.unwrap()}, Message::UpdatedList),
        )
    }

    fn title(&self) -> String {
        String::from("Scan - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UpdateList => {
//                 return Command::perform(scan::get_list(), Message::UpdatedList);
                return Command::perform(async{bluetooth::get_list().await.unwrap()}, Message::UpdatedList);
            }
            Message::UpdatedList(lst) => self.list = lst.into_iter().map(|i| (i.name.clone(), i)).collect(),
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        self.list.iter().fold(Column::new(),
            |c, (_name, info)| c.push(text(format!("Name: {};\n  is_connected: {}", info.name, info.is_connected)))
        )
        .push(button(text("Обновить")).on_press(Message::UpdateList))
        .padding(20)
        .spacing(10)
//         .align_items(Alignment::Center)
        .into()
    }
}
