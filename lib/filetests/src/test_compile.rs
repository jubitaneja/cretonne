//! Test command for testing the code generator pipeline
//!
//! The `compile` test command runs each function through the full code generator pipeline

use cretonne_codegen;
use cretonne_codegen::print_errors::pretty_error;
use cretonne_codegen::{binemit, ir};
use cretonne_reader::TestCommand;
use std::borrow::Cow;
use subtest::{run_filecheck, Context, SubTest, SubtestResult};

struct TestCompile;

pub fn subtest(parsed: &TestCommand) -> SubtestResult<Box<SubTest>> {
    assert_eq!(parsed.command, "compile");
    if !parsed.options.is_empty() {
        Err(format!("No options allowed on {}", parsed))
    } else {
        Ok(Box::new(TestCompile))
    }
}

impl SubTest for TestCompile {
    fn name(&self) -> &'static str {
        "compile"
    }

    fn is_mutating(&self) -> bool {
        true
    }

    fn needs_isa(&self) -> bool {
        true
    }

    fn run(&self, func: Cow<ir::Function>, context: &Context) -> SubtestResult<()> {
        let isa = context.isa.expect("compile needs an ISA");
        let mut comp_ctx = cretonne_codegen::Context::for_function(func.into_owned());

        let code_size = comp_ctx
            .compile(isa)
            .map_err(|e| pretty_error(&comp_ctx.func, context.isa, e))?;

        dbg!(
            "Generated {} bytes of code:\n{}",
            code_size,
            comp_ctx.func.display(isa)
        );

        // Verify that the returned code size matches the emitted bytes.
        let mut sink = SizeSink { offset: 0 };
        binemit::emit_function(
            &comp_ctx.func,
            |func, inst, div, sink| isa.emit_inst(func, inst, div, sink),
            &mut sink,
        );

        if sink.offset != code_size {
            return Err(format!(
                "Expected code size {}, got {}",
                code_size, sink.offset
            ));
        }

        // Run final code through filecheck.
        let text = comp_ctx.func.display(Some(isa)).to_string();
        run_filecheck(&text, context)
    }
}

/// Code sink that simply counts bytes.
struct SizeSink {
    offset: binemit::CodeOffset,
}

impl binemit::CodeSink for SizeSink {
    fn offset(&self) -> binemit::CodeOffset {
        self.offset
    }

    fn put1(&mut self, _: u8) {
        self.offset += 1;
    }

    fn put2(&mut self, _: u16) {
        self.offset += 2;
    }

    fn put4(&mut self, _: u32) {
        self.offset += 4;
    }

    fn put8(&mut self, _: u64) {
        self.offset += 8;
    }

    fn reloc_ebb(&mut self, _reloc: binemit::Reloc, _ebb_offset: binemit::CodeOffset) {}
    fn reloc_external(
        &mut self,
        _reloc: binemit::Reloc,
        _name: &ir::ExternalName,
        _addend: binemit::Addend,
    ) {
    }
    fn reloc_jt(&mut self, _reloc: binemit::Reloc, _jt: ir::JumpTable) {}
    fn trap(&mut self, _code: ir::TrapCode, _srcloc: ir::SourceLoc) {}
}
