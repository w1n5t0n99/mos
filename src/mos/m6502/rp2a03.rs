use super::core::*;
use super::operations;
use super::instructions;
use super::{Pin, Pinout};
use super::bus::Bus;



pub struct Rp2a03 {
    cpu: Context,
}

impl Rp2a03 {
    pub fn from_power_on() -> (Rp2a03, Pinout) {
        (Rp2a03 {cpu: Context::new(),}, Pinout::new())
    }

    pub fn tick(&mut self, bus: &mut dyn Bus, mut pinout: Pinout) -> Pinout {
        match self.cpu.ir.opcode {
            
        }

        pinout
    }

}