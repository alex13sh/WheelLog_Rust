// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
// use std::time::Duration;
// use tokio::time;

pub async fn get_list_info() -> Result<Vec<super::BlueToothInfo>, Box<dyn Error>> {
    let mut lst = Vec::new();
    let f_list = get_list().await?;
    for p in f_list.into_iter() {
        lst.push( super::BlueToothInfo {
                name: p.properties().await?.unwrap()
                    .local_name.unwrap_or(String::from("(peripheral name unknown)")),
                is_connected: p.is_connected().await?,
        });
    }
    Ok(lst)
}

pub async fn get_list() -> Result<Vec<super::Peripheral>, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

//     let mut list = Vec::new();
    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
//         time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        return Ok(peripherals);
    }
    Ok(Vec::new())
}
