use crate::token::Token;

pub fn tokenize(input: &str) -> Result<Vec<Token>, <String>{ // returns vector of tokens if valid else an error string
    let mut tokens = Vec::new();
    let mut  chars = input.chars().peekable();

    //similar to for each loop
    //Some() is similar to item != null
    while let Some(&c) = chars.peek(){

        //ignore white shape
        if c.is_whitespace(){
            chars.next(); // consume
            continue;
        }

        //pang check sa keyword
        if c.is_alphabetic() || c == '_'{
            let mut word = String::new();

            while let Some(&ch) = chars.peek(){
                if ch.is_alphanumeric() || ch == '_' {
                    word.push(ch);
                    chars.next();
                    // .next() will consume the said character
                    // similar siya sa .deque() sa queue maong dili na siya ma read sa outer loop once ma consume
                    // lol chuya atay
                } else{
                    break;
                }
            }
            //pattern matching?

            let token = match word.as_str(){
                //data types
                "NUMBER" => Token::NumberType,
                "DECIMAL" => Token::DecimalType,
                "LETTER" => Token::LetterType,
                "WORD" => Token::WordType,
                "TRUTH" => Token::TruthType,
                //control
                "if" => Token::If,
                "else" => Token::Else,
                // io
                "display" => Token::Display,
                "input" => Token::Input,
                //boolean values
                "true" => Token::TruthLiteral(true),
                "false" => Token::TruthLiteral(false),
                
                //if not any then it's an identifier
                _ => Token::Identifier(word),

            };
            tokens.push(token);
            continue;
        }

        //for numbers
        if c.is_ascii_digit(){
            let mut num_str = String::new();
            let mut has_decimal = false;

            while let Some(&ch) = chars.peek(){
                if ch.is_ascii_digit(){
                    num_str.push(ch);
                    chars.next();
                } else if ch == '.' && !has_decimal{
                    num_str.push(ch);
                    has_decimal = true;
                    chars.next();
                } else if ch == '.' && has_decimal{
                    return Err(format!("Error MIGO")) //temporary only
                } else{
                    break;
                }
            }

            if has_decimal{
                //check if numerical values are valid
                if let Ok(val) = num_str.parse::<f64>(){
                    tokens.push(Token::DecimalLiteral(val));
                } 
            } else{
                if let Ok(val) = num_str.parse::<i32>(){
                    tokens.push(Token::NumberLiteral(val));
                }
            }
            continue;

        }

        //for operators and other symbols
        match c{
            '=' => {
                chars.next(); //consume the initial character
                if let Some(&'=') = chars.peek(){  
                    tokens.push(Token::Equal);
                    chars.next(); // consume the character
                    
                } else{
                    tokens.push(Token::Assign);
                }
            }

            '<' => {
                chars.next(); // consume the initial character
                if let Some(&'=') = chars.peek(){
                    tokens.push(Token::LessThanOrEqual);
                    chars.next(); //consume
                } else{
                    tokens.push(Token::LessThan);
                }
            }

            '>' => {
                chars.next();
                if let Some(&'=') = chars.peek(){
                    tokens.push(Token::GreaterThanOrEqual);
                    chars.next(); //consume num num
                } else{
                    tokens.push(Token::GreaterThan);
                }
            }

            '+' => {tokens.push(Token::Add); chars.next()}
            '-' => {tokens.push(Token::Subtract); chars.next()}
            '*' => {tokens.push(Token::Multiply); chars.next()}
            '/' => {tokens.push(Token::Divide); chars.next()}
            '%' => {tokens.push(Token::Modulo); chars.next()}
            '&' => {tokens.push(Token::Concat); chars.next()}
            '^' => {tokens.push(Token::Exponentiate); chars.next()}
            '(' => {tokens.push(Token::LeftParen); chars.next()}
            ')' => {tokens.push(Token::RightParen); chars.next()}
            '{' => {tokens.push(Token::LeftBrace); char.next()}
            '}' => {tokens.push(Token::RightBrace); chars.next()}
            ';' => {tokens.push(Token::Semicolon); chars.next()}
            _ => return Err(format!("Unsa mani dong!"))
        }
    }
    Ok(tokens)
}
