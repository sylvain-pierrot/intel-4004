struct Cpu {
    pub acc: u8, // 4-bit accumulator
    pub cy: bool, // 1-bit carry flag
    pub r: [u8; 16], // 4-bit registers (R0–R15)
    pub pc: u16, // 12-bit program counter
    pub stack: [u16; 3], // 12-bit stack
    pub sp: u8, // 2-bit stack pointer

    //!TODO: Memmory
    // pub rom: [u8; 4096], // 8-bit words, 4096 words
    // pub ram: [u8; 4096], // 8-bit words, 4096 words

    //!TODO: IO
    // pub io: u8, // 4-bit I/O
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            acc: 0,
            cy: false,
            r: [0; 16],
            pc: 0,
            stack: [0; 3],
            sp: 0,
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.cy = false;
        self.r = [0; 16];
        self.pc = 0;
        self.stack = [0; 3];
        self.sp = 0;
    }

    pub fn step(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
    }

    fn fetch(&mut self) -> u8 {
        let instruction = self.rom[self.pc];
        self.pc += 1;
        instruction
    }
    
    fn execute(&mut self, instruction: u8) {
        //!TODO: Execute instruction
    }
}