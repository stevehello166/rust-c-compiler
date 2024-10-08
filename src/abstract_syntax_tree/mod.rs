use std::default;

use crate::{Keyword, Token};

pub fn create_abstract_syntax_tree(mut tokens: Vec<Token>) -> Result<Program, ()>{
    // extract list of tokens and place in an AST
    let mut program = Program(Vec::new());
    let mut token_counter = 0;
    let mut bracket_counter = 0;
    let mut function_counter = 0;
    let mut is_if = false;
    for token in tokens.clone() {
        let mut function_int = false;
        if token == Token::Keyword(Keyword::Int) && tokens[token_counter+2] == Token::OpenParenthesis && tokens[token_counter+3] == Token::CloseParenthesis{
            function_counter + 1;
            let tokenp1 = &tokens[token_counter+1];
            let mut name = "".to_string();
            match tokenp1 {
                Token::Identifier(ident) => name = ident.to_string(),
                _ => todo!()
            }
            program.0.push(Function {name: name, body: Body(Vec::new())});
            function_int = true
        }
        if token == Token::OpenBrace {
            bracket_counter + 1;
        }
        if token == Token::CloseBrace {
            bracket_counter - 1;
        }

        match token {
            Token::Keyword(keyword) => {
                match keyword {
                    Keyword::Return => {
                        let mut to_push:Option<u64> = None;
                        for next_token in &mut tokens[token_counter+1..] {
                            println!("{:?}", next_token);
                            match next_token {
                                Token::IntegerLiteral(x) => to_push = Some(x.parse::<u64>().expect("msg")),
                                Token::BitwiseComplement() => {},
                                Token::Semicolon => {
                                    let to_push = match to_push {
                                        Some(int) => int,
                                        None => return Err(())
                                    };
                                    program.0[0].body.0.push(Statement { 
                                        statement_type: StatementType::Return(Expression::Constant(
                                            to_push
                                        ))
                                    });
                                    break
                                },
                                _=> todo!()
                            }
                        }
                    },
                    Keyword::Int => {
                        if !function_int {
                            todo!()
                        }
                    }
                    _=> todo!()
                }
            }
            _=> {}
        }

        token_counter += 1;
    }
    if bracket_counter != 0 {
        println!("Not enough brackets");
        return Err(())
    }
    return Ok(program);
}

#[derive(Debug, Default)]
pub struct Program(pub Vec<Function>);

#[derive(Debug)]
pub struct Function{
    pub name: String,
    pub body: Body,
}

#[derive(Debug)]
pub struct Body (
    pub Vec<Statement>
);

#[derive(Debug)]
pub struct Statement{
    pub statement_type: StatementType
}

#[derive(Debug)]
pub struct BinaryOp{
    pub op: BinaryOpType,
    pub operand1: Variable,
    pub operand2: Variable
}

#[derive(Debug, Default)]
pub enum BinaryOpType {
    GreaterThan,
    LessThan,
    #[default]
    EqualTo
}

#[derive(Debug)]
pub enum StatementType {
    If{condition: BinaryOp, if_body: Body, else_body: Body},
    Return(Expression),
    UnaryOperation(Expression)
}
#[derive(Debug)]
pub struct Variable{
    name: String,
}

#[derive(Debug)]
//switch to an enum
pub enum Expression {
    Constant(u64),
    UnaryOperation{unary_op: UnaryOperation, constant: Expression}
}

#[derive(Debug)]
pub enum UnaryOperation {
    Negation,
    BitwiseComplement,
    LogicalNegation
}