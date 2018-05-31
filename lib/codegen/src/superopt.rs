// Superoptimization pass
//
use ir::function::Function;
use isa::TargetIsa;

// The main superoptimization pass
pub fn do_superopt(func: &mut Function, isa: &TargetIsa) {
    println!("In superopt pass");
}
