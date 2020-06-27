mod core;
mod instructions;
mod operations;
pub mod bus;
pub mod rp2a03;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pin {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pinout {
    pub address: u16,
    pub data: u8,
    pub opt0: u8,
    pub opt1: u8,
    pub io: u8,         // (6510 only) input-output pin
    pub rw: Pin,        // memory read or write access (high read, low write)
    pub sync: Pin,      // start of a new instruction *not actual pin on Rp2a03, used for debugging emulator
    pub irq: Pin,       // maskable interrupt requested, active low 
    pub nmi: Pin,       // non-maskable interrupt requested, active low
    pub rdy: Pin,       // freeze execution at next read cycle, gnd when cpu is not ready
    pub halt: Pin,      // (6502C only) freeze execution immedialty
}

//external state of cpu
impl Pinout {
    pub fn new() -> Pinout {
        Pinout {
            address: 0,
            data: 0,
            opt0: 0,
            opt1: 0,
            io: 0,
            rw: Pin::On,
            sync: Pin::Off,
            irq: Pin::On,
            nmi: Pin::On,
            rdy: Pin::On,
            halt: Pin::On,
        }
    }
}