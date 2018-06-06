// Superoptimization pass

use cursor::{Cursor, FuncCursor};
use isa::TargetIsa;
use timing;
use ir::Inst;
use ir::instructions::Opcode;
use ir::types::I32;
use ir::dfg::ValueDef;
use ir::{Function, InstBuilder, InstructionData};

// Optimization driver function that will invoke all
// the optimization transformations.

fn opt_driver(pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
        //
        // Optimization transformation x + x + x => 3 * x
        //
        InstructionData::Binary { opcode, args } => {
            match opcode {
                Opcode::Iadd => {
                    // check the type of args
                    if let ValueDef::Param(_, _) = pos.func.dfg.value_def(args[0]) {
                        if let ValueDef::Result(arg_ty, _) = pos.func.dfg.value_def(args[1]) {
                            if let InstructionData::Binary {
                                opcode: Opcode::Iadd, args,
                            } = pos.func.dfg[arg_ty]
                            {
                                if let ValueDef::Param(_, _) = pos.func.dfg.value_def(args[0]) {
                                    if let ValueDef::Param(_, _) = pos.func.dfg.value_def(args[1]) {
                                        pos.func
                                            .dfg
                                            .replace(inst)
                                            .imul_imm(args[1], 3);
                                    }
                                }
                            }
                        }
                    } else if let ValueDef::Param(_, _) = pos.func.dfg.value_def(args[1]) {
                        if let ValueDef::Result(arg_ty, _) = pos.func.dfg.value_def(args[0]) {
                            if let InstructionData::Binary {
                                opcode: Opcode::Iadd, args,
                            } = pos.func.dfg[arg_ty]
                            {
                                if let ValueDef::Param(_, _) = pos.func.dfg.value_def(args[0]) {
                                    if let ValueDef::Param(_, _) = pos.func.dfg.value_def(args[1]) {
                                        pos.func
                                            .dfg
                                            .replace(inst)
                                            .imul_imm(args[1], 3);
                                    }
                                }
                            }
                        }
                    } else {
                        return;
                    }
                },
                _ => (),
            };
        },
        //
        // Optimization found by Souper on binaryen bullet benchmark
        //
        // %0:i32 = var
        // %1:i32 = var
        // %2:i32 = add 4294967295:i32, %1
        // %3:i32 = shl %2, 2:i32
        // %4:i32 = add %0, %3
        // %5:i32 = add 348:i32, %4
        // infer %5
        // %6:i32 = shl %1, 2:i32
        // %7:i32 = sub %6, 4294966952:i32
        // %8:i32 = add %0, %7
        // result %8
        //
        InstructionData::BinaryImm { opcode, arg, imm } => {
            match opcode {
                Opcode::IaddImm => {
                    let rhs: i64 = imm.into(); 
                    if rhs == 348 {
                        if let ValueDef::Result(arg_ty, _) = pos.func.dfg.value_def(arg) {
                            if let InstructionData::Binary {
                                opcode: Opcode::Iadd,
                                args,
                            } = pos.func.dfg[arg_ty]
                            {
                                let params = args;
                                if let ValueDef::Param(_param_ty, _) = pos.func.dfg.value_def(args[0]) {
                                    if let ValueDef::Result(shift_ty, _) = pos.func.dfg.value_def(args[1]) {
                                        if let InstructionData::Binary {
                                            opcode: Opcode::Ishl,
                                            args,
                                        } = pos.func.dfg[shift_ty]
                                        {
                                            if let ValueDef::Result(shift_amt, _) = pos.func.dfg.value_def(args[1]) {
                                                if let InstructionData::UnaryImm {
                                                    opcode: Opcode::Iconst,
                                                    imm,
                                                } = pos.func.dfg[shift_amt]
                                                {
                                                    let amt: i64 = imm.into();
                                                    if amt == 2 {
                                                        if let ValueDef::Result(imm_add, _) = pos.func.dfg.value_def(args[0]) {
                                                            if let InstructionData::BinaryImm {
                                                                opcode: Opcode::IaddImm,
                                                                arg,
                                                                imm,
                                                            } = pos.func.dfg[imm_add]
                                                            {
                                                                if let ValueDef::Param(_param_1, _) = pos.func.dfg.value_def(arg) {
                                                                    let bignum: i64 = imm.into();
                                                                    if bignum == 4294967295 {
                                                                        // pattern matching done
                                                                        let inst6 = pos.ins()
                                                                                       .ishl_imm(arg, 2);
                                                                        let constinst = pos.ins()
                                                                                           .iconst(I32, 4294967295);
                                                                        let inst7 = pos.ins()
                                                                                       .isub(inst6, constinst);
                                                                        pos.func.dfg
                                                                                .replace(inst)
                                                                                .iadd(params[0], inst7);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => (),
            }
        },
        _ => (),
    }
}

// The main superoptimization pass
pub fn do_superopt(func: &mut Function, _isa: &TargetIsa) {
    let _tt = timing::superopt();
    let mut pos = FuncCursor::new(func);

    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            opt_driver(&mut pos, inst);
        }
    }
}
