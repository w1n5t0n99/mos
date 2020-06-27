use super::core::*;

#[inline]
fn set_zero(x: u8) -> bool {
    if x == 0 { true } else { false }
}

#[inline]
fn set_negative(x: u8) -> bool {
    if (x & 0x80) == 0x80 { true } else { false }
}

pub trait Instruction {
    fn execute(cpu: &mut Context);
}

//=====================================================
// official opcodes
//====================================================== 

pub struct AdcNoDec {}
impl Instruction for AdcNoDec {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let sum = (cpu.a as u16) + (cpu.ops.dl as u16) + (cpu.p.carry as u16); 
        cpu.p.carry = if sum > 255 { true } else {false };

        let result = sum as u8;
        cpu.a = result;
       // cpu.p.overflow =  if (signed_sum < -128) || (signed_sum > 127) { true } else { false };
        cpu.p.overflow =  if ((cpu.ops.dl ^ result) & (cpu.a & result) & 0x80) == 0x80 { true } else { false };
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);

    }
}

pub struct Adc {}
impl Instruction for Adc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        if cpu.p.decimal == false {
            let sum = (cpu.a as u16) + (cpu.ops.dl as u16) + (cpu.p.carry as u16); 
            cpu.p.carry = if sum > 255 { true } else {false };

            let result = sum as u8;
            cpu.a = result;
            cpu.p.overflow =  if ((cpu.ops.dl ^ result) & (cpu.a & result) & 0x80) == 0x80 { true } else { false };
            cpu.p.zero = set_zero(cpu.a);
            cpu.p.negative = set_negative(cpu.a);
        }
        else {
            panic!("decimal mode uniplemented");
        }
    }
}

pub struct And {}
impl Instruction for And {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let a = cpu.a & cpu.ops.dl;
        cpu.a = a;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Asl {}
impl Instruction for Asl {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct AslAccum {}
impl Instruction for AslAccum {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.a & 0x80) > 0 { true } else { false };
        cpu.a = cpu.a.wrapping_mul(2);

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Bcc {}
impl Instruction for Bcc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.carry == false { true } else { false };
    }
}

pub struct Bcs {}
impl Instruction for Bcs {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.carry == true { true } else { false };
    }
}

pub struct Beq {}
impl Instruction for Beq {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.zero == true { true } else { false };
    }
}

pub struct Bit {}
impl Instruction for Bit {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let x = cpu.a & cpu.ops.dl;
        cpu.p.negative = if (cpu.ops.dl & 0x80) == 0x80 { true } else { false };
        cpu.p.overflow = if (cpu.ops.dl & 0x40) == 0x40 { true } else { false };
        cpu.p.zero = if x == 0 { true } else { false };
    }
}

pub struct Bmi {}
impl Instruction for Bmi {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.negative == true { true } else { false };
    }
}

pub struct Bne {}
impl Instruction for Bne {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.zero == false { true } else { false };
    }
}

pub struct Bpl {}
impl Instruction for Bpl {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.negative == false { true } else { false };
    }
}

pub struct Bvc {}
impl Instruction for Bvc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.overflow == false { true } else { false };
    }
}

pub struct Bvs {}
impl Instruction for Bvs {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.branch_taken = if cpu.p.overflow == true { true } else { false };
    }
}

pub struct Clc {}
impl Instruction for Clc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.carry = false;
    }
}

pub struct Cld {}
impl Instruction for Cld {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.decimal = false;
    }
}

pub struct Cli {}
impl Instruction for Cli {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.interrupt_disable = false;
    }
}

pub struct Clv {}
impl Instruction for Clv {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.overflow = false;
    }
}

pub struct Cmp {}
impl Instruction for Cmp {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.carry = if cpu.a >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.a == cpu.ops.dl { true } else {false };
        cpu.p.negative = set_negative(cpu.a.wrapping_sub(cpu.ops.dl));
    }
}

pub struct Cpx {}
impl Instruction for Cpx {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.carry = if cpu.x >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.x == cpu.ops.dl { true } else {false };
        cpu.p.negative = set_negative(cpu.x.wrapping_sub(cpu.ops.dl));
    }
}

pub struct Cpy {}
impl Instruction for Cpy {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.carry = if cpu.y >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.y == cpu.ops.dl { true } else {false };
        cpu.p.negative = set_negative(cpu.y.wrapping_sub(cpu.ops.dl));
    }
}

pub struct Dec {}
impl Instruction for Dec {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.ops.dl.wrapping_sub(1);
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct Dex {}
impl Instruction for Dex {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.x.wrapping_sub(1);
        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Dey {}
impl Instruction for Dey {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.y.wrapping_sub(1);
        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Eor {}
impl Instruction for Eor {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.a ^ cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Inc {}
impl Instruction for Inc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.ops.dl.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct Inx {}
impl Instruction for Inx {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.x.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Iny {}
impl Instruction for Iny {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.y.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Lda {}
impl Instruction for Lda {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Ldx {}
impl Instruction for Ldx {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Ldy {}
impl Instruction for Ldy {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Lsr {}
impl Instruction for Lsr {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let old_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        // clear bit 7
        cpu.ops.dl &= 0b01111111;

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct LsrAccum {}
impl Instruction for LsrAccum {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let old_carry = if (cpu.a & 0x01) > 0 { true } else { false };

        cpu.a = cpu.a.wrapping_div(2);
        // clear bit 7
        cpu.a &= 0b01111111;

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Nop {}
impl Instruction for Nop {
    fn execute(_cpu: &mut Context) {
        // causes no changes to processor
    }
}

pub struct Ora {}
impl Instruction for Ora {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.a | cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Rol {}
impl Instruction for Rol {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);
        cpu.ops.dl |= cpu.p.carry as u8;

        cpu.p.carry = new_carry;
        cpu.p.negative = set_negative(cpu.ops.dl);
        cpu.p.zero = set_zero(cpu.ops.dl);
    }
}

pub struct RolAccum {}
impl Instruction for RolAccum {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.a & 0x80) > 0 { true } else { false };
        cpu.a = cpu.a.wrapping_mul(2);
        cpu.a |= cpu.p.carry as u8;

        cpu.p.carry = new_carry;
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.zero = set_zero(cpu.a);
    }
}

pub struct Ror {}
impl Instruction for Ror {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        cpu.ops.dl |= (cpu.p.carry as u8) << 7;

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct RorAccum {}
impl Instruction for RorAccum {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.a & 0x01) > 0 { true } else { false };

        cpu.a = cpu.a.wrapping_div(2);
        cpu.a |= (cpu.p.carry as u8) << 7;

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct SbcNoDec {}
impl Instruction for SbcNoDec {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
    /* 
    let (x1, o1) = cpu.a.overflowing_sub(cpu.ops.dl);
    let (x2, o2) = x1.overflowing_sub(!cpu.p.carry as u8);
    cpu.p.carry = !(o1 | o2);

    let signed_sub = (cpu.a as i8 as i16) - (cpu.ops.dl as i8 as i16) - (1 - (cpu.p.carry as i16));
    cpu.a = x2;
    cpu.p.overflow = (signed_sub < -128) || (signed_sub > 127);
    cpu.p.zero = set_zero(cpu.a);
    cpu.p.negative = set_negative(cpu.a);
    */
    let dl = cpu.ops.dl ^ 0xFF;
    //let sum = cpu.a.wrapping_add(dl).wrapping_add(cpu.p.carry as u8);
    let sum = (cpu.a as u16) + (dl as u16) + cpu.p.carry as u16;
    let result = (sum & 0xFF) as u8;
    cpu.p.carry = if sum > 255 { true } else { false };
    cpu.p.overflow = if ((cpu.a ^ result) & (dl ^ result) & 0x80) != 0 { true } else { false };
    cpu.a = result;
    cpu.p.negative = set_negative(cpu.a);
    cpu.p.zero = set_zero(cpu.a);  
    }
}

pub struct Sbc {}
impl Instruction for Sbc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        if cpu.p.decimal == false {
            let dl = cpu.ops.dl ^ 0xFF;
            //let sum = cpu.a.wrapping_add(dl).wrapping_add(cpu.p.carry as u8);
            let sum = (cpu.a as u16) + (dl as u16) + cpu.p.carry as u16;
            let result = (sum & 0xFF) as u8;
            cpu.p.carry = if sum > 255 { true } else { false };
            cpu.p.overflow = if ((cpu.a ^ result) & (dl ^ result) & 0x80) != 0 { true } else { false };
            cpu.a = result;
            cpu.p.negative = set_negative(cpu.a);
            cpu.p.zero = set_zero(cpu.a);
        }
        else {
            panic!("decimal mode not implemented");
        }    
    }
}

pub struct Sec {}
impl Instruction for Sec {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.carry = true;
    }
}

pub struct Sed {}
impl Instruction for Sed {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.decimal = true;
    }
}

pub struct Sei {}
impl Instruction for Sei {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.p.interrupt_disable = true;
    }
}

pub struct Sta {}
impl Instruction for Sta {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.a;
    }
}

pub struct Stx {}
impl Instruction for Stx {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.x;
    }
}

pub struct Sty {}
impl Instruction for Sty {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.y;
    }
}

pub struct Tax {}
impl Instruction for Tax {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.a;

        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Tay {}
impl Instruction for Tay {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.y = cpu.a;

        cpu.p.zero = set_zero(cpu.y);
        cpu.p.negative = set_negative(cpu.y);
    }
}

pub struct Tsx {}
impl Instruction for Tsx {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.sp;

        cpu.p.zero = set_zero(cpu.x);
        cpu.p.negative = set_negative(cpu.x);
    }
}

pub struct Txa {}
impl Instruction for Txa {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.x;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Txs {}
impl Instruction for Txs {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.sp = cpu.x;
    }
}

pub struct Tya {}
impl Instruction for Tya {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.y;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

//====================================================
// undocumented opcodes
//====================================================
pub struct Aac {}
impl Instruction for Aac {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a &= cpu.ops.dl;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.carry = if cpu.p.negative == true { true } else { false };
    }
}

pub struct Aax {}
impl Instruction for Aax {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.a & cpu.x;
        
        //cpu.p.zero = set_zero(cpu.ops.dl);
        //cpu.p.negative = set_negative(cpu.ops.dl);
    }
}

pub struct Arr {}
impl Instruction for Arr {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a &= cpu.ops.dl;
        // rotate right
        cpu.a = cpu.a.wrapping_div(2);
        cpu.a |= (cpu.p.carry as u8) << 7;

        let mask = cpu.a & 0b01100000;
        match mask {
            0b01100000 => {
                cpu.p.carry = true;
                cpu.p.overflow = false;
            }
            0b00000000 => {
                cpu.p.carry = false;
                cpu.p.overflow = false;
            }
            0b00100000 => {
                cpu.p.carry = false;
                cpu.p.overflow = true;
            }
            0b01000000 => {
                cpu.p.carry = true;
                cpu.p.overflow = true;
            }
            _ => panic!("Arr instruction"),
        }
    }
}

pub struct Asr {}
impl Instruction for Asr {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        
        let old_carry = if (cpu.a & 0x01) > 0 { true } else { false };
        cpu.a &= cpu.ops.dl;
        // rotate right
        cpu.a = cpu.a.wrapping_div(2);

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Atx {}
impl Instruction for Atx {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a &= cpu.ops.dl;
        cpu.x = cpu.a;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Axa {}
impl Instruction for Axa {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.a & cpu.x & (cpu.ops.adh.wrapping_add(1));

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Axs {}
impl Instruction for Axs {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.x = cpu.a & cpu.x;
        let (x, c) = cpu.x.overflowing_sub(cpu.ops.dl);
        cpu.x = x;
        cpu.p.carry = c;
    }
}

pub struct Dcp {}
impl Instruction for Dcp {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let (x, _c) = cpu.ops.dl.overflowing_sub(1);
        cpu.ops.dl = x;
        let result = cpu.a.wrapping_sub(cpu.ops.dl);
        cpu.p.carry = if cpu.a >= cpu.ops.dl { true } else { false };
        cpu.p.zero = if cpu.a == cpu.ops.dl { true } else { false }; 
        cpu.p.negative = set_negative(result);
    }
}

pub struct Isc {}
impl Instruction for Isc {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.ops.dl.wrapping_add(1);
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);
        
        let dl = cpu.ops.dl ^ 0xFF;
        let sum = (cpu.a as u16) + (dl as u16) + cpu.p.carry as u16;
        let result = (sum & 0xFF) as u8;
        cpu.p.carry = if sum > 255 { true } else { false };
        cpu.p.overflow = if ((cpu.a ^ result) & (dl ^ result) & 0x80) != 0 { true } else { false };
        cpu.a = result;
        cpu.p.negative = set_negative(cpu.a);
        cpu.p.zero = set_zero(cpu.a);
    }
}

pub struct Kil {}
impl Instruction for Kil {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let mut addr = u16::from(cpu.pc);
       // halt pc, lock up cpu
        addr -= 1;
        cpu.pc = ProgramCounter::from(addr);
    }
}

pub struct Lar {}
impl Instruction for Lar {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.sp & cpu.ops.dl;
        cpu.x = cpu.a;
        cpu.sp = cpu.a;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Lax {}
impl Instruction for Lax {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.ops.dl;
        cpu.x = cpu.ops.dl;

        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Rla {}
impl Instruction for Rla {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);
        cpu.ops.dl |= cpu.p.carry as u8;

        cpu.p.carry = new_carry;
        cpu.p.negative = set_negative(cpu.ops.dl);
        cpu.p.zero = set_zero(cpu.ops.dl);

        cpu.a = cpu.a & cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Rra {}
impl Instruction for Rra {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        cpu.ops.dl |= (cpu.p.carry as u8) << 7;

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);

        let sum = (cpu.a as u16) + (cpu.ops.dl as u16) + (cpu.p.carry as u16); 
        cpu.p.carry = if sum > 255 { true } else {false };

        let result = sum as u8;
        cpu.a = result;
       // cpu.p.overflow =  if (signed_sum < -128) || (signed_sum > 127) { true } else { false };
        cpu.p.overflow =  if ((cpu.ops.dl ^ result) & (cpu.a & result) & 0x80) == 0x80 { true } else { false };
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);

    }
}

pub struct Slo {}
impl Instruction for Slo {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let new_carry = if (cpu.ops.dl & 0x80) > 0 { true } else { false };
        cpu.ops.dl = cpu.ops.dl.wrapping_mul(2);

        cpu.p.carry = new_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);

        cpu.a = cpu.a | cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);
    }
}

pub struct Sre {}
impl Instruction for Sre {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        let old_carry = if (cpu.ops.dl & 0x01) > 0 { true } else { false };

        cpu.ops.dl = cpu.ops.dl.wrapping_div(2);
        // clear bit 7
        cpu.ops.dl &= 0b01111111;

        cpu.p.carry = old_carry;
        cpu.p.zero = set_zero(cpu.ops.dl);
        cpu.p.negative = set_negative(cpu.ops.dl);

        cpu.a = cpu.a ^ cpu.ops.dl;
        cpu.p.zero = set_zero(cpu.a);
        cpu.p.negative = set_negative(cpu.a);    
    }
}

pub struct Sxa {}
impl Instruction for Sxa {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.x & (cpu.ops.adh + 1);  
    }
}

pub struct Sya {}
impl Instruction for Sya {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.ops.dl = cpu.y & (cpu.ops.adh + 1);  
    }
}

pub struct Xaa {}
impl Instruction for Xaa {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.a = cpu.x;
        // TODO: A is anded with some unkown immediate value
        cpu.a &= cpu.ops.dl;
    }
}

pub struct Xas {}
impl Instruction for Xas {
    #[inline(always)]
    fn execute(cpu: &mut Context) {
        cpu.sp = cpu.x & cpu.a;
        cpu.ops.dl = cpu.sp & (cpu.ops.adh + 1);
    }
}
