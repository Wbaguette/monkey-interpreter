/*
   Author: Jean-Pierre Derbes
   Email: jderbes2021@my.fit.edu
   School: Florida Institute of Technology 

   References: https://interpreterbook.com/

   Short Description:   This is a Rust implementation of the above referenced book. 
                        The book uses Golang to teach an introduction to Interpreters.
                        I followed along with the book, but decided to implement in Rust.

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

use std::{process, env, fs, path::Path};
use std::io::{self, Write};
use color_eyre::{Result, eyre::eyre, owo_colors::OwoColorize};
use objects::Null;

use crate::lexer::Lexer;
use crate::objects::environment::Environment;
use crate::parser::Parser;
use crate::parser::ast::Program;

fn main() -> Result<()> {
   color_eyre::install()?;
   
   let args: Vec<String> = env::args().collect();
   if args.len() > 2 {
      // Too many arguments 
      return Err(eyre!("Usage: 'cargo run <filepath>'"))
   } else if args.len() == 2 {
      // Correct number of arguments to use a ".mky" file
      handle_file_path(args.get(1).unwrap().clone())?;
   } else {
      // Else, we go into the repl as no file paths were passed
      start_repl()?;
   }

   Ok(())
}

fn handle_file_path(path_str: String) -> Result<()> {
   let path: &Path = Path::new(&path_str);
   if !path.exists() {
      return Err(eyre!("Path {} does not exist.", path.display()))
   }
   if !path.is_file() {
      return Err(eyre!("Path {} is not a file.", path.display()))
   }
   match path.extension() {
      Some(extension) => {
         if extension.to_ascii_lowercase() != "mky" {
            return Err(eyre!("File extension is not not accepted. \nWant: \"mky\" Got: {:?}", extension))
         }
         
         let file_content: String = fs::read_to_string(path)?;
         println!();
         eval_mky_file(file_content)?
      }
      None => return Err(eyre!("File extension could not be read for path {}.", path.display()))
   }

   Ok(())
}

fn eval_mky_file(file_content: String) -> Result<()> {
   let mut env: Environment = Environment::new();
   let lexer: Lexer = Lexer::new(file_content);
   let mut parser: Parser = Parser::new(lexer);
   let program: Program = match parser.parse_program() {
      Ok(p) => p,
      Err(e) => return Err(e)
   };
   if !parser.errors().is_empty() {
      print_parser_errors(&parser.errors());
      process::exit(0)
   }
   
   match evaluator::eval(Box::new(&program), &mut env) {
      Some(e) => {
         // Maybe introduce a special object that is returned when the evaluation is without error and finished
         if e.as_any().is::<Null>() {
            eprintln!("\n{}", "Process exited successfully.".green().bold())
         } else {
            eprintln!("\n{:?}", e.red().bold())
         }
      },
      None => eprintln!("\n{}", "Process terminated, error in source code".red().bold())
   }

   Ok(())
}

fn print_parser_errors(errors: &[String]) {
   print!("{}", "Whoops!, We ran into some monkey business here.\n".bold().bright_red());
   print!("{}", " parser errors:\n".bold().bright_green());

   for msg in errors {
      print!("\t{}\n", msg)
   }
}

fn start_repl() -> Result<()> {
   let stdin: io::Stdin = io::stdin();
   let stdout: io::Stdout = io::stdout();
   let reader: io::StdinLock<'_> = stdin.lock();
   let mut writer: io::StdoutLock<'_> = stdout.lock();

   write!(writer, "Hello!, This is the Monkey Programming Language!\n")?;
   write!(writer, "Feel free to type in commands.\n")?;
   repl::start(reader, writer);
   Ok(())
}


// C game engine using SDL2 and OpenGL for a tower defense game (WIP)
// Interpreter in Rust for a mock programming language called "Monkey"   (Types, Arrays, HashMaps, Array and HashMap indexing, Functions, Closures)
// 8086 Assembly : Bloons tower defense 
// Java : SpUI     Java app using JavaFX using Spotify's web api to give users more features (top artists, top songs, true playlist shuffler)




// About me:
// My name is Jean-Pierre, but most people call me JP. I am a rising senior in Computer Science graduating in May of 2025. I've been programming for 3 and a half years. I am mainly 
// interested in embedded systems, networks, machine learning, and 


// How do my projects relate to the job? Make me qualified for the job?
// Database Systems Class : Used AWS to make a database for a project and queried the database using MySQL. 
// All of my projects have made me a more knowledgeable, capable, and confident programmer 

// What do I hope to gain from this internship?
// Mention data security and networks as  something I am really interested in 
// skill development, industry experience, professional growth, networking, or contributing to meaningful projects.