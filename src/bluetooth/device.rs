use super::{Peripheral, PeripheralProperties};

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
    pub info: BlueToothInfo,
//     pub euc_info: EucInfo,
}
impl Device {
    pub async fn new(p: Peripheral) -> Self {
        let (props, info) = Self::make_info(&p).await;
        Self {
            p, props, info
        }
    }
    async fn make_info(p: &Peripheral) -> (PeripheralProperties, BlueToothInfo) {
        use btleplug::api::Peripheral;
        let props = p.properties().await.unwrap().unwrap();
        let is_connected = p.is_connected().await.unwrap();
        let info = BlueToothInfo {
            name: props.local_name.clone().unwrap(),
            is_connected,
        };
        (props, info)
    }
    pub async fn update_info(&mut self) {
        let (props, info) = Self::make_info(&self.p).await;
        self.props = props;
        self.info = info;
    }
    pub fn is_connected(&self) -> bool {
        self.info.is_connected
    }
//     async fn make_euc_info(p: &Peripheral) -> EucInfo {
//         p.read().await;
//     }
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
    fn set_frame(self, frame: super::Frame) -> Self {
        match frame {
        super::Frame::FrameA {
            voltage, speed, distance, current, temperature
        } => Self {voltage, speed, distance, current, temperature, ..self},
        super::Frame::FrameB {
            total_distance, settings, alerts, led_mode, light_mode
        } => Self {total_distance, settings, alerts, led_mode, light_mode, ..self},
        }
    }
}

#[test]
fn test_unpacket() {
    let mut bytes = [85, 170, 22, 194, 0, 0, 0, 0, 0, 0, 255, 66, 240, 237, 0, 1, 255, 248, 0, 24];
    let mut bytes = bytes.as_slice();
    let frame = super::Frame::try_from(&mut bytes);
    dbg!(frame);

    let mut bytes = [Vec::from(bytes), vec![90, 90, 90, 90, 85, 170, 1, 22, 150, 186, 40, 0, 2, 208, 0, 57, 0, 0, 0, 7]].concat();
    let mut bytes = bytes.as_slice();
    dbg!(bytes.len(), bytes[18]);
    let frame = super::Frame::try_from(&mut bytes);
//     dbg!(bytes.len(), bytes[18]);
    dbg!(frame);

    let mut bytes = [Vec::from(bytes), vec![0, 8, 4, 24, 90, 90, 90, 90]].concat();
    let mut bytes = bytes.as_slice();
    dbg!(bytes.len(), bytes[18]);
    let frame = super::Frame::try_from(&mut bytes);
    dbg!(frame);

    assert!(false);
}

fn pop_front<'a, 'b>(slice: &'a mut &'b [i32]) {
    let pos = slice.array_chunks().position(|c| c == &[2, 3]).unwrap() * 2;

    *slice = &slice[pos..];
}

#[test]
fn test_slice_pop() {
    let mut slice =  &[0, 1, 2, 3][..];
    pop_front(&mut slice);
    println!("{:?}", slice);
    assert!(false);
}
