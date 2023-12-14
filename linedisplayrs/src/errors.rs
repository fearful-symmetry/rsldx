use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DisplayError {
    #[snafu(display("Could not find any matching USB displays"))]
    Descriptor{source: rusb::Error},
    #[snafu(display("No Matching USB display devices found"))]
    NoDevicesFound,
    #[snafu(display("Error finding USB device"))]
    DeviceFind{source: rusb::Error },
    #[snafu(display("Could not open device"))]
    HandleOpen{source: rusb::Error},
    #[snafu(display("Could not write to USB device"))]
    WriteBulk{source: rusb::Error},
    #[snafu(display("Could not set active configuration"))]
    SetActiveConfig{source: rusb::Error},
    #[snafu(display("Could not claim USB interface"))]
    ClaimInterface{source: rusb::Error},
    #[snafu(display("Could not set interface setting"))]
    SetAlternateSetting{source: rusb::Error},
    #[snafu(display("Could not list USB devices"))]
    DeviceList{source: rusb::Error},
    #[snafu(display("Could not read {field} from USB device"))]
    ReadString{source: rusb::Error, field: String}
}
