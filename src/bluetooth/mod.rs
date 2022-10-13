mod scan;

pub use scan::*;

pub use btleplug::platform::Peripheral;

#[derive(Debug, Clone)]
pub struct BlueToothInfo {
    pub name: String,
    pub is_connected: bool,
}
