#[macro_use]
extern crate bitflags;

pub mod mos;


#[cfg(test)]
mod tests {
    use super::mos::{rp2a03, Ctrl, Pinout};
    
    #[test]
    fn it_works() {
        let (mut cpu, mut cpu_pinout) = rp2a03::Rp2a03::from_power_on();
        
        assert_eq!(cpu_pinout.ctrl.contains(Ctrl::RDY), true);
    }
}
