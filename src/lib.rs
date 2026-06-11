pub mod core;
pub mod ast_printer;
use std::process;
use crate::core::Token;
use crate::core::TokenType;
use crate::core::scanner;
use crate::core::parser;
use crate::core::compiler;


#[derive (Debug, PartialEq)]
pub struct ParserError {
    error_msg: String,
    error_token: Token,
}

impl ParserError  { 
    pub fn parse_error(&self) {
        if self.error_token.token_type == TokenType::EOF {
            report(self.error_token.line, "at end", &self.error_msg)
        }
        else {
            report(self.error_token.line, format!("at '{}'", self.error_token.lexeme).as_str(), &self.error_msg);
        }
    }
}

#[derive(Debug)]
pub enum CompilerError {
    LLVMError(String),
    UndefinedVariable(String),
    TypeMismatch(String),
    VariableEnvironment(String),
}





pub fn scan_error (line:usize, ch:String, message: &str) {
    report(line, format!("at '{}'", ch).as_str(), message);
    process::exit(1)
}

fn report (line:usize, where_at:&str, message:&str) {
    eprintln!("[Line {line} ] Error {where_at} : {message}");
}


