//! Test command for checking the IR legalizer.
//!
//! The `test legalizer` test command runs each function through `legalize_function()` and sends
//! the result to filecheck.

use cretonne_codegen;
use cretonne_codegen::ir::Function;
use cretonne_codegen::print_errors::pretty_error;
use cretonne_reader::TestCommand;
use std::borrow::Cow;
use subtest::{run_filecheck, Context, SubTest, SubtestResult};

struct TestLegalizer;

pub fn subtest(parsed: &TestCommand) -> SubtestResult<Box<SubTest>> {
    assert_eq!(parsed.command, "legalizer");
    if !parsed.options.is_empty() {
        Err(format!("No options allowed on {}", parsed))
    } else {
        Ok(Box::new(TestLegalizer))
    }
}

impl SubTest for TestLegalizer {
    fn name(&self) -> &'static str {
        "legalizer"
    }

    fn is_mutating(&self) -> bool {
        true
    }

    fn needs_isa(&self) -> bool {
        true
    }

    fn run(&self, func: Cow<Function>, context: &Context) -> SubtestResult<()> {
        let mut comp_ctx = cretonne_codegen::Context::for_function(func.into_owned());
        let isa = context.isa.expect("legalizer needs an ISA");

        comp_ctx.compute_cfg();
        comp_ctx
            .legalize(isa)
            .map_err(|e| pretty_error(&comp_ctx.func, context.isa, e))?;

        let text = comp_ctx.func.display(Some(isa)).to_string();
        run_filecheck(&text, context)
    }
}
