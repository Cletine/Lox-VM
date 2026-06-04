use crate::lox::Expr;


pub fn print_ast(expr: &Expr) {
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
        Expr::Variable{name}=> {
            format!("(var {};\n)", name.lexeme)
        }
        Expr::Assign{name, value}=> {
            let rhs = evaluate_ast(value);
            format!("(var {} = {};\n)", name.lexeme, rhs)
        }
        Expr::Logical {left, operator, right}=> {
            let left_val = evaluate_ast(left);
            let right_val = evaluate_ast(right);
            format!("(if {} then {} else{})", operator.lexeme, left_val, right_val)
        }
        Expr::Call {callee,arguments, ..} => {
            let callee_val = evaluate_ast(callee);
            let args_val: String = arguments
                .iter()
                .map(|args| evaluate_ast(args))
                .collect::<Vec<String>>() 
                .join(",");               
            format!("{}({})", callee_val, args_val)
        }
    }
}
