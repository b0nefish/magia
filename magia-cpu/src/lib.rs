
/// This is the state the CPU is currently in regarding execute and instruction decode
enum DecodeExecState {
    Decode,
    Exec,
}

/// State for fetching of data.
enum FetchState {
    WaitFetch,
    ReadWord,
}

/// Data for the m68k CPU
struct Cpu {
    /// dn registers for the CPU
    d_registers: [u32; 8],
    /// an registers for the CPU
    a_registers: [u32; 8],
    /// First word of the instruction used for looking up handler
    irc: u16,
    /// First word of the instruction used for looking up handler
    ir: u16,
    /// First word of the instruction used for looking up handler
    ird: u16,
    /// Program counter
    pc: u32,
    exec_state: DecodeExecState,
    /// Number of cycles left for instruction decode
    decode_cycles: u8,
    /// How many cycles that are left in the exec state
    exec_cycles_left: u8,
}

impl Cpu {
    /// Step the CPU one cycle forward
    fn step(cpu: &mut Cpu, memory_bus: &mut MemoryBus) {


    }
}



