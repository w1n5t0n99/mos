
#[inline(always)]
#[must_use]
pub fn set_address_pins(mut pinout: u64, addr: u16) -> u64 {
    let addr = addr as u64;
    // clear old address
    pinout &= 0xFFFFFFFFFFFF0000;
    // set new address
    pinout | addr
}

#[inline(always)]
#[must_use]
pub fn get_address_pins(pinout: u64) -> u16 {
    pinout as u16
}

#[inline(always)]
#[must_use]
pub fn set_data_pins(mut pinout: u64, data: u8) -> u64 {
    let data = (data as u64) << 16;
    // clear old data
    pinout &= 0xFFFFFFFFFF00FFFF;
    // set new data
    pinout | data
}

#[inline(always)]
#[must_use]
pub fn get_data_pins(pinout: u64) -> u8 {
    (pinout >> 16) as u8
}

#[inline(always)]
#[must_use]
pub fn set_pin_on(pinout: u64, pin: u64) -> u64 {
    pinout | pin
}

#[inline(always)]
#[must_use]
pub fn set_pin_off(pinout: u64, pin: u64) -> u64 {
    pinout & (!pin)
}

#[inline(always)]
#[must_use]
pub fn is_pin_on(pinout: u64, pin: u64) -> bool {
    if (pinout & pin)  > 0 {
        return true;
    }
    
    false
}

#[inline(always)]
#[must_use]
pub fn is_pin_off(pinout: u64, pin: u64) -> bool {
    if (pinout & pin) == 0 {
        return true;
    }
    
    false
}
