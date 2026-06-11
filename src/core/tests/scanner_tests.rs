
use super::super::*;

    #[test]
    fn empty_scan() {
        let sample_source = String::from("");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
    }

    #[test]
    fn cons_scan() {
        let sample_source = String::from("+");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 })); 
    }

    #[test]
    fn single_char_lexeme() {
        // let sample_source = vec![Some(&')'), Some(&'('),Some(&'{'),Some(&'}'),Some(&','),Some(&'.'),Some(&'-'),Some(&'+'),Some(&';'),Some(&'*'),];
        let sample_source = String::from(")({},.-+;*"); 
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::SemiColon, lexeme: ";".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Dot, lexeme: ".".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Comma, lexeme: ",".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::LeftParen, lexeme: "(".to_string(), literal: Object::NULL, line: 1 })); 
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::RightParen, lexeme: ")".to_string(), literal: Object::NULL, line: 1 })); 
    }

    #[test]
    fn operator_lexeme() {
        //let sample_source = vec![Some(&'!'), Some(&' '),Some(&'!'),Some(&'='),Some(&' '),Some(&'='),Some(&' '),Some(&'='),Some(&'='),Some(&' '), Some(&'<'), Some(&' '),Some(&'<'),Some(&'='),Some(&' '),Some(&'>'),Some(&' '),Some(&'>'),Some(&'=')];
        let sample_source = String::from("! != = == < <= > >=");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::GreaterEqual, lexeme: ">=".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Greater, lexeme: ">".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::LessEqual, lexeme: "<=".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Less, lexeme: "<".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EqualEqual, lexeme: "==".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::BangEqual, lexeme: "!=".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Bang, lexeme: "!".to_string(), literal: Object::NULL, line: 1 }));  
    }

    #[test]
    fn comment_lexeme() {
        //let sample_source = vec![Some(&'/'),Some(&'/'),Some(&'1'),Some(&'+'),Some(&'1'),Some(&'\n'),Some(&'='),];
        let sample_source = String::from("//1+1\n=");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 2 }));   
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 2 }));  
    }

    #[test]
    fn empty_lexeme() {
        //let mut sample_source = vec![Some(&' '),Some(&'\t'),Some(&'\r'),Some(&'='),Some(&'\n')];
        let sample_source = String::from(" \t\r=\n");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 2 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::Equal, lexeme: "=".to_string(), literal: Object::NULL, line: 1 }));  
    }
    #[test]
        fn strings() {
            //let mut sample_source = vec![Some(&' '),Some(&'\t'),Some(&'\r'),Some(&'='),Some(&'\n')];
            let sample_source = String::from("\"Richard of York \n Gained Battle In Vain \"\n");
            let mut scanner = LoxScanner{
                source: sample_source, 
                tokens: Vec::new(),
                start: 0,
                current: 0,
                line: 1,
            };
            scanner.scan_tokens();
            assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 2 }));  
            assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::STRING, lexeme: "\"Richard of York \n Gained Battle In Vain \"".to_string(), literal: Object::STRING("Richard of York \n Gained Battle In Vain ".to_string()), line: 1 }));  
        }
        #[test]
    fn numbers() {
        //let mut sample_source = vec![Some(&' '),Some(&'\t'),Some(&'\r'),Some(&'='),Some(&'\n')];
        let sample_source = String::from("0 1 2 3 4 5 6 7 8 9 1.0 2.353612 2819238471321");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
 
        };


        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "2819238471321".to_string(), literal: Object::NUMBER(2819238471321.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "2.353612".to_string(), literal: Object::NUMBER(2.353612) , line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "1.0".to_string(), literal: Object::NUMBER(1.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "9".to_string(), literal: Object::NUMBER(9.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "8".to_string(), literal: Object::NUMBER(8.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "7".to_string(), literal: Object::NUMBER(7.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "6".to_string(), literal: Object::NUMBER(6.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "5".to_string(), literal: Object::NUMBER(5.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "4".to_string(), literal: Object::NUMBER(4.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "3".to_string(), literal: Object::NUMBER(3.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "2".to_string(), literal: Object::NUMBER(2.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "1".to_string(), literal: Object::NUMBER(1.0), line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NUMBER, lexeme: "0".to_string(), literal: Object::NUMBER(0.0), line: 1 }));  
    }
    #[test]
    fn indentifier() {
        let sample_source = String::from("and or class else false for fun if nil print return super this true var while ");

        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };

        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::WHILE, lexeme: "while".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::TRUE, lexeme: "true".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::THIS, lexeme: "this".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::SUPER, lexeme: "super".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::RETURN, lexeme: "return".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::PRINT, lexeme: "print".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NIL, lexeme: "nil".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IF, lexeme: "if".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::FUN, lexeme: "fun".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::FOR, lexeme: "for".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::FALSE, lexeme: "false".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::ELSE, lexeme: "else".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::CLASS, lexeme: "class".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::OR, lexeme: "or".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::AND, lexeme: "and".to_string(), literal: Object::NULL, line: 1 }));  
    }
    #[test]
    fn maximal_munch() {
        let sample_source = String::from("and andy or orca class  classic else elsea false falset for fore fun funny if iffy nil nile print printer return returner super superfl this thist true truest var vary while whiler");
        let mut scanner = LoxScanner{
            source: sample_source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };

        scanner.scan_tokens();
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::EOF, lexeme: "".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "whiler".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::WHILE, lexeme: "while".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "vary".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::VAR, lexeme: "var".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "truest".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::TRUE, lexeme: "true".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "thist".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::THIS, lexeme: "this".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "superfl".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::SUPER, lexeme: "super".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "returner".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::RETURN, lexeme: "return".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "printer".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::PRINT, lexeme: "print".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "nile".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::NIL, lexeme: "nil".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "iffy".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IF, lexeme: "if".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "funny".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::FUN, lexeme: "fun".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "fore".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::FOR, lexeme: "for".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "falset".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::FALSE, lexeme: "false".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "elsea".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::ELSE, lexeme: "else".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "classic".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::CLASS, lexeme: "class".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "orca".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::OR, lexeme: "or".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::IDENTIFIER, lexeme: "andy".to_string(), literal: Object::NULL, line: 1 }));  
        assert_eq!(scanner.tokens.pop(), Some(Token { token_type: TokenType::AND, lexeme: "and".to_string(), literal: Object::NULL, line: 1 }));  
    }


