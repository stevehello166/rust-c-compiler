use crate::*;
use regex::Regex;

macro_rules! push_token {
    ($push_token:expr) => {
        tokens.push($push_token)
    };
}

pub(crate) fn lexer(file: String) -> Vec<Token>{
    let regex = Regex::new(r"(?m)([\{]+)|([\}]+)|([\(]+)|([\)]+)|(int)|([a-zA-Z]+)\w*|([0-9])+|(=)|(<)|(>)|(;)|(-)|(~)|(!)").unwrap();
    let mut unparsed_tokens = vec![];
    for x in regex.captures_iter(&file) {
        unparsed_tokens.push(x.get(0).unwrap().as_str());
    }
    let mut tokens: Vec<Token> = vec![];
    for token in unparsed_tokens {
        match token {
            "{" => {tokens.push(Token::OpenBrace)},
            "}" => {tokens.push(Token::CloseBrace)},
            "(" => {tokens.push(Token::OpenParenthesis)},
            ")" => {tokens.push(Token::CloseParenthesis)},
            "int" => {tokens.push(Token::Keyword(Keyword::Int))},
            "return" => {tokens.push(Token::Keyword(Keyword::Return))},
            "if" => {tokens.push(Token::Keyword(Keyword::If))}
            ">" => {tokens.push(Token::BinaryOperation(BinaryOperation::GreaterThan))},
            "<" => {tokens.push(Token::BinaryOperation(BinaryOperation::LessThan))},
            ";" => {tokens.push(Token::Semicolon)},
            "-" => {tokens.push(Token::Negation)},
            "~" => {tokens.push(Token::BitwiseComplement)},
            "!" => {tokens.push(Token::LogicalNegation)}
            _ => {
                if token.parse::<f64>().is_ok() {
                    tokens.push(Token::IntegerLiteral(token.to_string()))
                } else {
                    tokens.push(Token::Identifier(token.to_string()))
                }
            }
        }
    }
    return tokens;
}

