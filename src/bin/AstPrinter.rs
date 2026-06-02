use lox_interpreter::lox::Expr;
use lox_interpreter::lox::Object;
use lox_interpreter::lox::Token;
use lox_interpreter::lox::TokenType;


fn main() {
    let l1 = Expr::Literal{value: Object::NUMBER(123.0)};
    let l2 = Expr::Literal{value: Object::NUMBER(41.23)};
    let op = Token {token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1   };
    let unary = Expr::Unary{operator: op, right: Box::new(l1)};
    let group = Expr::Grouping {expression: Box::new(l2)};
    let op2 = Token {token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1   };
    let binary = Expr::Binary {left: Box::new(unary), operator:op2, right: Box::new(group)};

    let _ = print_ast(&binary);
}

fn print_ast(expr: &Expr) {
    let ast_as_string : String = evaluate_ast(expr);
    println!("{}", ast_as_string); 
}

fn evaluate_ast (expr: &Expr) -> String {
    match expr {
        Expr::Binary {left, operator, right}=> {
            let left_val = evaluate_ast(left);
            let right_val = evaluate_ast(right);
            format!("({} {} {})", operator.lexeme, left_val, right_val)
        }
        Expr::Unary{operator, right}=> {
            let right_val = evaluate_ast(right);
            format!("({} {})", operator.lexeme, right_val)
        }
        Expr::Grouping{expression}=> {
            let group_expr = evaluate_ast(expression);
            format!("(group {} )", group_expr)
        }
        Expr::Literal{value} => {
            format!("{}", value.object_to_string())
        }
        Expr::Assign{name, value} => {
            let right_val = evaluate_ast(value);
            format!("({} {})", name.lexeme, right_val)
        }
        Expr::Variable{name} => {
            format!("({})", name.lexeme)
        }
        Expr::Logical {left, operator, right}=> {
            let left_val = evaluate_ast(left);
            let right_val = evaluate_ast(right);
            format!("(if {} then {} else {})", operator.lexeme, left_val, right_val)
        }
    }
}
