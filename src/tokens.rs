#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // readable tokens / keywords
    NumberType, 
    DecimalType,
    LetterType,
    WordType,
    TruthType,
    Identifier(String),
    If,
    Else,
    ElseIf,
    For,
    While,

    // I/O
    Display, // display()
    Input, // input()

    // Datatypes
    NumberLiteral(i32), // 'NUMBER' - Integer ni
    DecimalLiteral(f64), // 'DECIMAL' - Float ni
    LetterLiteral(char), // 'LETTER' - Char ni
    WordLiteral(String), // 'WORD' - String ni
    TruthLiteral(bool),  // 'TRUTH' - Boolean ni lol

    // Operators
    Assign, // =
    Concat, // &
    Add, // +
    Subtract, // -
    Multiply, // *
    Divide, // /
    Modulo, // %
    Exponentiate, // ^

    //boolean operators
    And, // &&
    Or, // ||
    Not, // !

    // logical operators
    Equal, // ==
    NotEqual, // !=
    LessThan, // <
    GreaterThan, // >
    LessThanOrEqual, // <=
    GreaterThanOrEqual, // >=

    // Block Structuring
    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }
    Semicolon, // ;
}   