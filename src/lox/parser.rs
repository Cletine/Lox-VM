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
        let name: Token;
        let mut error: Option<_> = None;

        match self.tokens[self.current_index].token_type {
            TokenType::IDENTIFIER => {
                name = self.tokens[self.current_index].clone(); 
            }
            _ => return Err(ParserError {error_msg: "Var Decl ERROR: Expect variable name.".to_string(), error_token: self.tokens[self.current_index].clone()})

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
                Ok(Statement::Var{name: name, initializer: Box::new(initializer)})
            }
            _ => Err(ParserError {error_msg: "Var Decl ERROR: Expect SemiColon After Expression.".to_string(), error_token: self.tokens[self.current_index].clone()}),
        }
    }



    fn statement<'a> (&mut self) ->  Result<Statement, ParserError> {
        match self.tokens[self.current_index].token_type {
            // Blocking Statement/s
            TokenType::LeftBrace  =>{
                //move off/consume  left brace
                self.current_index += 1;
                return self.block()
            }
            // If Statements
            TokenType::IF =>{
                self.current_index += 1;
                return self.if_statement()
            }
            // While Statements
            TokenType::WHILE => {
                self.current_index += 1;
                return self.while_statement()
            }
            // For Statements
            TokenType::FOR => {
                self.current_index += 1;
                return self.for_statement()
            }
            TokenType::FUN => {
                self.current_index += 1;
                return self.function("function".to_string())
            }
            TokenType::RETURN => {
                return self.return_statement()
            }

            // Expr Statement
            _ => return self.expr_statement()
        }
    }


    fn expr_statement<'a> (&mut self) ->  Result<Statement, ParserError> {
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
            _ => return Err(ParserError {error_msg: "Expr ERROR: Expect SemiColon After Expression.".to_string(), error_token: self.tokens[self.current_index].clone()}), 
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
            _ => return Err(ParserError {error_msg: "Block ERROR: Expect '}' After Blocking Statement.".to_string(), error_token: self.tokens[self.current_index].clone()}),
        }
    }



    fn if_statement<'a> (&mut self) -> Result<Statement, ParserError> {
        // if
        match self.tokens[self.current_index].token_type {
            // (condition)
            TokenType::LeftParen => {
                self.current_index += 1;
                let (condition, nx_index) = Self::expression(&self.tokens, self.current_index);
                self.current_index = nx_index;
                match condition {
                    Ok(cond) => {
                        match self.tokens[self.current_index].token_type {
                            TokenType::RightParen => {
                                self.current_index += 1;
                                let thenBranch = self.statement();
                                // then [statement]
                                match thenBranch {
                                    Ok(thenBr) => {
                                        let mut elseBranch = None;
                                        //else [statement]
                                        if self.tokens[self.current_index].token_type == TokenType::ELSE {
                                            self.current_index += 1;
                                            elseBranch = Some(self.statement());
                                            
                                        }
                                        match elseBranch {
                                            // if (condition) then [statement] else [statement]
                                            Some(Ok(elseBr)) => return Ok(Statement::IfStatement {condition: Box::new(cond), thenBranch: Box::new(thenBr), elseBranch: Box::new(Some(elseBr))}),
                                            // if (condition) then [statement] 
                                            None => return Ok(Statement::IfStatement {condition: Box::new(cond), thenBranch: Box::new(thenBr), elseBranch: Box::new(None)}),
                                            // Error Propagated from elseBranch
                                            Some(Err(e)) => return Err(e)
                                        }
                                    }

                                    Err(e) => return Err(e)
                                }
                            }
                            _ => return Err(ParserError {error_msg: "Conditional Statement ERROR: Expect ')' after 'if'".to_string(), error_token: self.tokens[self.current_index].clone()}),
                        }
                    }
                    Err(e) => return Err(e)
                }
            },

            _ => return Err(ParserError {error_msg: "Conditional Statement ERROR: Expect '(' after 'if'".to_string(), error_token: self.tokens[self.current_index].clone()}),

        }
    }


    fn while_statement<'a> (&mut self) -> Result<Statement, ParserError> {
        // while
        match self.tokens[self.current_index].token_type {
            // (condition)
            TokenType::LeftParen => {
                self.current_index += 1;
                let (condition, nx_index) = Self::expression(&self.tokens, self.current_index);
                self.current_index = nx_index;
                match condition {
                    Ok(cond) => {
                        match self.tokens[self.current_index].token_type {
                            TokenType::RightParen => {
                                self.current_index += 1;
                                let body = self.statement();
                                // do [statement]
                                match body {
                                    Ok(b) => {
                                      return  Ok(Statement::While {condition: Box::new(cond), body:Box::new(b)})
                                    }
                                    Err(e) => return Err(e)
                                }
                            }
                            _ => return Err(ParserError {error_msg: "Looping Statement ERROR: Expect ')' after while condition".to_string(), error_token: self.tokens[self.current_index].clone()}),
                        }
                    }
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(ParserError {error_msg: "Looping Statement ERROR: Expect '(' after 'while'".to_string(), error_token: self.tokens[self.current_index].clone()}),
        }
    }


    fn for_statement<'a> (&mut self) -> Result<Statement, ParserError> {
        //for 
        match self.tokens[self.current_index].token_type {
            TokenType::LeftParen => {
                self.current_index += 1;
                // var_decl | expr ;
                // Will consume ';' when processing statement
                let initializer : Option<Statement> = 
                    match self.tokens[self.current_index].token_type {
                        TokenType::SemiColon => {
                            self.current_index += 1;
                            None
                        }
                        TokenType::VAR => {
                            self.current_index += 1;
                            let stmt = self.var_declaration()?;
                            Some(stmt)

                        }
                        _ => {
                            let stmt = self.expr_statement()?;
                            Some(stmt)
                        }
                    };

                                // (cond)expr ;
                let mut condition: Option<Expr> =
                    match self.tokens[self.current_index].token_type { 
                        TokenType::SemiColon => None, 
                        _ => { 
                            let (expr, nx_index) = Self::expression(&self.tokens, self.current_index); 
                            self.current_index = nx_index; 
                            Some(expr?) 
                        }
                    };

                match self.tokens[self.current_index].token_type {
                    TokenType::SemiColon =>  { 
                        self.current_index += 1;
                        ()
                    }
                    _ => return Err(ParserError {error_msg: "Looping Statement ERROR: Expect SemiColon After for Loop Condition.".to_string(), error_token: self.tokens[self.current_index].clone()})
                }
                


                let increment: Option<Expr> = 
                    match self.tokens[self.current_index].token_type { 
                        TokenType::RightParen => {
                            None
                        }
                        _ => { 
                            let (expr, nx_index) = Self::expression(&self.tokens, self.current_index); 
                            self.current_index = nx_index; 
                            Some(expr?) 
                        }

                    };

                //for (init; cond; incr)
                match self.tokens[self.current_index].token_type {
                    TokenType::RightParen => {
                        self.current_index += 1;
                        let mut body: Statement = self.statement()?;

                        match increment  {
                            None => (),
                            Some(incr) => body = Statement::Block {statements: vec![Box::new(body), Box::new(Statement::ExprStatement{expression: Box::new(incr)})]},
                        }

                        if condition == None {
                            condition = Some(Expr::Literal{value: Object::BOOL(true)});
                        }

                        body = Statement::While {condition:Box::new(condition.expect("Looping Body Expects Some(Condition)")), body:Box::new(body)};

                        match initializer {
                            None => (),
                            Some(init) => body = Statement::Block {statements:vec![Box::new(init), Box::new(body)]}
                        }

                        return Ok(body)
                    }
                    _ => return Err(ParserError {error_msg: "Looping Statement ERROR: Expect ')' after for clauses".to_string(), error_token: self.tokens[self.current_index].clone()}),

                }
            }
            _ => return Err(ParserError {error_msg: "Looping Statement ERROR: Expect '(' after 'for'".to_string(), error_token: self.tokens[self.current_index].clone()}),
        }
    }


    fn function<'a> (&mut self, kind: String) -> Result<Statement, ParserError> {
        let mut parse_error_status: Option<_> = None;

        match self.tokens[self.current_index].token_type {
            TokenType::IDENTIFIER => {
                let name = self.tokens[self.current_index].clone();
                let mut arguments = Vec::new();
                //move off function name
                self.current_index += 1;
                match self.tokens[self.current_index].token_type {
                    TokenType::LeftParen => {
                        //move off/consume left paren
                        self.current_index += 1;
                        //loop through function args
                        if self.tokens[self.current_index].token_type != TokenType::RightParen {
                            loop {

                                if arguments.len() > 255 {
                                    return Err(ParserError {error_msg: "Function Declaration ERROR: Can't have more than 255 arguments".to_string(), error_token: self.tokens[self.current_index + 1].clone()})
                                }

                                let (nx_expression, nx_index) = Self::expression(&self.tokens, self.current_index);
                                self.current_index = nx_index;

                                match nx_expression  {
                                    Ok(expr) => {
                                        arguments.push(Box::new(expr));
                                    }
                                    Err(parse_error) => {
                                        parse_error_status = Some(parse_error);
                                        break
                                    }
                                }
                                if self.tokens[self.current_index].token_type != TokenType::Comma {
                                    break;
                                }
                                // move off/comsume comma
                                else {
                                    self.current_index += 1;
                                }
                            }
                        }
                        // check for any parsing error before parsing the body
                        match parse_error_status {
                            None => {
                                match self.tokens[self.current_index].token_type {
                                    TokenType::RightParen =>  { 
                                        //move off/consume right paren
                                        self.current_index += 1;
                                        match self.tokens[self.current_index].token_type {
                                            // parse the function body
                                            TokenType::LeftBrace => {
                                                //move off/comsome left brace
                                                self.current_index += 1;
                                                let body = self.block()?;
                                                match body {
                                                    Statement::Block{statements} => return Ok(Statement::Function{name: name, arguments:arguments, body: statements}),
                                                    // return function declaration
                                                    _ => return Err(ParserError {error_msg:"Function Declaration ERROR: Parsing error occured when attempting to parse function body".to_string(), error_token: self.tokens[self.current_index].clone()}),

                                                }
                                            }
                                            _ => Err(ParserError {error_msg: format!("Function Declaration ERROR: Expect '{{' Before {} body.", kind), error_token: self.tokens[self.current_index].clone()}),
                                        }
                                    }
                                    _ => Err(ParserError {error_msg: "Function Declaration ERROR: Expect ')' After Call Arguments.".to_string(), error_token: self.tokens[self.current_index].clone()}),
                                }
                            }
                            Some(parse_error) => return Err(parse_error),
                                }
                    }

                    _ => Err(ParserError {error_msg: format!("Function Declaration ERROR: Expect '(' After {} Name.", kind) ,error_token: self.tokens[self.current_index].clone()}),

                }
            }
            _ => return Err(ParserError {error_msg: format!("Function Declaration ERROR: Expect '{}' name ", kind), error_token: self.tokens[self.current_index].clone()})

        }
    }


    fn return_statement<'a> (&mut self) -> Result<Statement, ParserError> {
        let keyword = self.tokens[self.current_index].clone();
        let mut value = Ok(Expr::Literal {value: Object::NULL});
        let nx_index: usize;
        //move off keyword
        self.current_index += 1;
        match self.tokens[self.current_index].token_type {
            TokenType::SemiColon => (),
            _ => {
                (value, nx_index) = Self::expression(&self.tokens, self.current_index);
                self.current_index = nx_index;
            }
        }

        match self.tokens[self.current_index].token_type {
            TokenType::SemiColon =>  { 
                self.current_index += 1;
                return Ok(Statement::Return {keyword: keyword, value: Box::new(value?)})
            }
            _ => return Err(ParserError {error_msg: "Return ERROR: Expect SemiColon After return statement.".to_string(), error_token: self.tokens[self.current_index].clone()})
        }
    }





    // This function should parse through the entire expression and 
    // branch according to whether it is a sound expression or not 
    fn expression <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        Self::assignment(tokens, current)
    }

    fn assignment <'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let (left_expr, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::or(tokens, current);

        match tokens[nx_index].token_type {
            TokenType::Equal=> {
                let (result, nx) = Self::assignment(tokens, nx_index + 1);
                nx_index = nx;
                match result {
                    Ok(right_expr) => {
                        match left_expr {
                            Ok(Expr::Variable {name: n,}) => (Ok(Expr::Assign {name: n, value: Box::new(right_expr)}), nx_index),
                            _ => {
                                    ParserError {error_msg: "Variable Assign ERROR: Invalid assignment type".to_string(), error_token: tokens[nx_index - 1].clone() }.parse_error();
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


    fn or<'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let mut parse_error_status: Option<_> = None;
        let (left_expr, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::and(tokens, current);
        match left_expr {
            Ok(mut left_expr) => {
                while nx_index < tokens.len(){
                    match tokens[nx_index].token_type  {
                        TokenType::OR =>
                        {
                            let operator = tokens[nx_index].clone();
                            nx_index += 1;
                            // Determine right sided soundness of the right side of the expression
                            match Self::and(tokens, nx_index) {
                                // If both left and right are sound, update the index and add to the
                                // expression stack
                                (Ok(right_expr), nx) => {
                                    nx_index = nx;
                                    left_expr = Expr::Logical{
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

    fn and<'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
        let mut parse_error_status: Option<_> = None;
        let (left_expr, mut nx_index) : (Result<Expr, ParserError>, usize) = Self::equality(tokens, current);
        match left_expr {
            Ok(mut left_expr) => {
                while nx_index < tokens.len(){
                    match tokens[nx_index].token_type  {
                        TokenType::AND =>
                        {
                            let operator = tokens[nx_index].clone();
                            nx_index += 1;
                            // Determine right sided soundness of the right side of the expression
                            match Self::equality(tokens, nx_index) {
                                // If both left and right are sound, update the index and add to the
                                // expression stack
                                (Ok(right_expr), nx) => {
                                    nx_index = nx;
                                    left_expr = Expr::Logical{
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
                _ => Self::call(tokens, current),
            };

        return unary
    }


    fn call<'a> (tokens: &'a Vec<Token>, current: usize) -> (Result<Expr, ParserError>, usize) {
            //  Parse the primary expression (the left-hand base callee)
        let (call_expr, mut nx_index) = Self::primary(tokens, current);

        let mut callee = match call_expr {
            Ok(expr) => expr,
            Err(parse_error) => return (Err(parse_error), nx_index),
        };

        //  Loop and accumulate nested function calls (e.g., foo()()())
        while nx_index < tokens.len() {
            match tokens[nx_index].token_type {
                TokenType::LeftParen => {
                    nx_index += 1;

                    // Safely pass ownership of the accumulated callee
                    let (next_callee, next_index) = Self::finish_call(tokens, nx_index, callee);
                    nx_index = next_index;

                    match next_callee {
                        Ok(expr) => callee = expr,
                        Err(parse_error) => return (Err(parse_error), nx_index),
                    }
                }
                _ => break,
            }
        }

        (Ok(callee), nx_index)
    }


    fn finish_call <'a> (tokens: &'a Vec<Token>, current: usize, callee: Expr) -> (Result<Expr, ParserError>, usize) {
        let mut parse_error_status: Option<_> = None;
        let mut arguments = Vec::new();
        let mut cur_index = current;


        if tokens[cur_index].token_type != TokenType::RightParen {
            loop {

                if arguments.len() > 255 {
                    return (Err(ParserError {error_msg: "Function Call ERROR: Can't have more than 255 arguments".to_string(), error_token: tokens[cur_index + 1].clone()}), cur_index)
                }

                let (nx_expression, nx_index) = Self::expression(tokens, cur_index);
                cur_index = nx_index;

                match nx_expression  {
                    Ok(expr) => {
                        arguments.push(Box::new(expr));
                    }
                    Err(parse_error) => {
                        parse_error_status = Some(parse_error);
                        break
                    }
                }

                if tokens[cur_index].token_type != TokenType::Comma {
                    break;
                }
                // else move off/cosume the comma
                else {
                    cur_index += 1;
                }
            }
        }

        match parse_error_status  {
            None => {
                match tokens[cur_index].token_type {
                    TokenType::RightParen =>  { 
                        let paren = tokens[cur_index].clone();
                        cur_index += 1;
                        (Ok(Expr::Call{callee: Box::new(callee), paren:paren, arguments: arguments}), cur_index)
                    }
                    _ => (Err(ParserError {error_msg: "Function Call ERROR: Expect ')' After Call Arguments.".to_string(), error_token: tokens[cur_index].clone()}), cur_index),
                }
            }
            Some(parse_error) => (Err(parse_error), cur_index)
        }
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

                    let (expr, nx_index) : (Result<Expr, ParserError>, usize) = Self::expression(tokens, current);

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
                                    (Err(ParserError {error_msg: "Expression Grouping ERROR: Expect ')' after expression.".to_string(), error_token: tokens[nx_index].clone() }), nx_index)
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
}
