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

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }

    pub fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.cpu.step(&mut self.bus);
        }
    }

    pub fn run_until(&mut self, mut stop: impl FnMut(&Cpu4004) -> bool) {
        while !stop(&self.cpu) {
            self.cpu.step(&mut self.bus);
        }
    }
}
