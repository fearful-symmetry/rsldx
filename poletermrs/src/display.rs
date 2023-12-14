
use core::fmt;
use std::time::Duration;
use rusb::{TransferType, Direction};
use snafu::ResultExt;
use crate::{errors::*, Display, helpers::{find_first_device, find_endpoint, UsbEndpoint, self}};

/// Set the brightness level of the display.
#[derive(Debug, Clone)]
pub enum Brightness {
    TwentyPercent,
    FortyPercent,
    SixtyPercent,
    Full
}

impl From<Brightness> for u8 {
    fn from(val: Brightness) -> Self {
        match val {
            Brightness::TwentyPercent => 20,
            Brightness::FortyPercent => 40,
            Brightness::SixtyPercent=> 60,
            Brightness::Full => 100
        }
    }
}

/// Adjust the scroll direction for scroll and smart scroll modes.
#[derive(Debug, Clone)]
pub enum ScrollDirection {
    Left,
    Right
}

/// Adjust the scroll position for scroll and smart scroll modes.
#[derive(Debug, Clone)]
pub enum ScrollPosition {
    Top,
    Bottom
}

/// Enable or disable the blinking cursor.
#[derive(Debug, Clone)]
pub enum CursorMode {
    On,
    Off
}   

/// Find a connected display via USB. This searches for available displays and returns the first device that matches 
/// a supported vendor/device ID.
pub fn find_display() -> Result<Display, DisplayError> {
    let mut dev = find_first_device()?;
    let dev_desc = dev.device_descriptor().context(DescriptorSnafu)?;
    
    let endpoint = find_endpoint(&mut dev, &dev_desc, TransferType::Bulk, Direction::Out)?.ok_or(DisplayError::NoDevicesFound)?;
    let mut dev = Display{device: dev.open().context(HandleOpenSnafu)?, endpoint: endpoint.clone()};
    dev.configure_endpoint(&endpoint)?;
    Ok(dev)
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = helpers::get_device_string(&self.device.device()).unwrap_or_else(|e| format!("error opening device: {}", e));
        write!(f, "{}", info)
    }
}

impl Display {
    /// Write a byte buffer to the display.
    pub fn write_raw(&mut self, payload: &[u8]) -> Result<(), DisplayError>{
        self.device.write_bulk(self.endpoint.address, payload, Duration::from_secs(1)).context(WriteBulkSnafu)?;
        Ok(())
    }

    /// Write a string to the byte buffer.
    pub fn write_str(&mut self, payload: &str) -> Result<(), DisplayError>{
        self.write_raw(payload.as_bytes())
    }

    /// Enable or disable the cursor.
    pub fn cursor(&mut self, mode: CursorMode) -> Result<(), DisplayError> {
        let payload: u8 = match mode {
            CursorMode::Off =>0x14,
            CursorMode::On => 0x13
        };
        self.write_raw(&[payload])
    }

    /// Scroll a message in the given direction and position. Maximum 45 characters.
    pub fn write_str_scroll(&mut self, msg: &str, direction: ScrollDirection, position: ScrollPosition) -> Result<(), DisplayError>{
        // these prefix codes are wild
        let prefix: &[u8] = match direction{
            ScrollDirection::Left => {
                match position {
                    ScrollPosition::Top => { &[0x5]},
                    ScrollPosition::Bottom => {&[0x1b, 0x6]} ,
                }
            },
            ScrollDirection::Right => {
                match position {
                    ScrollPosition::Top => {&[0x1b, 0x7]},
                    ScrollPosition::Bottom => {&[0x1b, 0x0b]}
                }
            }
        };
        let end_char: &[u8] = &[0xd];
        let buf = [prefix, msg.as_bytes(), end_char].concat();
        self.write_raw(&buf)
    }

    /// Writes a "smart scroll" message. Each string in the `msg` vector scrolls on and then off the screen, in order.
    /// Each part must be less than 20 characters, and up to 45 characters total.
    pub fn write_str_smart_scroll(&mut self, msg: Vec<String>, directon: ScrollDirection, position: ScrollPosition) -> Result<(), DisplayError> {
        let prefix: &[u8] = match directon{
            ScrollDirection::Left => {
                match position {
                    ScrollPosition::Top => {&[0x1b, 0x15]},
                    ScrollPosition::Bottom => {&[0x1b, 0x16]},
                }
            },
            ScrollDirection::Right => {
                match position {
                    ScrollPosition::Top => {&[0x1b, 0x13]},
                    ScrollPosition::Bottom => {&[0x1b, 0x14]}
                }
            }
        };
        // Add delimiter
        let sep: &[u8] = &[0x1c];
        let payload: Vec<&[u8]> = msg.iter().map(|p| p.as_bytes()).collect();
        let payload_flat: Vec<u8> = payload.join(sep);
        let buf = [prefix, &[0x1c], payload_flat.as_ref(), &[0x0d]].concat();
        self.write_raw(&buf)
    }

    /// Resets the display.
    pub fn reset_display(&mut self) -> Result<(), DisplayError> {
        self.write_raw(&[0x1f])
    }

    /// Adjusts the brightness of the display.
    /// NOTE: support for this seems to be inconsistent, or perhaps the LCI controller docs are wrong. Not sure. 
    pub fn change_brightness(&mut self, brightness: Brightness) ->Result<(), DisplayError> {
        let buf: &[u8]= &[0x4, brightness.into()];
        self.write_raw(buf)
    }

    fn configure_endpoint(&mut self,  endpoint: &UsbEndpoint) -> Result<(), DisplayError> {
        self.device.set_active_configuration(endpoint.config).context(SetActiveConfigSnafu)?;
        self.device.claim_interface(endpoint.interface).context(ClaimInterfaceSnafu)?;
        self.device.set_alternate_setting(endpoint.interface, endpoint.setting).context(SetAlternateSettingSnafu)?;
        Ok(())
    }
}



#[cfg(test)]
mod test {

    use anyhow::Result;

    use super::{find_display, ScrollDirection, ScrollPosition};

    #[test]
    fn write_basic_string() -> Result<()> {
        let mut disp = find_display()?;
        let test = String::from("Hello word!");
        disp.write_raw(test.as_ref())?;

        Ok(())
    }

    #[test]
    fn reset_display() -> Result<()> {
        let mut disp = find_display()?;
        disp.reset_display()?;

        Ok(())
    }

    #[test]
    fn test_scroll() ->Result<()> {
        let mut disp = find_display()?;
        disp.write_str_scroll("Hello Word!", ScrollDirection::Left, ScrollPosition::Top)?;
        Ok(())
    }

    #[test]
    fn smart_scroll() ->Result<()> {
        let test_in = vec![String::from("TIGERS"), String::from("4 for $20"), String::from("THIS WEEKEND ONLY")];
        let mut disp = find_display()?;
        disp.write_str_smart_scroll(test_in, super::ScrollDirection::Right, super::ScrollPosition::Top)?;
        Ok(())
    }

    #[test]
    fn test_cursor() ->Result<()>{
        let mut disp = find_display()?;
        let test = String::from("Hello word!");
        disp.write_raw(test.as_ref())?;
        disp.cursor(super::CursorMode::Off)?;
        Ok(())
    }

}