#[allow(unused)]

use std::io::{BufRead, Write};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) {
   loop {
      print!(">> ");
      Write::flush(&mut writer).expect("Failed to flush output");
      let mut line: String = String::new();
      match reader.read_line(&mut line) {
         Ok(0) => continue,
         Ok(_) => {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
               continue;
            }

            let mut lexer: Lexer = Lexer::new(trimmed_line.to_string());
            loop {
               let tok: Token = lexer.next_token().unwrap();
               match tok.token_type {
                  TokenType::EOF => break,
                  _ => write!(writer, "{:?}\n", tok).expect("Failed to write Token"),
               }
            }
         },
         Err(e) => println!("Error reading input: {}", e),
      }
   }
}