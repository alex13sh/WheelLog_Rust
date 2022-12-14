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
    EucCommand(EucCommand),
    Tick,
}

#[derive(Debug, Clone)]
enum EucCommand {
    Beep,
    LedTurn,
    LightTurn,
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
            Command::perform(Self::connect("GotWay_39336".into()), Message::Connected),
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
        }
        else {
//             time::every(Duration::from_millis(5000))
//                 .map(|_| Message::Reconnect)
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
//         dbg!(&message);
        match message {
            Message::Connect(_name) => {
                return Command::perform(Self::connect(self.device_name()), Message::Connected);
            }
            Message::Reconnect => return Command::perform(Self::connect(self.device_name()), Message::Connected),
            Message::Connected(d) => {
                println!("Connected: {}", d.is_some());
                self.device = d
            },
            Message::Disconnect => {
                if let Some(d) = self.device.take() {
                    return Command::perform(d.disconnect(), |_| Message::Tick);
                }
            }
            Message::UpdatedDevice(d) => self.device = Some(d),
            Message::Tick => if let Some(d) = self.device.clone() {
                return Command::perform(Self::update_device(d), Message::UpdatedDevice);
            }
            Message::EucCommand(cmd) => if let Some(ref d) = self.device {
                return Command::perform(Self::command(d.clone(), cmd), Message::UpdatedDevice);
            },
        };
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            text( if let Some(d) = self.get_connect_device() {
                    format!("????????????????????: {name} -- ????????????????????\nSpeed: {info}"
                        , name = d.info.name
                        , info = &d.euc_info
                    )
                } else {"???? ????????????????????".into()}),
            if self.is_connected() {
                row![
                    button(text("??????????????????????")).on_press(Message::Disconnect),
                    button(text("Beep")).on_press(Message::EucCommand(EucCommand::Beep)),
                    button(text("Led Turn")).on_press(Message::EucCommand(EucCommand::LedTurn)),
                    button(text("Light Turn")).on_press(Message::EucCommand(EucCommand::LightTurn)),
                ].spacing(10)
            } else {
                row![button(text("????????????????????????")).on_press(Message::Connect(self.device_name()))]
            },
        ].spacing(20)
        .padding(50)
        .into()
    }

}

impl EucInfo {
    fn device_name(&self) -> String {
        "GotWay_39336".to_owned()
    }
    async fn connect(name: String) -> Option<bluetooth::Device> {
        let p = {
            let res = bluetooth::connect(&name).await;
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
    fn get_connect_device(&self) -> Option<&bluetooth::Device> {
        if self.is_connected() {
            self.device.as_ref()
        } else {None}
    }
    async fn update_device(mut d: bluetooth::Device) -> bluetooth::Device {
        d.update_info().await;
        d
    }

    async fn command(device: bluetooth::Device, cmd: EucCommand) -> bluetooth::Device {
        let euc_info = device.euc_info.clone();
        match cmd {
        EucCommand::Beep => device.beep().await,
        EucCommand::LedTurn => device.set_led_mode(euc_info.led_mode+1).await,
        EucCommand::LightTurn => device.set_light_mode(euc_info.light_mode+1).await,
        }
    }
}
