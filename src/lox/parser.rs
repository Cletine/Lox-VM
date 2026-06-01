use crate::lox::Token;
use crate::lox::TokenType;
use crate::lox::Object;
use crate::lox::Expr;
use crate::lox::Statement;
use crate::ParserError;



// The following parser represents a left-sided recursive descent parser
// The parser moves through the list of tokens and generates an AST 
// If any error occurs, the parser should propagate the error back up to the parse function


pub struct LoxParser {
    pub tokens: Vec<Token>,
    pub current_index: usize,
}


impl LoxParser {

    pub fn parse_expr (&mut self) -> Result<Expr, ParserError>{
        let (expr, _index) = Self::expression(&self.tokens, self.current_index);
        return expr
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_index < self.tokens.len() {
            let nx_statement = self.declaration();
            match nx_statement {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(e) => {
                    e.parse_error();
                    self.current_index = Self::syncronize(&self.tokens, self.current_index);

                }
            };
        }
        statements
    }

    fn declaration<'a> (&mut self) -> Result<Statement, ParserError> {
        match self.tokens[self.current_index].token_type {
            TokenType::VAR =>  {
                self.current_index +=1;
                return self.var_declaration()
            }
            _ => return self.statement(),
        }
    }

    
    fn var_declaration<'a> (&mut self) -> Result<Statement, ParserError> {
        let mut name: Option<_> = None;
        let mut error: Option<_> = None;

        match self.tokens[self.current_index].token_type {
            TokenType::IDENTIFIER => {
                name = Some(self.tokens[self.current_index].clone()); 
            }
            _ => return Err(ParserError {error_msg: "Expect variable name.".to_string(), error_token: self.tokens[self.current_index].clone()})

        }

        self.current_index += 1;
        let mut initializer : Expr = Expr::Literal { value : Object::NULL };

        match self.tokens[self.current_index].token_type {
            TokenType::Equal => { 
                self.current_index += 1;
                let (value, nx_index) = Self::expression(&self.tokens, self.current_index);
                self.current_index = nx_index;
                match value {
                    Ok(expr) => {
                        initializer = expr;
                    }
                    Err(e) => {
                        error = Some(e);
                    } 
                };
            }
            _ => (),
        }


        // checks if any errors were propagaeted during expression parse
        if error != None {
            return Err(error.expect("Missing error propagated from expression in statement"))
        }

        match self.tokens[self.current_index].token_type {
            TokenType::SemiColon =>  { 
                self.current_index += 1;
                Ok(Statement::Var{name: name.expect("Missing variable name identifier in statement"), initializer: Box::new(initializer)})
            }
            _ => Err(ParserError {error_msg: "Expect SemiColon After Expression.".to_string(), error_token: self.tokens[self.current_index].clone()}),
        }
    }

    fn statement<'a> (&mut self) ->  Result<Statement, ParserError> {
        // Determines if the statement/s are blocked in a higher order block scope
        if self.tokens[self.current_index].token_type == TokenType::LeftBrace {
            //move off/consume  left brace
                self.current_index += 1;
               return self.block()
        }

        let (value, nx_index) = Self::expression(&self.tokens, self.current_index);
        self.current_index = nx_index;
        match self.tokens[self.current_index].token_type {
            TokenType::SemiColon => { 
                match value {
                   Ok(val) => {
                       self.current_index += 1;
                       return Ok(Statement::ExprStatement{expression: Box::new(val)}) 
                   }
                   Err(e) => return Err(e),
                }
           }
            _ => return Err(ParserError {error_msg: "Expect SemiColon After Expression.".to_string(), error_token: self.tokens[self.current_index].clone()}), 
        }
    }


    fn block<'a> (&mut self) -> Result<Statement, ParserError> {
        let mut block_statements = Vec::new();
        // loop through till a right brace/ End of Block
        while self.current_index < self.tokens.len() {
            match self.tokens[self.current_index].token_type {
                TokenType::RightBrace => break, 
                _ => {
                    let nx_statement = self.declaration();
                    match nx_statement {
                        Ok(stmt) => {
                            block_statements.push(Box::new(stmt));
                        }
                        Err(e) => {
                            e.parse_error();
                            self.current_index = Self::syncronize(&self.tokens, self.current_index);
                        }
                    }
                }
            };
        }

        match self.tokens[self.current_index].token_type {
            TokenType::RightBrace=>  { 
                    self.current_index += 1;
                    return Ok(Statement::Block {statements: block_statements})
                 }
            _ => return Err(ParserError {error_msg: "Expect '}' After Expression.".to_string(), error_token: self.tokens[self.current_index].clone()}),
        }
    }




    // This function should parse through the entire expression and 
    // branch according to whether it is a sound expression or not 
    fn expression <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        Self::assignment(tokens, current)
    }

    fn assignment <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let (left_expr, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::equality(tokens, current);

        match tokens[nx_index].token_type {
            TokenType::Equal=> {
                let (result, nx_index) = Self::assignment(tokens, nx_index + 1);
                match result {
                    Ok(right_expr) => {
                        match left_expr {
                            Ok(Expr::Variable {name: n,}) => (Ok(Expr::Assign {name: n, value: Box::new(right_expr)}), nx_index),
                            _ => {
                                    ParserError {error_msg: "Invalid assignment type".to_string(), error_token: tokens[nx_index - 1].clone() }.parse_error();
                                    return (left_expr, nx_index)
                            }
                        }
                    }
                    Err(e) => {
                        return (Err(e), nx_index)
                    }
                }
            }
            _ => {
                return (left_expr, nx_index)
            }
        }
    }

    fn equality <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let mut parse_error_status: Option<_> = None;
        // Determine left sided soundness of the left side of the exression
        let (left_expr, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::comparision(tokens, current);

        match left_expr {
            Ok(mut left_expr) => {
                while nx_index < tokens.len(){
                    match tokens[nx_index].token_type  {
                        TokenType::BangEqual | TokenType::EqualEqual => {
                            let operator = tokens[nx_index].clone();
                            nx_index += 1;
                            // Determine right sided soundness of the right side of the expression
                            match Self::comparision(tokens, nx_index) {
                                // If both left and right are sound, update the index and add to the
                                // expression stack
                                (Ok(right_expr), nx) => {
                                    nx_index = nx;
                                    left_expr = Expr::Binary{
                                        left: Box::new(left_expr),
                                        operator: operator, 
                                        right: Box::new(right_expr),
                                    };
                                }

                                // else return right sided parsing error
                                (Err(parse_error), nx) => { 
                                    nx_index = nx;
                                    parse_error_status = Some(parse_error);
                                    break
                                },
                            };
                        }
                        _ => break,
                    }
                }
                //Determines if any errors propagated back.
                match parse_error_status {
                    Some (parse_error) => (Err(parse_error), nx_index) ,
                    None => (Ok(left_expr), nx_index),
                }
            }
            // else return left sided parsing error 
            Err(parse_error) => (Err(parse_error), nx_index),
        }

    }


    fn comparision <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {      
        let mut parse_error_status: Option<_> = None;
        // Determine left sided soundness of the left side of the exression
        let (left_term, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::term(tokens, current);

        match left_term {
            Ok(mut left_term) => {
                // Iterate through contiguous instances of the Toketype
                while nx_index < tokens.len() {
                    match tokens[nx_index].token_type  {
                        TokenType::Greater | TokenType::Less | TokenType::GreaterEqual | TokenType::LessEqual => {
                            let operator = tokens[nx_index].clone();
                            nx_index += 1;
                            // Determine right sided soundness of the right side of the expression
                            match Self::term(tokens, nx_index) {
                                // If both left and right are sound, return the expression
                                (Ok(right_term), nx) => {
                                    nx_index = nx;
                                    left_term = Expr::Binary{
                                        left: Box::new(left_term),
                                        operator: operator, 
                                        right: Box::new(right_term),
                                    }
                                }
                                // else return right sided parsing error
                                (Err(parse_error), nx) =>  { 
                                    nx_index = nx;
                                    parse_error_status = Some(parse_error);
                                    break
                                }
                            }
                        }
                        _ => break,
                    }
                }
                //Determines if right descent propagated any errors back.
                match parse_error_status {
                    Some(parse_error) => (Err(parse_error), nx_index),
                    None => (Ok(left_term), nx_index),
                }
            }
            // return left sided parsing propagated error 
            Err(parse_error) => (Err(parse_error), nx_index)
        }
    }



    fn term <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let mut parse_error_status: Option<_> = None;
        // Determine left sided soundness of the left side of the exression
        let (left_factor, mut nx_index) : (Result<Expr, ParserError>, usize)  = Self::factor(tokens, current);

        match left_factor {
            Ok(mut left_factor) => {
                // Iterate through contiguous instances of the Toketype
                while nx_index < tokens.len() {
                    match tokens[nx_index].token_type  {
                        TokenType::Minus | TokenType::Plus => {
                            let operator = tokens[nx_index].clone();
                            nx_index += 1;

                            match Self::factor(tokens, nx_index) {
                                (Ok(right_factor), nx) => {
                                    nx_index = nx;
                                    left_factor = Expr::Binary{
                                        left: Box::new(left_factor),
                                        operator: operator, 
                                        right: Box::new(right_factor),
                                    };
                                }
                                // return right sided parsing error
                                (Err(parse_error), nx) => { 
                                    nx_index = nx;
                                    parse_error_status = Some(parse_error);
                                    break
                                }
                            }
                        }
                        _ => break,
                    }
                }
                //Determines if right descent propagated any errors back.
                match parse_error_status {
                    Some(parse_error) => (Err(parse_error), nx_index),
                    None => (Ok(left_factor), nx_index),
                }            
            }
            // return left sided parsing error 
            Err(parse_error) => (Err(parse_error), nx_index)
        }
    }


    fn factor<'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let mut parse_error_status: Option<_> = None;
        // Determine left sided soundness of the left side of the exression
        let (left_unary, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::unary(tokens, current);
        match left_unary {
            Ok(mut left_unary) => {
                // Iterate through contiguous instances of the Toketype
                while nx_index < tokens.len(){
                    match tokens[nx_index].token_type  {
                        TokenType::Slash | TokenType::Star => {
                            let operator = tokens[nx_index].clone();
                            nx_index += 1;

                            // Determine right sided soundness of the right side of the expression
                            match Self::unary(tokens, nx_index) {
                                // If both left and right are sound, return the expression
                                (Ok(right_unary), nx) => {
                                    nx_index = nx;
                                    left_unary= Expr::Binary{
                                        left: Box::new(left_unary),
                                        operator: operator, 
                                        right: Box::new(right_unary),
                                    };
                                }
                                // else return right sided parsing error
                                (Err(parse_error), nx) => { 
                                    nx_index = nx;
                                    parse_error_status = Some(parse_error);
                                    break
                                }
                            }
                        }
                        _ => break,
                    }
                }
                //Determines if right descent propagated any errors back.
                match parse_error_status {
                    Some(parse_error) => (Err(parse_error), nx_index),
                    None => (Ok(left_unary), nx_index),
                }           
            }
            // return left sided parsing error 
            Err(parse_error) => (Err(parse_error), nx_index)
        }
    }


    fn unary <'a> (tokens: &'a Vec<Token>, mut current: usize) -> (Result<Expr, ParserError>, usize) {
        let unary: (Result<Expr, ParserError>, usize) = 
            match tokens[current].token_type  {
                TokenType::Bang | TokenType::Minus => {
                    let operator = tokens[current].clone();
                    current += 1;

                    match Self::unary(tokens, current) {
                        // If the expression is sound, return the expression
                        (Ok(unary), cur_index)  => {
                            (Ok(Expr::Unary {
                                operator: operator, 
                                right: Box::new(unary),
                            }), cur_index)    
                        }
                        // else return right sided parsing error
                        (Err(parse_error), cur_index)  => (Err(parse_error), cur_index)
                    }
                }
                _ => Self::primary(tokens, current),
            };

        return unary
    }


    fn primary<'a> (tokens: &'a Vec<Token>, mut current: usize) -> (Result<Expr, ParserError>, usize) {
        let primary: (Result<Expr, ParserError>, usize) = 
            match tokens[current].token_type {
                TokenType::NUMBER => {
                    (Ok(Expr::Literal {
                        value: tokens[current].literal.clone(),
                    }), current + 1)
                }
                TokenType::STRING => {
                    (Ok(Expr::Literal {
                        value: tokens[current].literal.clone(),
                    }), current + 1)

                }
                TokenType::FALSE => {
                    (Ok(Expr::Literal {
                        value: Object::BOOL(false),
                    }), current + 1)

                }
                TokenType::TRUE => {
                    (Ok(Expr::Literal {
                        value: Object::BOOL(true),
                    }), current + 1)

                }
                TokenType::NIL => {
                    (Ok(Expr::Literal {
                        value: Object::NULL
                    }), current + 1 )

                }
                TokenType::IDENTIFIER => {
                    (Ok(Expr::Variable {
                        name: tokens[current].clone(),
                    }), current + 1)
                }
                TokenType::LeftParen => {
                    current += 1;

                    let (expr, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::expression(tokens, current);

                    match expr {
                        Ok(expr) => {
                            match tokens[nx_index].token_type {
                                TokenType::RightParen => {
                                    (Ok(Expr::Grouping {
                                        expression: Box::new(expr),
                                    }), nx_index + 1)
                                }
                                _ => {
                                    //parse_error(self.tokens[self.current], "Expect ')' after expression.");
                                    //TODO some syncronizing effort or panic to evaluate the entire ast 
                                    (Err(ParserError {error_msg: "Expect ')' after expression.".to_string(), error_token: tokens[nx_index].clone() }), nx_index)
                                }
                            }
                        }
                        Err(parse_error) => (Err(parse_error), nx_index),
                    }
                }
                _ => {  
                    (Err(ParserError {error_msg: "Expect expression".to_string(), error_token: tokens[current].clone()}), current)
                }
            };

        return primary
    }


    fn syncronize<'a> (tokens: &'a Vec<Token>, mut current: usize) -> usize {
        // move off the current error throwing token
        current += 1;

        while current < tokens.len() {
            // checks if previous token was a statement terminator ';'
            if tokens[current].token_type == TokenType::SemiColon {
                break
            }
            if tokens[current].token_type == TokenType::RightBrace {
                break
            }
            match tokens[current].token_type {
                TokenType::CLASS |
                    TokenType::FUN |
                    TokenType::FOR |
                    TokenType::IF |
                    TokenType::WHILE |
                    TokenType::PRINT |
                    TokenType::RETURN |
                    TokenType::VAR => break,
                _ => current += 1,
            }
        }

        current
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.tokens.len() 
    }
}
