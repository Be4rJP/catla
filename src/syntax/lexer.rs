
pub enum TokenType {
    Integer,
    Float,
    Name,
    String,
    OperatorPlus,
    OperatorMinus,
    OperatorMulti,
    OperatorDivision,
    Operator
}

#[derive(Clone)]
pub struct TokenPosition {
    pub line: i32,
    pub column: i32
}

pub struct Token {
    pub lexer: Box<Lexer>,

    pub start_position: TokenPosition,
    pub end_position: TokenPosition,

    pub word: String
}

pub struct Lexer {

}