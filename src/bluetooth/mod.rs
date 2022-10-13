mod scan;

pub use scan::*;

pub use btleplug::platform::{Peripheral};
pub use btleplug::api::PeripheralProperties;

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
}
impl Device {
    pub async fn new(p: Peripheral) -> Self {
        use btleplug::api::Peripheral;

        let props = p.properties().await.unwrap().unwrap();
        let is_connected = p.is_connected().await.unwrap();
        let info = BlueToothInfo {
            name: props.local_name.clone().unwrap(),
            is_connected,
        };
        Self {
            p, props, info
        }
    }
    pub async fn update_info(&mut self) {
        use btleplug::api::Peripheral;

        self.props = self.p.properties().await.unwrap().unwrap();
        let is_connected = self.p.is_connected().await.unwrap();
        let info = BlueToothInfo {
            name: self.props.local_name.clone().unwrap(),
            is_connected,
        };
        self.info = info;
    }
    pub fn is_connected(&self) -> bool {
        self.info.is_connected
    }
}
