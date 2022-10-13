use super::{Peripheral, PeripheralProperties};

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