mod ins;

use crate::ir;

use capstone::arch::x86::X86Insn;
use capstone::arch::x86::X86Operand;
use capstone::prelude::*;
use capstone::Insn;

pub fn opcode(ins: &Insn) -> X86Insn {
    unsafe { std::mem::transmute(ins.id().0) }
}

pub fn ins_operands<'a>(ins: &'a Insn, cs: &Capstone) -> Vec<X86Operand> {
    if let ArchDetail::X86Detail(detail) = cs
        .insn_detail(ins)
        .expect("failed getting instruction details")
        .arch_detail()
    {
        detail.operands().collect()
    } else {
        panic!("instruction detail is not x86")
    }
}

/// Lift instruction to IR
pub fn lift_ins(ins: &Insn, cs: &Capstone) -> ir::Stmt {
    let operands = ins_operands(ins, cs);

    match opcode(ins) {
        X86Insn::X86_INS_XOR => ins::lift_xor(&operands, cs),
        ins => panic!("instruction {:?} is unimplemented", ins),
    }
}

/// Lift bytes to IR
pub fn lift_bytes(bytes: &[u8], addr: u64) -> ir::Stmt {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode32)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");
    let insns = cs.disasm_all(bytes, addr).expect("failed to disassemble");
    for ins in insns.iter() {
        return lift_ins(&ins, &cs);
    }

    unimplemented!()
}