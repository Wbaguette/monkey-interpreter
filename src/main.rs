mod lexer;
mod parser;
mod repl;
mod helper;
mod tests;

use std::io::{self, Write};
use color_eyre::Result;

fn main() -> Result<()> {
   let _ = color_eyre::install();
   
   let stdin: io::Stdin = io::stdin();
   let stdout: io::Stdout = io::stdout();
   let reader: io::StdinLock<'_> = stdin.lock();
   let mut writer: io::StdoutLock<'_> = stdout.lock();

   write!(writer, "Hello!, This is the Monkey Programming Language!\n")?;
   write!(writer, "Feel free to type in commands.\n")?;
   repl::start(reader, writer);

   Ok(())
}