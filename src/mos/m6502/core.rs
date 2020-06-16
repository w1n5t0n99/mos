
/*
Mos 6502

## Emulated Pins
************************************
*           +-----------+          *
*   IRQ --->|           |---> A0   *
*   NMI --->|           |...       *
*    RDY--->|           |---> A15  *
*    RES--->|           |          *
*    RW <---|           |<--- HALT *
*  SYNC <---|           |          *
*           |           |<--> D0   *
*   (P0)<-->|           |...       *
*        ...|           |<--> D7   *
*   (P5)<-->|           |          *
*           +-----------+          *
************************************

The input/output P0..P5 pins only exist on the m6510.

The HALT pin is only used by the 6502C (Atari 5200/ Sally), unlike the RDY pin HALT halts
the cpu during Rd or Wr cycles.

If the RDY pin is active (1) the CPU will loop on the next read
access until the pin goes inactive.

*/

// pins 0-23 are for address and data lines
pub const RW_PIN: u64 = 1 << 24;                // memory read or write access (high read, low write)
pub const SYNC_PIN: u64 =  1 << 25;             // start of a new instruction *not actual pin on Rp2a03, used for debugging emulator
pub const IRQ_PIN: u64 = 1 << 26;               // maskable interrupt requested 
pub const NMI_PIN: u64 = 1 << 27;               // non-maskable interrupt requested
pub const RDY_PIN: u64 = 1 << 28;               // freeze execution at next read cycle
pub const RES_PIN: u64 = 1 << 29;               // reset CPU
pub const HALT_PIN: u64 = 1 << 30;              // (6502C only) freeze execution immedialty
pub const P0_PIN: u64 = 1 << 31;                // (6510 only) input-output pin
pub const P1_PIN: u64 = 1 << 32;
pub const P2_PIN: u64 = 1 << 33;
pub const P3_PIN: u64 = 1 << 34;
pub const P4_PIN: u64 = 1 << 35;
pub const P5_PIN: u64 = 1 << 36;


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FlagsRegister {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl FlagsRegister {
    pub fn push_with_b_set(&mut self) -> u8{
        let mut p = u8::from(*self);
        p = p | (1 << 4);
        p
    }

    pub fn push_with_b_clear(&mut self) -> u8{
        let p = u8::from(*self);
        // bit 5 is always 1 when pushed
        p
    }

    pub fn pull(p: u8) -> FlagsRegister {
        let p = p & 0b11101111;
        let fr = FlagsRegister::from(p);
        fr
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.carry              { 1 } else { 0 }) << 0 |
        (if flag.zero               { 1 } else { 0 }) << 1 |
        (if flag.interrupt_disable  { 1 } else { 0 }) << 2 |
        (if flag.decimal            { 1 } else { 0 }) << 3 |
        (if flag.overflow           { 1 } else { 0 }) << 6 | 
        (if flag.negative           { 1 } else { 0 }) << 7 |
        1 << 5
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let carry = ((byte >> 0) & 0b1) != 0;
        let zero = ((byte >> 1) & 0b1) != 0;
        let interrupt_disable = ((byte >> 2) & 0b1) != 0;
        let decimal = ((byte >> 3) & 0b1) != 0;
        let overflow = ((byte >> 6) & 0b1) != 0;
        let negative = ((byte >> 7) & 0b1) != 0;

        FlagsRegister {
            carry,
            zero,
            interrupt_disable,
            decimal,
            overflow,
            negative,
        }
    }    
}

/*
internally the 16bit program counter was 2 8bit registers, requiring 
two cycles to set both the high and low address bytes
*/
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ProgramCounter {
    pub pcl: u8,
    pub pch: u8,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            pch: 0,
            pcl: 0,
        }
    }

    #[inline]
    pub fn increment(&mut self) {
        let mut pc: u16 = u16::from(*self);
        pc += 1;
        *self = ProgramCounter::from(pc);
    }
}

impl std::convert::From<ProgramCounter> for u16 {
    fn from(pc: ProgramCounter) -> u16 {
        (pc.pch as u16) << 8 | (pc.pcl as u16) 
    }
}

impl std::convert::From<u16> for ProgramCounter {
    fn from(b: u16) -> ProgramCounter {
        ProgramCounter {
            pcl: b as u8,
            pch: (b >> 8) as u8,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct InstructionRegister {
    pub opcode: u8,
    pub tm: u8,
}

impl InstructionRegister {
    pub fn new() -> InstructionRegister {
        InstructionRegister {
            opcode: 0,
            tm: 0,
        }
    }

    #[inline]
    pub fn reset(&mut self, opcode: u8) {
        self.opcode = opcode;
        self.tm = 0;
    }

    #[inline]
    pub fn increment(&mut self) {
        self.tm = self.tm.wrapping_add(1);
    }
}

impl std::convert::From<InstructionRegister> for u16 {
    fn from(ir: InstructionRegister) -> u16 {
        (ir.opcode as u16) << 8 | (ir.tm as u16) 
    }
}

impl std::convert::From<u16> for InstructionRegister {
    fn from(b: u16) -> InstructionRegister {
        InstructionRegister {
            tm: b as u8,
            opcode: (b >> 8) as u8,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InterruptState {
    None,
    BrkHijack,
    IrqHijack,
    Irq,
    Nmi,
}

/*
Holds intermediate data during cycle operations, the 6502 would have used internal registers
or the ALU to hold this data
 */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct OpState {
    //base address - address in index addressing modes that specifies index location
    pub bal: u8,
    pub bah: u8,
    // effective address - destination in memory where data is found
    pub adl: u8,
    pub adh: u8,
    // indirect address - address found in operand of instruction using (Indirect),Y
    pub ial: u8,
    pub iah: u8,
    // offset address
    pub offset: u8,
    pub offset_carry: bool,
    pub offset_neg: bool,
    pub branch_taken: bool,
    // data input latch
    pub dl: u8,
}

impl OpState {
    pub fn new() -> OpState {
        OpState {
            bal: 0,
            bah: 0,
            adl: 0,
            adh: 0,
            ial: 0,
            iah: 0,
            offset: 0,
            offset_carry: false,
            offset_neg: false,
            branch_taken: false,
            dl: 0,
        }
    }

    pub fn reset(&mut self) {
        self.bal = 0;
        self.bah = 0;
        self.adl = 0;
        self.adh = 0;
        self.ial = 0;
        self.iah = 0;
        self.offset = 0;
        self.offset_carry = false;
        self.offset_neg = false;
        self.branch_taken = false;
        self.dl = 0;
    }
}

#[derive(Debug, Clone)]
pub struct Context
{
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub cycle: u64,
    pub ir: InstructionRegister,
    pub p: FlagsRegister,
    pub pc: ProgramCounter,
    pub ops: OpState,
    pub ints: InterruptState,
    pub nmi_detected: bool,
}

impl Context
{
    pub fn new() -> Context
    {
        Context
        {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            cycle: 0,
            ir: InstructionRegister::new(),
            p: FlagsRegister::from(0),
            pc: ProgramCounter::from(0),
            ops: OpState::new(),
            ints: InterruptState::None,
            nmi_detected: false,
        }
    }
}

