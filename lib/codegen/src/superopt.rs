// Superoptimization pass

use cursor::{Cursor, FuncCursor};
//use ir::function::Function;
use isa::TargetIsa;
use timing;
use ir::Inst;
//use ir::InstructionData;
use ir::instructions::Opcode;
use ir::dfg::ValueDef;
use ir::{Function, InstBuilder, InstructionData};

// Optimization driver function that will invoke all
// the optimization transformations.

fn opt_driver(pos: &mut FuncCursor, inst: Inst) {
    match pos.func.dfg[inst] {
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
                    }
                    else {
                        return;
                    }
                },
                _ => {},
            };
        },
        _ => {},
    }
}

// The main superoptimization pass
pub fn do_superopt(func: &mut Function, _isa: &TargetIsa) {
    let _tt = timing::superopt();
    let mut pos = FuncCursor::new(func);

    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            // add transformations here
            opt_driver(&mut pos, inst);
        }
    }
}
