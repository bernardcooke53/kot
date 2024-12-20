use crate::magik;
use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

pub const REPL_PROMPT: &str = ">>> ";

fn is_exit(line: String) -> bool {
    line == String::from("quit")
}

pub fn repl(foreign_object: String) -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    'repl: loop {
        match rl.readline(REPL_PROMPT) {
            Ok(line) => {
                let is_exit_call = is_exit(line.clone());
                println!(
                    "{}",
                    magik::with_consent(line, foreign_object.clone().into())
                );
                if is_exit_call {
                    break 'repl;
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(reason) => {
                println!("Error: {:?}", reason);
                break 'repl;
            }
        }
    }

    Ok(())
}
