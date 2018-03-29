pub struct Cpu {
    /// Index resister
    pub i: u16,
    /// Program counter
    pub pc: u16,
    pub memory: [u8; 4096],
    /// registers
    pub v: [u8; 16],
    // pub keypad: Keypad,
    // pub display: Display,
    pub stack: [u16; 16],
    /// stack pointer
    pub sp: u8,
    /// delay timer
    pub dt: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0x200,
            pc: 0x200,
            memory: [0; 4096],
            v: [0; 16],
            // keypad: None,
            // display: None,
            stack: [0; 16],
            sp: 0,
            dt: 0,
        }
    }
}

fn main() {
    println!("Hello, cpu!");
}
