mod core;
mod instructions;
mod operations;

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

#[derive(Debug, Clone)]
pub struct Pinout {
    pub address: u16,
    pub data: u8,
    pub rw: Pin,        // memory read or write access (high read, low write)
    pub sync: Pin,      // start of a new instruction *not actual pin on Rp2a03, used for debugging emulator
    pub irq: Pin,       // maskable interrupt requested 
    pub nmi: Pin,       // non-maskable interrupt requested
    pub rdy: Pin,       // freeze execution at next read cycle
    pub res: Pin,       // reset CPU
    pub halt: Pin,      // (6502C only) freeze execution immedialty
    pub p0: Pin,        // (6510 only) input-output pin
    pub p1: Pin,
    pub p2: Pin,
    pub p3: Pin,
    pub p4: Pin,
    pub p5: Pin,
}

//external state of cpu
impl Pinout {
    pub fn new() -> Pinout {
        Pinout {
            address: 0,
            data: 0,
            rw: Pin::Off,
            sync: Pin::Off,
            irq: Pin::Off,
            nmi: Pin::Off,
            rdy: Pin::Off,
            res: Pin::Off,
            halt: Pin::Off,
            p0: Pin::Off,
            p1: Pin::Off,
            p2: Pin::Off,
            p3: Pin::Off,
            p4: Pin::Off,
            p5: Pin::Off,
        }
    }
}