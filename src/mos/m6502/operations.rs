use super::{Pin, Pinout};
use super::core::*;
use super::instructions::Instruction;
use super::super::bus::Bus;

const fn to_address(hb: u8, lb: u8) -> u16 {
    (hb as u16) << 8 | (lb as u16) 
}

fn poll_interrupts(cpu: &mut Context, pinout: &mut Pinout) {
    // nmi is edge detected, only needs to be held one cycle to set flag
    if cpu.nmi_detected == true {
        cpu.ints = InterruptState::Nmi;
        cpu.ops.reset();
        cpu.ir.reset(0x00);
        cpu.nmi_detected = false;
    }
    // irq is level detected and must be held every cycle until handled
    else if pinout.irq == Pin::On && cpu.p.interrupt_disable != true {
        cpu.ints = InterruptState::Irq;
        cpu.ops.reset();
        cpu.ir.reset(0x00);
    }
}

//====================================================
// helper macros
//====================================================
macro_rules! first_cycle {
    ($cpu:ident, $bus:ident, $pinout:ident) => {
        $pinout.sync = Pin::On;
        $pinout.rw = Pin::On;
        $pinout.address = u16::from($cpu.pc);

        // fetch opcode
        $cpu.ops.dl = $bus.read($pinout.address);
        $cpu.ir.reset($cpu.ops.dl);
        $pinout.data = $cpu.ops.dl;

        $cpu.pc.increment();
        $cpu.ops.reset();
    }
}

macro_rules! second_cycle {
    ($cpu:ident, $bus:ident, $pinout:ident) => {
        $pinout.sync = Pin::Off;
        // instructions always read next byte after opcode
        $pinout.rw = Pin::On;
        $pinout.address = u16::from($cpu.pc);

        $cpu.ops.dl = $bus.read($pinout.address);
        $pinout.data = $cpu.ops.dl;
    }
}

macro_rules! last_cycle {
    ($cpu:ident, $pinout:ident) => {
        poll_interrupts($cpu, $pinout);
        if $cpu.ints != InterruptState::None {
            return;
        }
    }
}

macro_rules! read {
    ($cpu:ident, $bus:ident, $pinout:ident, $addr:expr) => {
        // set rw pin
        $pinout.rw = Pin::On;
        // get results from bus
        $pinout.address = $addr;
        $pinout.data = $bus.read($addr);
        // get data pins
        $cpu.ops.dl = $pinout.data;
    }
}

macro_rules! write {
    ($bus:ident, $pinout:ident, $addr:expr, $data: expr) => {
        // set rw pin
        $pinout.rw = Pin::Off;
        $pinout.address = $addr;
        $pinout.data = $data;
        // get results from bus
        $bus.write($addr, $data);
    }
}

//===================================================
// Reset
//====================================================
#[inline(always)]
pub fn rst_c0(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    cpu.reset();
    cpu.a = 0xAA;
    cpu.p = FlagsRegister::from(0x24);
    cpu.pc = ProgramCounter::from(0x00FF);
    read!(cpu, bus, pinout, 0x00FF);
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c1(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0x00FF);
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c2(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0x00FF);
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c3(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0x0100);
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c4(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0x01FF);
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c5(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0x01FE);
    cpu.sp = 0xFD;
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c6(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0xFFFC);
    cpu.ops.adl = cpu.ops.dl;
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c7(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    read!(cpu, bus, pinout, 0xFFFD);
    cpu.ops.adh = cpu.ops.dl;
    cpu.ir.increment();
}

#[inline(always)]
pub fn rst_c8(cpu: &mut Context, bus: &mut dyn Bus, pinout: &mut Pinout) {
    // first cycle of next instruction 
    cpu.pc.pcl = cpu.ops.adl;
    cpu.pc.pch = cpu.ops.adh;
    // kludge to match nestest.log cycle timing after reset
    cpu.cycle = 6;
    first_cycle!(cpu, bus, pinout);
}

