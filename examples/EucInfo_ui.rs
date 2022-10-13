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
use iced::{Color, Command, Font, Length, Settings, Subscription, time};

use BlueToothCommand::bluetooth;

pub fn main() -> iced::Result {
    EucInfo::run(Settings::default())
}

struct EucInfo {
    device: Option<bluetooth::Device>,

}

#[derive(Debug, Clone)]
enum Message {
    Connect(String),
    Reconnect, Disconnect,
    Connected(Option<bluetooth::Device>),
    UpdatedDevice(bluetooth::Device),
    Tick,
}

impl Application for EucInfo {
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (EucInfo, Command<Message>) {
        (
            EucInfo {
                device: Default::default(),
            },
            Command::perform(Self::connect(), Message::Connected),
        )
    }

    fn title(&self) -> String {
        String::from("Scan - Iced")
    }

    fn subscription(&self) -> Subscription<Message> {
        use std::time::Duration;
        if self.is_connected() {
            time::every(Duration::from_millis(1000))
                .map(|_| Message::Tick)
        } else {
            time::every(Duration::from_millis(5000))
                .map(|_| Message::Reconnect)
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        dbg!(&message);
        match message {
            Message::Connect(_name) => {
                return Command::perform(Self::connect(), Message::Connected);
            }
            Message::Reconnect => return Command::perform(Self::connect(), Message::Connected),
            Message::Connected(d) => self.device = d,
            Message::Disconnect => self.device = None,
            Message::UpdatedDevice(d) => self.device = Some(d),
            Message::Tick => if let Some(d) = self.device.clone() {
                return Command::perform(Self::update_device(d), Message::UpdatedDevice);
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text( if self.is_connected() {
                    format!("Устройство: {} -- Подключено", &self.device.as_ref().unwrap().info.name)
                } else {"Не подключено".into()}),
            if self.is_connected() {
                button(text("Отключиться")).on_press(Message::Disconnect)
            } else {
                button(text("Подключиться")).on_press(Message::Connect(self.device_name()))
            }
        ].spacing(20)
        .into()
    }

}

impl EucInfo {
    fn device_name(&self) -> String {
        "GotWay_39336".to_owned()
    }
    async fn connect() -> Option<bluetooth::Device> {
        let p = {
            let res = bluetooth::connect("GotWay_39336").await;
            if let Err(ref err) = &res {
                dbg!(err);
                return None;
            }
            res.unwrap()
        };
        let d = bluetooth::Device::new(p).await;
        Some(d)
    }

    fn is_connected(&self) -> bool {
        if let Some(d) = &self.device {
            d.is_connected()
        } else {false}
    }
    async fn update_device(mut d: bluetooth::Device) -> bluetooth::Device {
        d.update_info().await;
        d
    }
}
