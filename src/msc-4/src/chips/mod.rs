pub mod i4001;
pub mod i4002;
pub mod i4004;

pub use i4001::Rom4001;
pub use i4002::DataRam4002;
pub use i4004::Cpu4004;

use crate::dev::IoDevice;

pub struct Port {
    dev: Option<Box<dyn IoDevice>>,
}

impl Port {
    pub fn new() -> Self {
        Self { dev: None }
    }

    pub fn attach(&mut self, dev: Box<dyn IoDevice>) {
        assert!(self.dev.is_none(), "Port already has a device attached");
        self.dev = Some(dev);
    }

    #[inline]
    pub fn write4(&mut self, value: u8) {
        if let Some(d) = &mut self.dev {
            d.write4(value & 0x0F);
        }
    }

    #[inline]
    pub fn read4(&mut self) -> u8 {
        self.dev.as_mut().map(|d| d.read4() & 0x0F).unwrap_or(0)
    }
}

impl Default for Port {
    fn default() -> Self {
        Self::new()
    }
}
