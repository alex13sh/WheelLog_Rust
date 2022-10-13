mod scan;

pub use scan::get_list;

#[derive(Debug, Clone)]
pub struct BlueToothInfo {
    pub name: String,
    pub is_connected: bool,
}
