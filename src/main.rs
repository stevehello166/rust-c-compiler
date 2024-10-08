mod abstract_syntax_tree;
mod lexer;
mod generate_assembly;

use std::fs::read_to_string;
use std::fs::File;
use regex::Regex;

use abstract_syntax_tree::create_abstract_syntax_tree;
use generate_assembly::generate_assembly;

#[cfg(test)]
mod tests;

fn main() {
    let source = "src/tests/generaltests/stage_1/valid/return_2.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    println!("{:?}", tokens);
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    println!("{:?}", abstract_syntax_tree);
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    println!("{}", finished_asm);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Keyword(Keyword),
    Identifier(String),
    IntegerLiteral(String),
    BinaryOperation(BinaryOperation),
    Semicolon,
    Negation,
    BitwiseComplement,
    LogicalNegation
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Int,
    Return,
    If
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
    Equal,
    Add,
    Subtract,
}

#[derive(Debug, PartialEq, Clone, Copy)]
// NOTE THIS IS MEANT FOR UNARY OPERATIONS

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOperation {
    GreaterThan,
    LessThan
}
