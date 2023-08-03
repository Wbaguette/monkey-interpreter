use std::io::{BufRead, Write};
use color_eyre::owo_colors::OwoColorize;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::ast::Program;
use crate::objects::Object;
use crate::evaluator;

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) {
   loop {
      write!(writer, "{}", PROMPT).expect("Failed to write prompt");
      writer.flush().expect("Failed to flush output");

      let mut line: String = String::new();
      match reader.read_line(&mut line) {
         Ok(0) => continue,
         Ok(_) => {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
               continue;
            }

            let lexer: Lexer = Lexer::new(trimmed_line.to_string());
            let mut parser: Parser = Parser::new(lexer);

            let program: Program = match parser.parse_program() {
               Ok(p) => p,
               Err(e) => panic!("{}", e),
            };
            if !parser.errors().is_empty() {
               print_parser_errors(&mut writer, &parser.errors());
               continue;
            }
            
            let evaluated: Option<Box<dyn Object>> = evaluator::eval(Box::new(&program));
            match evaluated {
               Some(e) => {
                  write!(writer, "{}\n", e.inspect()).expect("Failed to write Evaluation")
               },
               None => {}
            }

            // write!(writer,"{}\n", program.string()).expect("Failed to write AST");
         },
         Err(e) => println!("Error reading input: {}", e),
      }
   }
}

fn print_parser_errors<W: Write>(writer: &mut W, errors: &[String]) {
   write!(writer, "{}", "Whoops!, We ran into some monkey business here.\n".bold().bright_red()).expect("Failed to write MONKEY_FACE");
   write!(writer, "{}", " parser errors:\n".bold().bright_green()).expect("Failed to write parser errors clause");

   for msg in errors {
      write!(writer, "\t{}\n", msg).expect("Failed to write parser error")
   }
}
