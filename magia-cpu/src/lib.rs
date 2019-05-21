/// Data for the m68k CPU
struct Cpu {
    /// dn registers for the CPU
    d_registers: [u32; 8],
    /// an registers for the CPU
    a_registers: [u32; 8],
    /// First word of the instruction used for looking up handler
    ir: u32,
    /// Program counter
    pc: u32,
}

impl Cpu {



}


