use crate::lexer::{C1Lexer, C1Token};
use crate::ParseResult;
use std::ops::{Deref, DerefMut};

use C1Token::*;

pub struct C1Parser<'a>(C1Lexer<'a>);
// Implement Deref and DerefMut to enable the direct use of the lexer's methods
impl<'a> Deref for C1Parser<'a> {
    type Target = C1Lexer<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for C1Parser<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> C1Parser<'a> {
    pub fn parse(text: &str) -> ParseResult {
        let mut parser = Self::initialize_parser(text);
        parser.program()
    }

    fn initialize_parser(text: &str) -> C1Parser {
        C1Parser(C1Lexer::new(text))
    }
	fn expect_token(&mut self, token: C1Token) -> ParseResult {
		if self.current_matches(&token) {
            self.eat();
            Ok(())
        } else {
            Err(self.error_message_current("unexpected token"))
        }
	}
	// TODO here
    // program ::= ( functiondefinition )* <EOF>
    fn program(&mut self) -> ParseResult {
		while let Some(_) = self.current_token(){
			self.function_definition()?;
		}
        ParseResult::Ok(())
    }
	// functiondefinition  ::= type <ID> "(" ")" "{" statementlist "}"
	// <ID> identifier
	fn function_definition(&mut self) -> ParseResult {
		self.return_type()?;
		self.expect_token(Identifier)?;
		self.expect_token(LeftParenthesis)?;
		self.expect_token(RightParenthesis)?;
		self.expect_token(LeftBrace)?;
		self.statement_list()?;
		self.expect_token(RightBrace)?;
		Result::Ok(())
    }
	// functioncall ::= <ID> "(" ")"
	fn function_call(&mut self) -> ParseResult {
        self.expect_token(Identifier)?;
		self.expect_token(LeftParenthesis)?;
		self.expect_token(RightParenthesis)?;
		Result::Ok(())
    }
	// statementlist ::= ( block )*
	fn statement_list(&mut self) -> ParseResult {
        while !self.current_empty_or_matches(&RightBrace){
			self.block()?;
		}
		Result::Ok(())
    }
	// block ::= "{" statementlist "}" | statement
	fn block(&mut self) -> ParseResult {
        if self.current_matches(&LeftBrace){
			self.eat();
			self.statement_list()?;
			self.expect_token(RightBrace)
		} 
		else {
			self.statement()
		}
    }
	/*statement       ::= ifstatement
                      | returnstatement ";"
                      | printf ";"
                      | statassignment ";"
                      | functioncall ";" */
	fn statement(&mut self) -> ParseResult {
        if self.current_matches(&KwIf){
			self.if_statement()
		}
		else if self.current_matches(&KwReturn){
			self.return_statement()?;
			self.expect_token(Semicolon)
		}
		else if self.current_matches(&KwPrintf){
			self.printf()?;
			self.expect_token(Semicolon)
		}
		else if self.current_matches(&Identifier){
			if self.next_matches(&Assign){
				self.stat_assignment()?;
			} else {
				self.function_call()?;
			}
			self.expect_token(Semicolon)
		}
		else{
			Result::Err(self.error_message_current("empty statement"))
		}
    }
	//ifstatement ::= <KW_IF> "(" assignment ")" block
	fn if_statement(&mut self) -> ParseResult {
        self.expect_token(KwIf)?;
		self.expect_token(LeftParenthesis)?;
		self.assignment()?;
		self.expect_token(RightParenthesis)?;
		self.block()?;
		Result::Ok(())
    }
	// returnstatement ::= <KW_RETURN> ( assignment )?
	fn return_statement(&mut self) -> ParseResult {
        self.expect_token(KwReturn)?;
		if self.current_empty_or_matches(&Semicolon){
			Result::Ok(())
		}
		else {
			self.assignment()
		}
    }
	// printf ::= <KW_PRINTF> "(" assignment ")"
	fn printf(&mut self) -> ParseResult {
        self.expect_token(KwPrintf)?;
		self.expect_token(LeftParenthesis)?;
		self.assignment()?;
		self.expect_token(RightParenthesis)?;
		Result::Ok(())
    }
	fn return_type(&mut self) -> ParseResult {
        self.any_match_and_eat(&[KwBoolean,KwFloat,KwInt, KwVoid], &self.error_message_current("unexpected type"))
    }
	// statassignment ::= <ID> "=" assignment
	fn stat_assignment(&mut self) -> ParseResult {
        self.expect_token(Identifier)?;
		self.expect_token(Assign)?;
		self.assignment()
    }
	// assignment ::= ( ( <ID> "=" assignment ) | expr )
	fn assignment(&mut self) -> ParseResult {
		if self.current_matches(&Identifier) && self.next_matches(&Assign){
			self.eat(); 
			self.eat();
			self.assignment()
		}else{
			self.expr()
		}
    }
	// expr ::= simpexpr ( ( "==" | "!=" | "<=" | ">=" | "<" | ">" ) simpexpr )?
	fn expr(&mut self) -> ParseResult {
        self.simpexpr()?;
		if self.any_match_current(&[Equal,NotEqual, Less, Greater, LessEqual, GreaterEqual]){
			self.eat();
			self.simpexpr()
		}
		else{
			Result::Ok(())
		}
    }
	// simpexpr ::= ( "-" )? term ( ( "+" | "-" | "||" ) term )*
	fn simpexpr(&mut self) -> ParseResult {
        if self.current_matches(&Minus){
			self.eat();
		}
		self.term()?;
		while self.any_match_current(&[Plus,Minus,Or]){
			self.eat();
			self.term()?;
		}
		Result::Ok(())
    }
	// term ::= factor ( ( "*" | "/" | "&&" ) factor )*
	fn term(&mut self) -> ParseResult {
		self.factor()?;
		while self.any_match_current(&[Asterisk,Slash,And]){
			self.eat();
			self.factor()?;
		}
		Result::Ok(())
    }
	// factor / atom
	fn factor(&mut self) -> ParseResult {
        if self.any_match_current(&[ConstInt, ConstFloat, ConstBoolean]){
			self.eat();
			return Result::Ok(());
		}
		if self.current_matches(&Identifier){
			if self.next_matches(&LeftParenthesis){
				self.function_call()
			}
			else {
				self.eat();
				Result::Ok(())
			}
		}
		else{
			self.expect_token(LeftParenthesis)?;
			self.assignment()?;
			self.expect_token(RightParenthesis)?;
			Result::Ok(())
		}
    }
	

    /// Check whether the current token is equal to the given token. If yes, consume it, otherwise
    /// return an error with the given error message
    fn check_and_eat_token(&mut self, token: &C1Token, error_message: &str) -> ParseResult {
        if self.current_matches(token) {
            self.eat();
            Ok(())
        } else {
            Err(String::from(error_message))
        }
    }

    /// For each token in the given slice, check whether the token is equal to the current token,
    /// consume the current token, and check the next token in the slice against the next token
    /// provided by the lexer.
    /* fn check_and_eat_tokens(&mut self, token: &[C1Token], error_message: &str) -> ParseResult {
        match token
            .iter()
            .map(|t| self.check_and_eat_token(t, error_message))
            .filter(ParseResult::is_err)
            .last()
        {
            None => Ok(()),
            Some(err) => err,
        }
    }
	*/

    /// Check whether the given token matches the current token
    fn current_matches(&self, token: &C1Token) -> bool {
        match &self.current_token() {
            None => false,
            Some(current) => current == token,
        }
    }
	fn current_empty_or_matches(&self, token: &C1Token) -> bool {
        match &self.current_token() {
            None => true,
            Some(current) => current == token,
        }
    }

    /// Check whether the given token matches the next token
    fn next_matches(&self, token: &C1Token) -> bool {
        match &self.peek_token() {
            None => false,
            Some(next) => next == token,
        }
    }

    /// Check whether any of the tokens matches the current token.
    fn any_match_current(&self, token: &[C1Token]) -> bool {
        token.iter().any(|t| self.current_matches(t))
    }

    /// Check whether any of the tokens matches the current token, then consume it
    fn any_match_and_eat(&mut self, token: &[C1Token], error_message: &str) -> ParseResult {
        if token
            .iter()
            .any(|t| self.check_and_eat_token(t, "").is_ok())
        {
            Ok(())
        } else {
            Err(String::from(error_message))
        }
    }

    fn error_message_current(&self, reason: &'static str) -> String {
        match self.current_token() {
            None => format!("{}. Reached EOF", reason),
            Some(_) => format!(
                "{} at line {:?} with text: '{}'",
                reason,
                self.current_line_number().unwrap(),
                self.current_text().unwrap()
            ),
        }
    }

    /*fn error_message_peek(&mut self, reason: &'static str) -> String {
        match self.peek_token() {
            None => format!("{}. Reached EOF", reason),
            Some(_) => format!(
                "{} at line {:?} with text: '{}'",
                reason,
                self.peek_line_number().unwrap(),
                self.peek_text().unwrap()
            ),
        }
    }
	*/

}

#[cfg(test)]
mod tests {
    use crate::parser::{C1Parser, ParseResult};

    fn call_method<'a, F>(parse_method: F, text: &'static str) -> ParseResult
    where
        F: Fn(&mut C1Parser<'a>) -> ParseResult,
    {
        let mut parser = C1Parser::initialize_parser(text);
        if let Err(message) = parse_method(&mut parser) {
            eprintln!("Parse Error: {}", message);
            Err(message)
        } else {
            Ok(())
        }
    }

	// NOTE additional tests
	#[test]
    fn valid_statement() {
		assert!(call_method(C1Parser::statement, "foo();").is_ok());
		assert!(call_method(C1Parser::statement, "if(x==y){}").is_ok());
		assert!(call_method(C1Parser::statement, "return x;").is_ok());
		assert!(call_method(C1Parser::statement, "x=y;").is_ok());
		assert!(call_method(C1Parser::statement, "x=a+b;").is_ok());
	}
	#[test]
    fn valid_blocks() {
        assert!(call_method(C1Parser::block, "{}").is_ok());
		assert!(call_method(C1Parser::block, "{}{}").is_ok());
		assert!(call_method(C1Parser::block, "{{}}").is_ok());
		assert!(call_method(C1Parser::block, "{} x=y;").is_ok());
		assert!(call_method(C1Parser::block, "x=y;").is_ok());
		assert!(call_method(C1Parser::block, "x=y;{}").is_ok());
        assert!(call_method(C1Parser::block, "if(x==y){}{}").is_ok());
    }


    #[test]
    fn parse_empty_program() {
        let result = C1Parser::parse("");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("   ");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("// This is a valid comment!");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("/* This is a valid comment!\nIn two lines!*/\n");
        assert_eq!(result, Ok(()));

        let result = C1Parser::parse("  \n ");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn fail_invalid_program() {
        let result = C1Parser::parse("  bool  ");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("x = 0;");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("// A valid comment\nInvalid line.");
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn valid_function() {
        let result = C1Parser::parse("  void foo() {}  ");
        assert!(result.is_ok());

        let result = C1Parser::parse("int bar() {return 0;}");
        assert!(result.is_ok());

        let result = C1Parser::parse(
            "float calc() {\n\
        x = 1.0;
        y = 2.2;
        return x + y;
        \n\
        }",
        );
        assert!(result.is_ok());

		let result = C1Parser::parse("int blub() {\n\
			blub1 = 23;\n\
			blub2 = 17;\n\
			blub3 = 42;\n\
			blub4 = blub1 * (blub2 + blub3);\n\
			if (blub1 < blub4) return blub2;\n\
			return blub3;\n\
		}");
		assert!(result.is_ok());
    }

    #[test]
    fn fail_invalid_function() {
        let result = C1Parser::parse("  void foo()) {}  ");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse("const bar() {return 0;}");
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse(
            "int bar() {
                                                          return 0;
                                                     int foo() {}",
        );
        println!("{:?}", result);
        assert!(result.is_err());

        let result = C1Parser::parse(
            "float calc(int invalid) {\n\
        x = 1.0;
        y = 2.2;
        return x + y;
        \n\
        }",
        );
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn valid_function_call() {
        assert!(call_method(C1Parser::function_call, "foo()").is_ok());
        assert!(call_method(C1Parser::function_call, "foo( )").is_ok());
        assert!(call_method(C1Parser::function_call, "bar23( )").is_ok());
    }

    #[test]
    fn fail_invalid_function_call() {
        assert!(call_method(C1Parser::function_call, "foo)").is_err());
        assert!(call_method(C1Parser::function_call, "foo{ )").is_err());
        assert!(call_method(C1Parser::function_call, "bar _foo( )").is_err());
    }

    #[test]
    fn valid_statement_list() {
        assert!(call_method(C1Parser::statement_list, "x = 4;").is_ok());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
        y = 2.1;"
        )
        .is_ok());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
        {\
        foo();\n\
        }"
        )
        .is_ok());
        assert!(call_method(C1Parser::statement_list, "{x = 4;}\ny = 1;\nfoo();\n{}").is_ok());
    }

    #[test]
    fn fail_invalid_statement_list() {
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4\n\
        y = 2.1;"
        )
        .is_err());
        assert!(call_method(
            C1Parser::statement_list,
            "x = 4;\n\
        {\
        foo();"
        )
        .is_err());
        assert!(call_method(C1Parser::statement_list, "{x = 4;\ny = 1;\nfoo;\n{}").is_err());
    }

    #[test]
    fn valid_if_statement() {
        assert!(call_method(C1Parser::if_statement, "if(x == 1) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(x == y) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(z) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(true) {}").is_ok());
        assert!(call_method(C1Parser::if_statement, "if(false) {}").is_ok());
    }

    #[test]
    fn fail_invalid_if_statement() {
        assert!(call_method(C1Parser::if_statement, "if(x == ) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if( == y) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if(> z) {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if( {}").is_err());
        assert!(call_method(C1Parser::if_statement, "if(false) }").is_err());
    }

    #[test]
    fn valid_return_statement() {
        assert!(call_method(C1Parser::return_statement, "return x").is_ok());
        assert!(call_method(C1Parser::return_statement, "return 1").is_ok());
        assert!(call_method(C1Parser::return_statement, "return").is_ok());
    }

    #[test]
    fn fail_invalid_return_statement() {
        assert!(call_method(C1Parser::return_statement, "1").is_err());
    }

    #[test]
    fn valid_printf_statement() {
        assert!(call_method(C1Parser::printf, " printf(a+b)").is_ok());
        assert!(call_method(C1Parser::printf, "printf( 1)").is_ok());
        assert!(call_method(C1Parser::printf, "printf(a - c)").is_ok());
    }

    #[test]
    fn fail_invalid_printf_statement() {
        assert!(call_method(C1Parser::printf, "printf( ").is_err());
        assert!(call_method(C1Parser::printf, "printf(printf)").is_err());
        assert!(call_method(C1Parser::printf, "Printf()").is_err());
    }

    #[test]
    fn valid_return_type() {
        assert!(call_method(C1Parser::return_type, "void").is_ok());
        assert!(call_method(C1Parser::return_type, "bool").is_ok());
        assert!(call_method(C1Parser::return_type, "int").is_ok());
        assert!(call_method(C1Parser::return_type, "float").is_ok());
    }

    #[test]
    fn valid_assignment() {
        assert!(call_method(C1Parser::assignment, "x = y").is_ok());
        assert!(call_method(C1Parser::assignment, "x =y").is_ok());
        assert!(call_method(C1Parser::assignment, "1 + 2").is_ok());
    }

    #[test]
    fn valid_stat_assignment() {
        assert!(call_method(C1Parser::stat_assignment, "x = y").is_ok());
        assert!(call_method(C1Parser::stat_assignment, "x =y").is_ok());
        assert!(call_method(C1Parser::stat_assignment, "x =y + t").is_ok());
    }

    #[test]
    fn valid_factor() {
        assert!(call_method(C1Parser::factor, "4").is_ok());
        assert!(call_method(C1Parser::factor, "1.2").is_ok());
        assert!(call_method(C1Parser::factor, "true").is_ok());
        assert!(call_method(C1Parser::factor, "foo()").is_ok());
        assert!(call_method(C1Parser::factor, "x").is_ok());
        assert!(call_method(C1Parser::factor, "(x + y)").is_ok());
    }

    #[test]
    fn fail_invalid_factor() {
        assert!(call_method(C1Parser::factor, "if").is_err());
        assert!(call_method(C1Parser::factor, "(4").is_err());
        assert!(call_method(C1Parser::factor, "bool").is_err());
    }

    #[test]
    fn multiple_functions() {
        assert!(call_method(
            C1Parser::program,
            "void main() { hello();}\nfloat bar() {return 1.0;}"
        )
        .is_ok());
    }
}
