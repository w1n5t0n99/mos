use super::core::*;
use super::instructions::Instruction;

const fn to_address(hb: u8, lb: u8) -> u16 {
    (hb as u16) << 8 | (lb as u16) 
}

#[inline(always)]
#[must_use]
pub fn sa(mut pinout: u64, addr: u16) -> u64 {
    let addr = addr as u64;
    // clear old address
    pinout &= 0xFFFFFFFFFFFF0000;
    // set new address
    pinout | addr
}

#[inline(always)]
#[must_use]
pub fn ga(pinout: u64) -> u16 {
    pinout as u16
}

#[inline(always)]
#[must_use]
pub fn sd(mut pinout: u64, data: u8) -> u64 {
    let data = (data as u64) << 16;
    // clear old data
    pinout &= 0xFFFFFFFFFF00FFFF;
    // set new data
    pinout | data
}

#[inline(always)]
#[must_use]
pub fn gd(pinout: u64) -> u8 {
    (pinout >> 16) as u8
}

#[inline(always)]
#[must_use]
pub fn on(pinout: u64, pin: u64) -> u64 {
    pinout | pin
}

#[inline(always)]
#[must_use]
pub fn off(pinout: u64, pin: u64) -> u64 {
    pinout & (!pin)
}

#[inline(always)]
#[must_use]
pub fn is_on(pinout: u64, pin: u64) -> bool {
    if (pinout & pin)  > 0 {
        return true;
    }
    
    false
}

#[inline(always)]
#[must_use]
pub fn is_off(pinout: u64, pin: u64) -> bool {
    if (pinout & pin) == 0 {
        return true;
    }
    
    false
}

#[inline(always)]
#[must_use]
pub fn rd(pinout: u64) -> u64 {
    on(pinout, RW_PIN)
}

#[inline(always)]
#[must_use]
pub fn wr(pinout: u64) -> u64 {
    off(pinout, RW_PIN)
}

