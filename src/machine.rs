use crate::bus::Bus;
use crate::chips::Cpu4004;

pub struct Machine<B: Bus> {
    cpu: Cpu4004,
    bus: B,
}

impl<B: Bus> Machine<B> {
    pub fn new(bus: B) -> Self {
        Self {
            cpu: Cpu4004::default(),
            bus,
        }
    }

    pub fn cpu(&self) -> &Cpu4004 {
        &self.cpu
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.bus);
        }
    }

    pub fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.cpu.step(&mut self.bus);
        }
    }
}
