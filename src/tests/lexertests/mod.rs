use crate::lexer::lexer;
use crate::Token::*;
use crate::Keyword::*;
use crate::*;
use std::fs::read_to_string;

#[test]
fn test_lexer () {
    let source = "src/tests/lexertests/return2.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
    assert_eq!(tokens, vec![Keyword(Int), Identifier("main".to_string()), OpenParenthesis, CloseParenthesis, OpenBrace, Keyword(Return), IntegerLiteral("2".to_string()), Semicolon, CloseBrace])
}