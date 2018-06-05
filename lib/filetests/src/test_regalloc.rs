//! Test command for testing the register allocator.
//!
//! The `regalloc` test command runs each function through the register allocator after ensuring
//! that all instructions are legal for the target.
//!
//! The resulting function is sent to `filecheck`.

use cretonne_codegen;
use cretonne_codegen::ir::Function;
use cretonne_codegen::print_errors::pretty_error;
use cretonne_reader::TestCommand;
use std::borrow::Cow;
use subtest::{run_filecheck, Context, SubTest, SubtestResult};

struct TestRegalloc;

pub fn subtest(parsed: &TestCommand) -> SubtestResult<Box<SubTest>> {
    assert_eq!(parsed.command, "regalloc");
    if !parsed.options.is_empty() {
        Err(format!("No options allowed on {}", parsed))
    } else {
        Ok(Box::new(TestRegalloc))
    }
}

impl SubTest for TestRegalloc {
    fn name(&self) -> &'static str {
        "regalloc"
    }

    fn is_mutating(&self) -> bool {
        true
    }

    fn needs_isa(&self) -> bool {
        true
    }

    fn run(&self, func: Cow<Function>, context: &Context) -> SubtestResult<()> {
        let isa = context.isa.expect("register allocator needs an ISA");
        let mut comp_ctx = cretonne_codegen::Context::for_function(func.into_owned());

        comp_ctx.compute_cfg();
        // TODO: Should we have an option to skip legalization?
        comp_ctx
            .legalize(isa)
            .map_err(|e| pretty_error(&comp_ctx.func, context.isa, e))?;
        comp_ctx.compute_domtree();
        comp_ctx
            .regalloc(isa)
            .map_err(|e| pretty_error(&comp_ctx.func, context.isa, e))?;

        let text = comp_ctx.func.display(Some(isa)).to_string();
        run_filecheck(&text, context)
    }
}
