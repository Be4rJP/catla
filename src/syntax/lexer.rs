use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::ops::Add;
use std::sync::{Arc, RwLock};
use regex::Regex;

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
    pub token_length: u16,
    pub serial_position: u32,
}

impl TokenPosition {
    pub fn new(serial_position: u32) -> Self {
        Self {
            line: 0,
            column: 0,
            token_length: 0,
            serial_position
        }
    }
}

pub struct Token {
    pub lexer: Arc<Lexer>,
    pub position: TokenPosition,
    pub word: String,
    pub key_word: Option<&'static KeyWord>,
    pub scanned_type: ScannedTypeMayBe
}

impl Token {
    pub fn new(lexer: Arc<Lexer>, current_position: u32, word: String,
               key_word: Option<&'static KeyWord>) -> Self {
        let position = TokenPosition::new(current_position);
        Self {
            lexer,
            position,
            word,
            key_word,
            scanned_type: ScannedTypeMayBe::NotNumber
        }
    }

    pub fn to_string(&self) -> String {
        return if self.key_word.is_some() {
            "[".to_owned() + self.word.as_str() + "]"
        } else {
            self.word.clone()
        }
    }
}



pub struct Lexer {
    pub source_code: String
}


pub struct KeyWord {
    pub words: Box<Vec<&'static str>>,
}

impl KeyWord {
    fn new(words: Vec<&'static str>) -> Arc<Self> {
        let mut word_max_length = 0;
        for word in (&words).iter() {
            let length = word.chars().count();
            if length > word_max_length {
                word_max_length = length;
            }
        }

        let instance = Arc::new(Self {
            words: Box::new(words)
        });
        unsafe {
            KEYWORD_REGISTRY.add(instance.clone());

            if word_max_length > KEYWORD_MAX_LENGTH {
                KEYWORD_MAX_LENGTH = word_max_length;
            }
        }

        return instance;
    }
}


static mut KEYWORD_REGISTRY: Lazy<KeyWordRegistry> = Lazy::new(|| {KeyWordRegistry::new()});
static mut KEYWORD_MAX_LENGTH: usize = 0;

pub struct KeyWordRegistry {
    keyword_list: Vec<Arc<KeyWord>>
}

impl KeyWordRegistry {
    pub fn new() -> Self {
        Self {
            keyword_list: Vec::new()
        }
    }

    pub fn get_list(&'static self) -> &'static Vec<Arc<KeyWord>> {
        return &self.keyword_list
    }

    pub fn add(&mut self, keyword: Arc<KeyWord>) {
        self.keyword_list.push(keyword);
    }
}


pub static NONE_SPACE: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec![" "])});
pub static NONE_STRING: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["\""])});

pub static EXPRESSION_SPLIT: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["\n", ";"])});
pub static VARIABLE: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["var"])});
pub static OPERATOR_PLUS: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["+"])});
pub static OPERATOR_MINUS: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["-"])});
pub static OPERATOR_MULTIPLY: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["*"])});
pub static OPERATOR_DIVISION: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["/"])});
pub static OPERATOR_REMAINDER: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["%"])});
pub static OPERATOR_ASSIGNMENT: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["="])});
pub static DOT: Lazy<Arc<KeyWord>> = Lazy::new(|| {KeyWord::new(vec!["."])});

#[derive(PartialEq)]
pub enum ScannedTypeMayBe {
    Integer,
    Float,
    NotNumber
}

impl Lexer {

    pub fn new(source_code: String) -> Arc<Lexer> {
        let instance = Self {
            source_code
        };

        return Arc::new(instance);
    }

    pub fn scan(self: Arc<Self>) -> Vec<Arc<Token>> {
        let mut tokens: Vec<Arc<Token>> = Vec::new();

        let mut current_index = 0;
        let mut token_buffer = "".to_string();

        let source_code_length = self.source_code.chars().count();

        'scan : while current_index < source_code_length {
            //Get character at current index.
            let character = self.source_code.chars().nth(current_index).unwrap();

            //Cut the string short in advance to avoid unnecessary comparisons with keywords.
            let word_cut = cut_string(&self.source_code, current_index,
                                      current_index + unsafe { KEYWORD_MAX_LENGTH });
            //Get keyword list.
            let keyword_list = unsafe { KEYWORD_REGISTRY.get_list()};

            let mut character_buffer = true;

            for keyword in keyword_list.iter() {
                for word in keyword.words.iter() {

                    if word_cut.starts_with(word) {
                        //Create token and skip scan.

                        let mut match_keyword = true;

                        //Check has next.
                        if current_index + 1 != source_code_length {
                            //If there is a character other than a symbol to the right of
                            //the detected keyword, it is ignored.
                            let word_length = word.chars().count();
                            let word_last_char: char = word.chars().nth(word_length - 1).unwrap();

                            let next_char: char = self.source_code.chars().nth(current_index + word_length).unwrap();

                            if next_char.is_alphanumeric() && word_last_char.is_alphanumeric() {
                                //Ignore detected keywords.
                                match_keyword = false;
                            }
                        }

                        //Create token by buffer.
                        if token_buffer.len() != 0 {
                            let scanned_type = get_token_type_maybe(token_buffer.as_str());

                            if word.eq(&".") && scanned_type == ScannedTypeMayBe::Integer {
                                //Skip float number token.
                                match_keyword = false;
                            } else {
                                //If not float token.
                                let index = (current_index - token_buffer.len()) as u32;
                                let mut token = Token::new(self.clone(), index,
                                                           token_buffer.clone(), Option::None);
                                token.scanned_type = get_token_type_maybe(token_buffer.as_str());

                                tokens.push(Arc::new(token));
                                token_buffer.clear();
                            }
                        }

                        if match_keyword {
                            match word {
                                &" " => {
                                    //Skip
                                    character_buffer = false;
                                }
                                _ => {
                                    //Create token by keyword.
                                    let token = Token::new(self.clone(), current_index as u32,
                                                           word.to_string(), Option::Some(keyword));
                                    tokens.push(Arc::new(token));
                                    current_index += word.len();
                                    continue 'scan;
                                }
                            }
                        }
                    }
                }
            }

            if character_buffer {
                token_buffer.push(character);
            }

            current_index += 1;
        }

        return tokens;
    }

}

static REG_NORMAL_INT: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[0-9]+$").unwrap()});
static REG_NORMAL_FLOAT: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^[0-9]+\.[0-9]+$").unwrap()});

pub fn get_token_type_maybe(name: &str) -> ScannedTypeMayBe {
    match name {
        n if REG_NORMAL_INT.is_match(n) => ScannedTypeMayBe::Integer,
        n if REG_NORMAL_FLOAT.is_match(n) => ScannedTypeMayBe::Float,
        _ => ScannedTypeMayBe::NotNumber
    }
}

pub fn cut_string(target: &str, start: usize, end: usize) -> String {
    let mut buffer = String::new();
    for i in target.chars().enumerate() {
        match i.0 {
            n if n < start => {continue}
            n if n >= end => {break}
            _ => {buffer.push(i.1)}
        }
    }
    return buffer;
}