mod scan;
mod device;

pub use scan::*;
pub use device::Device;

use btleplug::platform::{Peripheral};
use btleplug::api::PeripheralProperties;
use device::BlueToothInfo;
