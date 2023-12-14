//! Display implements a user space driver for LDX-model line displays.
//! Complete documentation for LDX controllers can be found here: 
//! <https://logiccontrols.com/wp-content/uploads/2020/02/LC2020_LineDisplay_UserManual.pdf>
//!
//! Creating and sending text to a connected device is simple:
//! ```Rust
//! let mut disp = find_display()?;
//! disp.write_str_scroll("Hello Word!", ScrollDirection::Left, ScrollPosition::Top)?;
//! ```

pub use crate::display::find_display;
pub use crate::display::{ScrollDirection, ScrollPosition, Brightness, CursorMode};

use helpers::UsbEndpoint;
use rusb::{DeviceHandle, GlobalContext};
mod helpers;

mod display;
pub mod errors;

/// Control a single connected display device.
#[derive(Debug)]
pub struct Display {
    device: DeviceHandle<GlobalContext>,
    endpoint: UsbEndpoint
}

