mod scan;

pub use scan::{
    get_list, get_list_info
};

pub use btleplug::platform::Peripheral;

#[derive(Debug, Clone)]
pub struct BlueToothInfo {
    pub name: String,
    pub is_connected: bool,
}
