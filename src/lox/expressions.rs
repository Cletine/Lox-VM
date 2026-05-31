use crate::lox::Token;
use crate::lox::Object;


#[derive (Debug, PartialEq, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

#[derive (Debug, PartialEq, Clone)]
pub enum Statement{
    Block {
        statements : Vec<Box<Statement>>,
    },
    ExprStatement {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Expr>,
    },
}


