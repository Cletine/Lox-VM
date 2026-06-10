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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
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

pub trait ExprVisitor<'a, R> {
    fn visit_assign_expr(&mut self, name: &'a Token, value: &'a Expr) -> R;
    fn visit_binary_expr(&mut self, left: &'a Expr, operator: &'a Token, right: &'a Expr) -> R;
    fn visit_call_expr(&mut self, callee: &'a Expr, paren: &'a Token, arguments: &'a Vec<Box<Expr>>) -> R;
    fn visit_grouping_expr(&mut self, expression: &'a Expr) -> R;
    fn visit_literal_expr(&mut self, value: &'a Object) -> R;
    fn visit_logical_expr(&mut self, left: &'a Expr, operator: &'a Token, right: &'a Expr) -> R;
    fn visit_unary_expr(&mut self, operator: &'a Token, right: &'a Expr) -> R;
    fn visit_variable_expr(&mut self, name: &'a Token) -> R;
}


impl Expr {
    pub fn accept<'a, R>(&'a self, visitor: &'a mut impl ExprVisitor<'a, R>) -> R {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary { left, operator, right } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call { callee, paren, arguments } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Logical { left, operator, right } => visitor.visit_logical_expr(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
        }
    }
}


#[derive (Debug, PartialEq, Clone)]
pub enum Statement{
    Block {
        statements : Vec<Box<Statement>>,
    },
    ExprStatement {
        expression: Box<Expr>,
    },
    Function {
        name: Token,
        arguments: Vec<Box<Expr>>,
        body: Vec<Box<Statement>>,
    },
    IfStatement{
        condition : Box<Expr>,
        thenBranch : Box<Statement>,
        elseBranch : Box<Option<Statement>>,
    },
    Return{
        keyword : Token,
        value : Box<Expr>,
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

pub trait StmtVisitor<'a, R> {
    fn visit_block_stmt(&mut self, statements: &'a Vec<Box<Statement>>) -> R;
    fn visit_expr_stmt(&mut self, expression: &'a Expr) -> R;
    fn visit_function_stmt(&mut self, name: &'a Token, arguments: &'a Vec<Box<Expr>>, body: &'a Vec<Box<Statement>>) -> R;
    fn visit_if_stmt(&mut self, condition: &'a Expr, then_branch: &'a Statement, else_branch: &'a Option<Statement>) -> R;
    fn visit_return_stmt(&mut self, keyword: &'a Token, value: &'a Expr) -> R;
    fn visit_while_stmt(&mut self, condition: &'a Expr, body: &'a Statement) -> R;
    fn visit_var_stmt(&mut self, name: &'a Token, initializer: &'a Expr) -> R;
}

impl Statement {
    pub fn accept<'a, R>(&'a self, visitor: &mut impl StmtVisitor<'a, R>) -> R {
        match self {
            Statement::Block { statements } => visitor.visit_block_stmt(statements),
            Statement::ExprStatement { expression } => visitor.visit_expr_stmt(expression),
            Statement::Function { name, arguments, body } => visitor.visit_function_stmt(name, arguments, body),
            Statement::IfStatement { condition, thenBranch, elseBranch } => {
                visitor.visit_if_stmt(condition, thenBranch, elseBranch.as_ref())
            }
            Statement::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
            Statement::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Statement::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
        }
    }
}



