use super::super::*;

#[test]
fn nil_primary() {
    let test_tokens = vec![Token { token_type: TokenType::NIL, lexeme: "nill".to_string(), literal: Object::NULL, line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Literal {value: Object::NULL, }));
}


#[test]
fn number_primary() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Literal {value: Object::NUMBER(1.0), }));
}


#[test]
fn string_primary() {
    let test_tokens = vec![Token { token_type: TokenType::STRING, lexeme: "String".to_string(), literal: Object::STRING("String".to_string()), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Literal {value: Object::STRING("String".to_string()), }));
}


#[test]
fn true_primary() {
    let test_tokens = vec![Token { token_type: TokenType::TRUE, lexeme: "true".to_string(), literal: Object::NULL, line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Literal {value: Object::BOOL(true), }));
}


#[test]
fn false_primary() {
    let test_tokens = vec![Token { token_type: TokenType::FALSE, lexeme: "false".to_string(), literal: Object::NULL, line: 1 }, Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}]; let mut parser = LoxParser{ tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Literal {value: Object::BOOL(false), }));
}


#[test]
fn expression_primary() {
    let test_tokens = vec![Token { token_type: TokenType::LeftParen, lexeme: "(".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::RightParen, lexeme: ")".to_string(), literal: Object::NULL, line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Grouping {expression: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }), }));
}


#[test]
fn neg_unary () {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                        right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}));

}


#[test]
fn bang_unary () {
    let test_tokens = vec![Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Object::NULL, line: 1 }, 
                        Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                        Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Unary{operator: Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}));
}


#[test]
fn div_factor() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Slash, lexeme: "/".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Slash, lexeme: "/".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));

}


#[test]
fn mul_factor() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}


#[test]
fn add_terms() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}


#[test]
fn sub_terms() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}


#[test]
fn factored_term() {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                              right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}));
}


#[test]
fn unary_after_term() {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                              right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                              right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),}));
}


#[test]
fn less_than_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}


#[test]
fn less_than_eq_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}


#[test]
fn greater_than_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}


#[test]
fn greater_than_eq_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}

#[test]
fn equal_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}



#[test]
fn not_equal_comp() {
    let test_tokens = vec![Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Binary{
                                            left:  Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),
                                            operator: Token { token_type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: Object::NULL, line: 1 },
                                            right:  Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),);
}


#[test]
fn var_assign() {
    let test_tokens = vec![Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::STRING, lexeme: "espresso".to_string(), literal: Object::STRING("espresso".to_string()), line: 1},
                           Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse_expr(), Ok(Expr::Assign {
                                            name: Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 },
                                            value: Box::new(Expr::Literal {value: Object::STRING("espresso".to_string()), }),}));
}



#[test]
fn var_declaration() {
    let test_tokens = vec![Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::STRING, lexeme: "espresso".to_string(), literal: Object::STRING("espresso".to_string()), line: 1},
                           Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse()[0], Statement::Var {
                                            name: Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 },
                                            initializer: Box::new(Expr::Literal {value: Object::STRING("espresso".to_string()), }),});
}


#[test]
fn null_var_declaration() {
    let test_tokens = vec![Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse()[0], Statement::Var {
                                            name: Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 },
                                            initializer: Box::new(Expr::Literal {value: Object::NULL, }),});
}


#[test]
fn expression_statements() {
    let test_tokens = vec![Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }, 
                           Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 },
                           Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }, 
                           Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1},
                           Token {token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    assert_eq!(parser.parse()[0], Statement::ExprStatement {
                                    expression: Box::new(Expr::Binary{
                                        left: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                            right: Box::new(Expr::Literal {value: Object::NUMBER(1.0), }),}),
                                            operator: Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 },
                                            right: Box::new(Expr::Unary{operator: Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 }, 
                                                right: Box::new(Expr::Literal {value: Object::NUMBER(2.0), }),}),}),});
}

#[test]
fn blocking_statements() {
    let test_tokens = vec![Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::STRING, lexeme: "espresso".to_string(), literal: Object::STRING("espresso".to_string()), line: 1},
                           Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }, 
                           Token { token_type: TokenType::STRING, lexeme: "lemonade".to_string(), literal: Object::STRING("espresso".to_string()), line: 1},
                           Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), literal: Object::NULL, line: 1},
                           Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1}];
    let mut parser = LoxParser{
        tokens: test_tokens, current_index: 0 
    };

    let statements = parser.parse();
    assert_eq!(statements[0], Statement::Var {
                                            name: Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 },
                                            initializer: Box::new(Expr::Literal {value: Object::STRING("espresso".to_string()), }),});

    assert_eq!(statements[1], Statement::Block{
                                    statements: vec![Box::new(Statement::Var {
                                                            name: Token { token_type: TokenType::IDENTIFIER, lexeme: "beverage".to_string(), literal: Object::NULL, line: 1 },
                                                            initializer: Box::new(Expr::Literal {value: Object::STRING("espresso".to_string()), }),})],});
}




