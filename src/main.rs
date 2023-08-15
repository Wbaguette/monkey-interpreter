/*
   Author: Jean-Pierre Derbes
   Email: jderbes2021@my.fit.edu
   School: Florida Institute of Technology 

   References: https://interpreterbook.com/

   Short Description:   This is a Rust implementation of the above referenced book. 
                        The book uses Golang to teach an introduction to Interpreters.

                        I would highly recommend checking out this website and book as I
                        personally learned a ton from it. (No, I am not sponsored)

*/

mod lexer;
mod parser;
mod repl;
mod helper;
mod objects;
mod evaluator;
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
