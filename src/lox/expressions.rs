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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
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

    IfStatement{
        condition : Box<Expr>,
        thenBranch : Box<Statement>,
        elseBranch : Box<Option<Statement>>,
    },

    While {
        condition : Box<Expr>,
        body : Box<Statement>,
    },

    Var {
        name: Token,
        initializer: Box<Expr>,
    },

}


