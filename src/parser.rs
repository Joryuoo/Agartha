use crate::token::Token;
use crate::ast::{Program, Statement, Expression};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // --- CONSTRUCTOR ---
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // --- MAIN ENGINE ---
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        //Check if naay START
        self.consume(&Token::Start, "Syntax Error: Agartha programs MUST begin with 'START' dong!")?;

        // Loop through everything until we hit END or run out of tokens
        while !self.is_at_end() && !self.check(&Token::End) {
            statements.push(self.parse_statement()?);
        }
        
        //check if naay END
        self.consume(&Token::End, "Syntax Error: Agartha programs MUST finish with 'END' dong!")?;

        Ok(Program { statements })
    }

    // --- STATEMENT ROUTER ---
    fn parse_statement(&mut self) -> Result<Statement, String> {
        // 1. Declarations (e.g., NUMBER num = 10;)
        if self.match_type(&[Token::NumberType, Token::DecimalType, Token::LetterType, Token::WordType, Token::TruthType]) {
            return self.parse_declaration();
        }
        // 2. Display (e.g., display("Hello");)
        if self.check(&Token::Display) {
            return self.parse_display();
        }
        // 3. Input (e.g., input(n);)
        if self.check(&Token::Input) {
            return self.parse_input();
        }
        // 4. If / Else Blocks
        if self.check(&Token::If) {
            return self.parse_if();
        }
        
        // 5. Default to Assignment (e.g., num = 20;)
        self.parse_assignment()
    }

    // --- STATEMENT BUILDERS ---

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        let var_type = self.previous().clone();
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Syntax Error: Expected a variable name after the data type.".to_string()),
        };

        // Support for Uninitialized Variables (e.g., NUMBER n;)
        let value = if self.match_type(&[Token::Assign]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(&Token::Semicolon, "Syntax Error: Nakalimot ka sa semicolon (;) dong!")?;
        
        Ok(Statement::Declaration { var_type, name, value })
    }

    fn parse_assignment(&mut self) -> Result<Statement, String> {
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Syntax Error: Expected a variable name for assignment.".to_string()),
        };

        self.consume(&Token::Assign, "Syntax Error: Expected '=' after variable name.")?;
        let value = self.parse_expression()?;
        self.consume(&Token::Semicolon, "Syntax Error: Nakalimot ka sa semicolon (;) dong!")?;

        Ok(Statement::Assignment { name, value })
    }

    fn parse_display(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'display'
        self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after display.")?;
        let expr = self.parse_expression()?;
        self.consume(&Token::RightParen, "Syntax Error: Expected ')' after display expression.")?;
        self.consume(&Token::Semicolon, "Syntax Error: Nakalimot ka sa semicolon (;) dong!")?;
        
        Ok(Statement::Display(expr))
    }

    fn parse_input(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'input'
        self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after input.")?;
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n.clone(),
            _ => return Err("Syntax Error: Expected variable name inside input().".to_string()),
        };
        
        self.consume(&Token::RightParen, "Syntax Error: Expected ')' after input.")?;
        self.consume(&Token::Semicolon, "Syntax Error: Nakalimot ka sa semicolon (;) dong!")?;
        
        Ok(Statement::Input(name))
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance(); // consume 'if'

        // Condition
        self.consume(&Token::LeftParen, "Syntax Error: Expected '(' after 'if'.")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::RightParen, "Syntax Error: Expected ')' after if condition.")?;

        // Main Body
        self.consume(&Token::LeftBrace, "Syntax Error: Expected '{' to start the if block.")?;
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::RightBrace, "Syntax Error: Expected '}' to close the if block.")?;

        // Optional Else Body
        let mut else_body = None;
        if self.match_type(&[Token::Else]) {
            self.consume(&Token::LeftBrace, "Syntax Error: Expected '{' to start the else block.")?;
            let mut e_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                e_body.push(self.parse_statement()?);
            }
            self.consume(&Token::RightBrace, "Syntax Error: Expected '}' to close the else block.")?;
            else_body = Some(e_body);
        }

        Ok(Statement::If { condition, body, else_body })
    }

    // --- EXPRESSION WATERFALL (Order of Operations) ---

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_equality()
    }

    // 1. EQUALITY (==, !=)
    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;
        while self.match_type(&[Token::Equal, Token::NotEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_comparison()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // 2. COMPARISON (<, >, <=, >=)
    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;
        while self.match_type(&[Token::LessThan, Token::GreaterThan, Token::LessThanOrEqual, Token::GreaterThanOrEqual]) {
            let operator = self.previous().clone();
            let right = self.parse_term()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // 3. TERM (+, -, &)
    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?;
        while self.match_type(&[Token::Add, Token::Subtract, Token::Concat]) {
            let operator = self.previous().clone();
            let right = self.parse_factor()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // 4. FACTOR (*, /, %, ^)
    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;
        while self.match_type(&[Token::Multiply, Token::Divide, Token::Modulo, Token::Exponentiate]) {
            let operator = self.previous().clone();
            let right = self.parse_primary()?;
            expr = Expression::BinaryOp { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // 5. PRIMARY (Raw Values & Parentheses)
    fn parse_primary(&mut self) -> Result<Expression, String> {
        let token = self.advance().ok_or("Syntax Error: Unexpected end of file.")?.clone();
        
        match token {
            Token::NumberLiteral(n) => Ok(Expression::Number(n)),
            Token::DecimalLiteral(d) => Ok(Expression::Decimal(d)),
            Token::WordLiteral(w) => Ok(Expression::Word(w)),
            Token::LetterLiteral(l) => Ok(Expression::Letter(l)),
            Token::TruthLiteral(t) => Ok(Expression::Truth(t)),
            Token::Identifier(id) => Ok(Expression::Identifier(id)),
            
            // Parenthesis logic for grouping math: (5 + 5)
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                self.consume(&Token::RightParen, "Syntax Error: Expected ')' after expression.")?;
                Ok(expr)
            }
            _ => Err(format!("Syntax Error: I expected a value, variable, or math, but found {:?}", token)),
        }
    }

    // --- HELPER TOOLS ---

    // Checks if current token is of a specific type (doesn't care about internal data)
    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(self.peek().unwrap()) == std::mem::discriminant(token_type)
    }

    // Checks a list of types. If one matches, it eats it and returns true.
    fn match_type(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    // Enforcer: Must match this token, or throw an error!
    fn consume(&mut self, token_type: &Token, message: &str) -> Result<&Token, String> {
        if self.check(token_type) { 
            Ok(self.advance().unwrap()) 
        } else { 
            Err(message.to_string()) 
        }
    }

    // Moves the bookmark forward and returns the token we just passed
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() { self.current += 1; }
        self.previous_opt()
    }

    fn is_at_end(&self) -> bool { self.peek().is_none() }
    
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.current) }
    
    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }
    
    fn previous_opt(&self) -> Option<&Token> { self.tokens.get(self.current - 1) }
}