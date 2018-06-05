//! Test command for checking the IR verifier.
//!
//! The `test verifier` test command looks for annotations on instructions like this:
//!
//! ```cton
//!     jump ebb3 ; error: jump to non-existent EBB
//! ```
//!
//! This annotation means that the verifier is expected to given an error for the jump instruction
//! containing the substring "jump to non-existent EBB".

use cretonne_codegen::ir::Function;
use cretonne_codegen::verify_function;
use cretonne_reader::TestCommand;
use match_directive::match_directive;
use std::borrow::{Borrow, Cow};
use subtest::{Context, SubTest, SubtestResult};

struct TestVerifier;

pub fn subtest(parsed: &TestCommand) -> SubtestResult<Box<SubTest>> {
    assert_eq!(parsed.command, "verifier");
    if !parsed.options.is_empty() {
        Err(format!("No options allowed on {}", parsed))
    } else {
        Ok(Box::new(TestVerifier))
    }
}

impl SubTest for TestVerifier {
    fn name(&self) -> &'static str {
        "verifier"
    }

    fn needs_verifier(&self) -> bool {
        // Running the verifier before this test would defeat its purpose.
        false
    }

    fn run(&self, func: Cow<Function>, context: &Context) -> SubtestResult<()> {
        let func = func.borrow();

        // Scan source annotations for "error:" directives.
        let mut expected = None;
        for comment in &context.details.comments {
            if let Some(tail) = match_directive(comment.text, "error:") {
                // Currently, the verifier can only report one problem at a time.
                // Reject more than one `error:` directives.
                if expected.is_some() {
                    return Err("cannot handle multiple error: directives".to_string());
                }
                expected = Some((comment.entity, tail));
            }
        }

        match verify_function(func, context.flags_or_isa()) {
            Ok(_) => match expected {
                None => Ok(()),
                Some((_, msg)) => Err(format!("passed, expected error: {}", msg)),
            },
            Err(got) => match expected {
                None => Err(format!("verifier pass, got {}", got)),
                Some((want_loc, want_msg)) if got.message.contains(want_msg) => {
                    if want_loc == got.location {
                        Ok(())
                    } else {
                        Err(format!(
                            "correct error reported on {}, but wanted {}",
                            got.location, want_loc
                        ))
                    }
                }
                Some(_) => Err(format!("mismatching error: {}", got)),
            },
        }
    }
}
