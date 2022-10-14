use super::{Peripheral, PeripheralProperties};
use btleplug::api::Characteristic;
use btleplug::api::Peripheral as _;

use super::Frame;
use uuid::Uuid;
const CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x0000ffe1_0000_1000_8000_00805f9b34fb);
#[derive(Debug, Clone)]
pub struct BlueToothInfo {
    pub name: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone)]
pub struct Device {
    p: Peripheral,
    props: PeripheralProperties,
    char: Characteristic,
    pub info: BlueToothInfo,
    pub euc_info: EucInfo,
}
impl Device {
    pub async fn new(p: Peripheral) -> Self {
        p.is_connected().await.unwrap();
        let (props, info) = Self::make_info(&p).await;
        let char = Self::make_char(&p).await;
        let euc_info = Self::make_euc_info(&p, &char).await.unwrap();
        Self {
            p, props, char, info, euc_info
        }
    }
    async fn make_info(p: &Peripheral) -> (PeripheralProperties, BlueToothInfo) {
        let props = p.properties().await.unwrap().unwrap();
        let is_connected = p.is_connected().await.unwrap();
        let info = BlueToothInfo {
            name: props.local_name.clone().unwrap(),
            is_connected,
        };
        (props, info)
    }
    async fn make_char(p: &Peripheral) -> Characteristic {
        p.discover_services().await.unwrap();
        let chars = p.characteristics();
        let char = chars.into_iter().find(|c| c.uuid == CHARACTERISTIC_UUID).unwrap();
        char
    }
    pub async fn update_info(&mut self) {
        let (props, info) = Self::make_info(&self.p).await;
        self.props = props;
        self.info = info;
        self.euc_info = Self::make_euc_info(&self.p, &self.char).await.unwrap();
    }
    pub fn is_connected(&self) -> bool {
        self.info.is_connected
    }
    async fn make_euc_info(p: &Peripheral, chr: &Characteristic) -> Result<EucInfo, ()> {
        use futures::StreamExt;
        let mut frame_ab = FrameAB::default();
        p.subscribe(&chr).await.map_err(|_| ())?;
        let mut stream = p.notifications().await.map_err(|_| ())?;
        let mut info = None;
        let mut buf = Vec::new();
        while info.is_none() {
//             let bytes = p.read(&chr).await.map_err(|_| ())?;
            let bytes = stream.next().await.ok_or(())?.value;
//             dbg!(&bytes);
            buf = [buf, bytes].concat();
            let mut bytes = buf.as_slice();
            let frame = Frame::try_from(&mut bytes);
            buf = Vec::from(bytes);
//             dbg!(&frame);
            if let Ok(frame) = frame {
                frame_ab.set_frame(frame);
                info = frame_ab.build();
            }
        }
//         dbg!(&info);
        Ok(info.unwrap())
    }

    async fn send_command(&self, cmd: &[u8]) {
        self.p.write(&self.char, cmd, btleplug::api::WriteType::WithoutResponse).await.unwrap();
    }
    pub async fn beep(&self) {
        self.send_command(b"b").await;
    }
}

#[derive(Debug, Clone, Default)]
pub struct EucInfo {

    pub voltage: f32,   // Bytes 2-3:   BE voltage, fixed point, 1/100th (assumes 67.2 battery, rescale for other voltages)
    pub speed: f32,     // Bytes 4-5:   BE speed, fixed point, 3.6 * value / 100 km/h
    pub distance: f32,  // Bytes 6-9:   BE distance, 32bit fixed point, meters
    pub current: f32,   // Bytes 10-11: BE current, signed fixed point, 1/100th amperes
    pub temperature: f32,// Bytes 12-13: BE temperature, (value / 340 + 36.53) / 100, Celsius degrees (MPU6050 native data)

    pub total_distance: f32,    // Bytes 2-5:   BE total distance, 32bit fixed point, meters
    pub settings: super::frame::Settings, // Byte  6:     pedals mode (high nibble), speed alarms (low nibble)
    pub alerts: super::frame::Alerts,
    pub led_mode: u8,           // Byte  13:    LED mode
    pub light_mode: u8,
}

impl EucInfo {
    fn set_frame(self, frame: Frame) -> Self {
        match frame {
        Frame::FrameA {
            voltage, speed, distance, current, temperature
        } => Self {voltage, speed, distance, current, temperature, ..self},
        Frame::FrameB {
            total_distance, settings, alerts, led_mode, light_mode
        } => Self {total_distance, settings, alerts, led_mode, light_mode, ..self},
        }
    }
    fn from_frame_ab(a: Frame, b: Frame) -> Self {
        match (a, b) {
        (Frame::FrameA {
            voltage, speed, distance, current, temperature
        }, Frame::FrameB {
            total_distance, settings, alerts, led_mode, light_mode
        }) | (Frame::FrameB {
            total_distance, settings, alerts, led_mode, light_mode
        }, Frame::FrameA {
            voltage, speed, distance, current, temperature
        }) => Self {voltage, speed, distance, current, temperature,
                    total_distance, settings, alerts, led_mode, light_mode},

        _ => unreachable!()
        }
    }
}

#[derive(Default)]
struct FrameAB {
    a: Option<Frame>,
    b: Option<Frame>,
}
impl FrameAB {
    fn set_frame(&mut self, frame: Frame) {
        match &frame {
        Frame::FrameA{..} => self.a = Some(frame),
        Frame::FrameB{..} => self.b = Some(frame),
        }
    }
    fn build(&mut self) -> Option<EucInfo> {
        match (self.a.take(), self.b.take()) {
        (Some(a), Some(b)) => Some(EucInfo::from_frame_ab(a,b)),
        (a, b) => {
            self.a = a;
            self.b = b;
            None
        },
        }
    }
}
#[test]
fn test_unpacket() {
    let mut bytes = [85, 170, 22, 194, 0, 0, 0, 0, 0, 0, 255, 66, 240, 237, 0, 1, 255, 248, 0, 24];
    let mut bytes = bytes.as_slice();
    let frame = Frame::try_from(&mut bytes);
    assert_eq!(frame, Err(()));

    let mut bytes = [Vec::from(bytes), vec![90, 90, 90, 90, 85, 170, 1, 22, 150, 186, 40, 0, 2, 208, 0, 57, 0, 0, 0, 7]].concat();
    let mut bytes = bytes.as_slice();
    dbg!(bytes.len(), bytes[18]);
    let frame = Frame::try_from(&mut bytes);
    assert_eq!(frame, Ok(Frame::FrameA {
            voltage: 58.26,
            speed: 0.0,
            distance: 0.0,
            current: -1.9,
            temperature: 0.25179997,
        })
    );

    let mut bytes = [Vec::from(bytes), vec![0, 8, 4, 24, 90, 90, 90, 90]].concat();
    let mut bytes = bytes.as_slice();
    dbg!(bytes.len(), bytes[18]);
    let frame = Frame::try_from(&mut bytes);
    assert_eq!(frame, Ok(Frame::FrameB {
            total_distance: 18257594.0,
            settings: super::frame::Settings {
                pedals_mode: None,
                speedAlarms: 2,
                rollAngle: 0,
                inMiles: false,
            },
            alerts: super::frame::Alerts::default(),
            led_mode: 0,
            light_mode: 7,
        })
    )

//     assert!(false);
}
