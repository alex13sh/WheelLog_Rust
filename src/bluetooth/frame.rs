
#[derive(Debug, PartialEq)]
pub enum Frame {
    // Bytes 0-1:   frame header, 55 AA
    // Byte  18:    frame type, 00 for frame A
    // Byte  19:    18 frame footer 00
    FrameA {
        voltage: f32,   // Bytes 2-3:   BE voltage, fixed point, 1/100th (assumes 67.2 battery, rescale for other voltages)
        speed: f32,     // Bytes 4-5:   BE speed, fixed point, 3.6 * value / 100 km/h
        distance: f32,  // Bytes 6-9:   BE distance, 32bit fixed point, meters
        current: f32,   // Bytes 10-11: BE current, signed fixed point, 1/100th amperes
        temperature: f32,// Bytes 12-13: BE temperature, (value / 340 + 36.53) / 100, Celsius degrees (MPU6050 native data)
    },
    // Byte  18:    frame type, 04 for frame B
    FrameB {
        total_distance: f32,    // Bytes 2-5:   BE total distance, 32bit fixed point, meters
        settings: Settings,     // Byte  6-7:     pedals mode (high nibble), speed alarms (low nibble)
        alerts: Alerts,
        led_mode: u8,           // Byte  13:    LED mode
        light_mode: u8,
    }
    // Bytes 20-23: frame footer, 5A 5A 5A 5A
}

impl TryFrom<&[u8; 24]> for Frame {
    type Error = ();
    fn try_from(bytes: &[u8; 24]) -> Result<Frame, ()> {
        if &bytes[0..=1] != &[0x55, 0xAA]
        || &bytes[20..=23] != &[0x5A; 4] {
            return Err(());
        }
        let frame = match bytes[18] {
        0x00 => Frame::FrameA {
            voltage: u16::from_be_bytes(to_arr(&bytes[2..4])) as f32 / 100.0,
            speed: i16::from_be_bytes(to_arr(&bytes[4..6])) as f32 * 3.6 / 100.0,
            distance: u32::from_be_bytes(to_arr(&bytes[6..10])) as f32,
            current: i16::from_be_bytes(to_arr(&bytes[10..12])) as f32 / 100.0,
//             temperature: (i16::from_be_bytes(to_arr(&bytes[12..14])) as f32 / 340.0 + 36.53) * 100.0,
//             temperature: (i16::from_be_bytes(to_arr(&bytes[12..14])) as f32 / 333.87 + 21.00) * 100.0,
            temperature: (i16::from_be_bytes(to_arr(&bytes[12..14])) as f32 / 340.0 + 36.53),
        },
        0x04 => Frame::FrameB {
            total_distance: u32::from_be_bytes(bytes[2..6].try_into().unwrap()) as f32,
            settings: Settings::from(u16::from_be_bytes(bytes[6..8].try_into().unwrap())),
            alerts: Alerts::from(bytes[12]),
            led_mode: bytes[13],
            light_mode: bytes[14],
        },
        _ => return Err(()),
        };
        Ok(frame)
    }
}

impl <'a, 'b> TryFrom<&'a mut &'b [u8]> for Frame {
    type Error = ();
    fn try_from(mut bytes: &'a mut &'b [u8]) -> Result<Frame, ()> {
        let pos = bytes.array_chunks().position(|c| c == &[0x55, 0xAA]).ok_or(())? * 2;
        *bytes = &bytes[pos..];
        if bytes.len()>=24 {
            if let Ok(frame) = Frame::try_from(&bytes.as_chunks::<24>().0[0]) {
                *bytes = &bytes[24..];
                Ok(frame)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

fn to_arr<const N: usize>(arr: &[u8]) -> [u8; N] {
    arr.try_into().unwrap()
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Settings {
    pub pedals_mode: Option<PedalMode>,
    pub speedAlarms: u8,
    pub rollAngle: u8,
    pub inMiles: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PedalMode {
    HighNibble,
    LowNibble,
}
impl From<u16> for Settings {
    fn from(settings: u16) -> Settings {
        let pedalsMode = ((settings >> 13) & 0x03) as u8;
        let speedAlarms = ((settings >> 10) & 0x03) as u8;
        let rollAngle = ((settings >> 7) & 0x03) as u8;
        let inMiles = settings == 1;

        Settings {
            pedals_mode: None,
            speedAlarms, rollAngle, inMiles,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Alerts(Vec<Alert>);

#[derive(Debug, Clone, PartialEq)]
pub enum Alert {
    HighPower,
    Speed2,
    Speed1,
    LowVoltage,
    OverVoltage,
    OverTemperature,
    errHallSensors,
    TransportMode
}

impl From<u8> for Alerts {
    fn from(alert: u8) -> Self {
        let mut allerts = Vec::new();

        if (alert & 0x01) == 1 {
            allerts.push(Alert::HighPower);
        }
        if ((alert>>1) & 0x01) == 1 {
            allerts.push(Alert::Speed2);
        }
        if ((alert>>2) & 0x01) == 1 {
            allerts.push(Alert::Speed1);
        }
        if ((alert>>3) & 0x01) == 1 {
            allerts.push(Alert::LowVoltage);
        }
        if ((alert>>4) & 0x01) == 1 {
            allerts.push(Alert::OverVoltage);
        }
        if ((alert>>5) & 0x01) == 1 {
            allerts.push(Alert::OverTemperature);
        }
        if ((alert>>6) & 0x01) == 1 {
            allerts.push(Alert::errHallSensors);
        }
        if ((alert>>7) & 0x01) == 1 {
            allerts.push(Alert::TransportMode);
        }
        Self(allerts)
    }
}
