mod defs;
mod generic;

use super::Interpreter;
use crate::{cpu::instructions::Unprefixed, Emu};

impl Emu<Interpreter> {
    pub(crate) fn fetch_decode_execute(&mut self) {
        let val = Unprefixed::from_byte(self.fetch());
        match val {
            Unprefixed::NOP => self.nop(),
            Unprefixed::STOP => self.stop(),
            Unprefixed::RLCA => self.rlca(),
            Unprefixed::RRCA => self.rrca(),
            Unprefixed::RLA => self.rla(),
            Unprefixed::RRA => self.rra(),
            Unprefixed::DAA => self.daa(),
            Unprefixed::CPL => self.cpl(),
            Unprefixed::SCF => self.scf(),
            Unprefixed::CCF => self.ccf(),
            Unprefixed::JR => self.jr(),
            Unprefixed::HALT => self.halt(),
            Unprefixed::RET => self.ret(),
            Unprefixed::RETI => self.reti(),
            Unprefixed::JPHL => self.jp_hl(),
            Unprefixed::JP => self.jp(),
            Unprefixed::DI => self.di(),
            Unprefixed::EI => self.ei(),
            Unprefixed::CALL => self.call(),
            Unprefixed::ADD_SP_I => self.add_sp_e8(),
            Unprefixed::CB => self.cb(),
            Unprefixed::RST(vec) => self.rst(vec),
            Unprefixed::PUSH(src) => self.push_r16(src),
            Unprefixed::POP(dst) => self.pop_r16(dst),
            Unprefixed::CALLCC(cc) => self.call_cc(cc),
            Unprefixed::JPCC(cc) => self.jp_cc(cc),
            Unprefixed::RETCC(cc) => self.ret_cc(cc),
            Unprefixed::JRCC(cc) => self.jr_cc(cc),
            Unprefixed::ADD_HL_R16(src) => self.add_hl_r16(src),
            Unprefixed::ADD_HL_SP => self.add_hl_sp(),
            Unprefixed::LD_PNN_SP => self.ld_pn16_sp(),
            Unprefixed::LD_PHLI_A => self.ld_phli_a(),
            Unprefixed::LD_PHLD_A => self.ld_phld_a(),
            Unprefixed::LDH_A_PN => self.ldh_a_pn8(),
            Unprefixed::LDH_PN_A => self.ldh_pn8_a(),
            Unprefixed::LDH_A_PC => self.ldh_a_pc(),
            Unprefixed::LDH_PC_A => self.ldh_pc_a(),
            Unprefixed::LD_A_PHLI => self.ld_a_phli(),
            Unprefixed::LD_A_PHLD => self.ld_a_phld(),
            Unprefixed::LD_R8_R8(dst, src) => self.ld_r8_r8(dst, src),
            Unprefixed::LD_R8_PHL(dst) => self.ld_r8_phl(dst),
            Unprefixed::LD_PHL_R8(src) => self.ld_phl_r8(src),
            Unprefixed::LD_R8_N(dst) => self.ld_r8_n8(dst),
            Unprefixed::LD_PHL_N => self.ld_phl_n8(),
            Unprefixed::LD_R16_NN(dst) => self.ld_r16_n16(dst),
            Unprefixed::LD_SP_NN => self.ld_sp_n16(),
            Unprefixed::LD_PR16_A(dst) => self.ld_pr16_a(dst),
            Unprefixed::LD_A_PR16(src) => self.ld_a_pr16(src),
            Unprefixed::LD_PNN_A => self.ld_pn16_a(),
            Unprefixed::LD_A_PNN => self.ld_a_pn16(),
            Unprefixed::LD_HL_SP_I => self.ld_hl_sp_e8(),
            Unprefixed::LD_SP_HL => self.ld_sp_hl(),
            Unprefixed::INC_R8(dst) => self.inc_r8(dst),
            Unprefixed::INC_PHL => self.inc_phl(),
            Unprefixed::INC_R16(dst) => self.inc_r16(dst),
            Unprefixed::INC_SP => self.inc_sp(),
            Unprefixed::DEC_R8(dst) => self.dec_r8(dst),
            Unprefixed::DEC_PHL => self.dec_phl(),
            Unprefixed::DEC_R16(dst) => self.dec_r16(dst),
            Unprefixed::DEC_SP => self.dec_sp(),
            Unprefixed::ADD_N => self.add_n8(),
            Unprefixed::ADD_R8(src) => self.add_r8(src),
            Unprefixed::ADD_PHL => self.add_phl(),
            Unprefixed::ADC_N => self.adc_n8(),
            Unprefixed::ADC_R8(src) => self.adc_r8(src),
            Unprefixed::ADC_PHL => self.adc_phl(),
            Unprefixed::SUB_N => self.sub_n8(),
            Unprefixed::SUB_R8(src) => self.sub_r8(src),
            Unprefixed::SUB_PHL => self.sub_phl(),
            Unprefixed::SBC_N => self.sbc_n8(),
            Unprefixed::SBC_R8(src) => self.sbc_r8(src),
            Unprefixed::SBC_PHL => self.sbc_phl(),
            Unprefixed::AND_N => self.and_n8(),
            Unprefixed::AND_R8(src) => self.and_r8(src),
            Unprefixed::AND_PHL => self.and_phl(),
            Unprefixed::XOR_N => self.xor_n8(),
            Unprefixed::XOR_R8(src) => self.xor_r8(src),
            Unprefixed::XOR_PHL => self.xor_phl(),
            Unprefixed::OR_N => self.or_n8(),
            Unprefixed::OR_R8(src) => self.or_r8(src),
            Unprefixed::OR_PHL => self.or_phl(),
            Unprefixed::CP_N => self.cp_n8(),
            Unprefixed::CP_R8(src) => self.cp_r8(src),
            Unprefixed::CP_PHL => self.cp_phl(),
            Unprefixed::INVALID => logger::fatal!("Attempted to executed invalid instruction"),
        }
    }
}
