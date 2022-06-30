use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::ops::Add;
use std::sync::{Arc, RwLock};

pub enum TokenType {
    Int,
    Long,
    Float,
    Double,
    VariableName,
    String,
    OperatorPlus,
    OperatorMinus,
    OperatorMulti,
    OperatorDiv
}

#[derive(Clone)]
pub struct TokenPosition {
    pub line: u32,
    pub column: u32,
    pub token_length: u16
}

pub struct Token {
    pub lexer: Option<Arc<Lexer>>,

    pub start_position: TokenPosition,
    pub end_position: TokenPosition,

    pub word: String
}

pub struct Lexer {

    pub source_code: String

}



pub struct KeyWord {
    pub words: Box<Vec<&'static str>>,
}

impl KeyWord {
    fn new(words: Vec<&'static str>) -> Arc<Self> {
        let instance = Arc::new(Self {
            words: Box::new(words)
        });

        &KEYWORD_LIST.write().unwrap().push(instance.clone());

        return instance;
    }
}


static KEYWORD_LIST: Lazy<RwLock<Vec<Arc<KeyWord>>>> = Lazy::new(|| {RwLock::new(Vec::new())});
static KEYWORD_MAX_LENGTH: usize = 5;

static EXPRESSION_SPLIT: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["\n", ";"])});
static OPERATOR_PLUS: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["+"])});
static OPERATOR_ASSIGNMENT: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["="])});

pub enum ScanningTypeMayBe {
    Integer,
    Float,
    NotNumber
}

impl Lexer {

    pub fn new(source_code: String) -> Self {
        Self {
            source_code
        }
    }

    pub fn scan(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut current_index = 0;
        let mut token_buffer = "".to_string();
        let mut scanning_type = ScanningTypeMayBe::NotNumber;

        while current_index < self.source_code.len() {
            //Get character at current index.
            let character = self.source_code.chars().nth(current_index).unwrap();

            let is_digit = char::is_ascii_digit(&character);

            if token_buffer.len() == 0 {
                if is_digit {
                    scanning_type = ScanningTypeMayBe::Integer;
                } else {
                    scanning_type = ScanningTypeMayBe::NotNumber;
                }
            }

            if !is_digit {
                //let word_cut = self.source_code.

                //Check keywords
                for keyword in KEYWORD_LIST.write().unwrap().iter() {

                }
            }

            current_index += 1;
        }

        return tokens;
    }

}