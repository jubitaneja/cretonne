// Superoptimization pass

use cursor::{Cursor, FuncCursor};
use ir::function::Function;
use isa::TargetIsa;
use timing;
use ir::Inst;
use ir::InstructionData;
use ir::instructions::Opcode;

// Optimization driver function that will invoke all
// the optimization transformations.
fn opt_driver(pos: &mut FuncCursor, inst: Inst) {
    println!("** Inst Name === {}", pos.func.dfg.display_inst(inst, None));
    match pos.func.dfg[inst] {
        InstructionData::Binary { opcode, args } => {
            let new_opcode = match opcode {
                Opcode::Iadd => {
                    println!("Found iadd");
                },
                Opcode::Imul => {
                    println!("Found imul");
                },
                _ => {
                    println!("This binary ppcode is not yet handled");
                },
            };
        },
        _ => {
            println!("Instruction Kind not yet handled");
        },
    }
}

// The main superoptimization pass
pub fn do_superopt(func: &mut Function, isa: &TargetIsa) {
    let _tt = timing::superopt();
    let mut pos = FuncCursor::new(func);

    println!("==============================");
    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            // add transformations here
            opt_driver(&mut pos, inst);
        }
    }
    println!("==============================");

}
