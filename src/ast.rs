use crate::token::Token;

#[derive(Debug, Clone)]
//espression enum para for things that compute values  
pub enum Expression{
    Number(i32),
    Decimal(f64),
    Word(String),
    Letter(char),
    Truth(bool),
    Identifier(String),

    BinaryOp{
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>, 
    },

}
// statements - mga di mu return og value
#[derive(Debug, Clone)]
pub enum Statement{
    //example  WORD name = "Agartha"
    Declaration{
        var_type: Token,
        name: String,
        value: Option<Expression>, // naka  option since value may or may not be present 
                                   // example: WORD name;  (no value assigned) vs WORD name = "Agartha"; (value assigned)
    },

    //assignment
    // NUMBER num = 67;
    Assignment{
        name: String,
        value: Expression,
    },

    Display(Expression),
    Input(String), // string lang para pwde ingani input(var_name); wala nay %c %d etc kay hasol

    If{
        condition: Expression,
        body: Vec<Statement>, //list of actions?
        else_body: Option<Vec<Statement>>,
    }
}

// THE ROOT NODE 
#[derive(Debug)]
pub struct Program {
    //  just a massive list of statements executed top to bottom
    pub statements: Vec<Statement>, 
}