//! UDP datagram device.
//!
//! Attaches to the ROM I/O port (`WRR`) and sends a UDP datagram each time
//! the 4004 finishes writing a complete message.
//!
//! # Wire protocol (write4 / WRR)
//!
//! ```text
//! nibble 0       length high nibble  ┐
//! nibble 1       length low nibble   ┘ byte count (0–255)
//! nibble 2,3     first byte (hi, lo)
//! nibble 4,5     second byte …
//! …
//! ```
//!
//! The device fires `send()` after the last byte arrives and resets for the
//! next message automatically.
//!
//! # Example
//!
//! ```no_run
//! use intel_4004::dev::udp::UdpDevice;
//! use intel_4004::chips::Rom4001;
//!
//! let mut rom = Rom4001::from_bytes(&[/* your ROM bytes */]);
//! rom.attach_port(UdpDevice::new("0.0.0.0:0", "127.0.0.1:1234").unwrap());
//! ```

use crate::dev::IoDevice;
use std::net::UdpSocket;
use std::time::Duration;

enum State {
    WaitLenHi,
    WaitLenLo { hi: u8 },
    Data { bytes_left: usize, hi: Option<u8>, buf: Vec<u8> },
}

pub struct UdpDevice {
    socket:        UdpSocket,
    state:         State,
    /// Optional pause after each send. Useful to throttle a looping ROM.
    send_interval: Option<Duration>,
}

impl UdpDevice {
    pub fn new(local_addr: &str, remote_addr: &str) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(local_addr)?;
        socket.connect(remote_addr)?;
        Ok(Self {
            socket,
            state:         State::WaitLenHi,
            send_interval: None,
        })
    }

    pub fn with_interval(mut self, d: Duration) -> Self {
        self.send_interval = Some(d);
        self
    }
}

impl IoDevice for UdpDevice {
    fn read4(&mut self) -> u8 {
        0
    }

    fn write4(&mut self, nibble: u8) {
        match &mut self.state {
            State::WaitLenHi => {
                self.state = State::WaitLenLo { hi: nibble };
            }
            State::WaitLenLo { hi } => {
                let len = ((*hi as usize) << 4) | (nibble as usize);
                self.state = State::Data {
                    bytes_left: len,
                    hi:         None,
                    buf:        Vec::with_capacity(len),
                };
            }
            State::Data { bytes_left, hi, buf } => {
                match hi.take() {
                    None    => *hi = Some(nibble),
                    Some(h) => {
                        buf.push((h << 4) | nibble);
                        *bytes_left -= 1;
                        if *bytes_left == 0 {
                            let _ = self.socket.send(buf);
                            if let Some(d) = self.send_interval {
                                std::thread::sleep(d);
                            }
                            self.state = State::WaitLenHi;
                        }
                    }
                }
            }
        }
    }
}
