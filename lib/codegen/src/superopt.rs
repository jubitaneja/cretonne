// Superoptimization pass

use cursor::{Cursor, FuncCursor};
use ir::function::Function;
use isa::TargetIsa;
use timing;
use ir::Inst;

// Optimization driver function that will invoke all
// the optimization transformations.
fn opt_driver(pos: &mut FuncCursor, inst: Inst) {
    println!("** Inst Name === {}", pos.func.dfg.display_inst(inst, None));
}

// The main superoptimization pass
pub fn do_superopt(func: &mut Function, isa: &TargetIsa) {
    let _tt = timing::superopt();
    let mut pos = FuncCursor::new(func);
    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            // add transformations here
            opt_driver(&mut pos, inst);
        }
    }
}
