use crate::{
    cpu::Cpu,
    memory::{data::DataMemory, program::ProgramMemory},
};

pub struct Machine<P: ProgramMemory, D: DataMemory> {
    cpu: Cpu,
    program: P,
    data: D,
}

impl<P: ProgramMemory, D: DataMemory> Machine<P, D> {
    pub fn new(program: P, data: D) -> Self {
        Self {
            cpu: Cpu::default(),
            program,
            data,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&self.program, &mut self.data);
        }
    }
}
