use crate::syntax::lexer::TokenPosition;

//OpeCodes
const NONE: u16 = 0x00;
const NULL: u16 = 0x01;
const PUSH_CONST: u16 = 0x02;
const B_ADD: u16 = 0x03;
const S_ADD: u16 = 0x04;
const I_ADD: u16 = 0x05;
const L_ADD: u16 = 0x06;
const F_ADD: u16 = 0x07;
const D_ADD: u16 = 0x08;
const B_MIN: u16 = 0x09;
const S_MIN: u16 = 0x0A;
const I_MIN: u16 = 0x0B;
const L_MIN: u16 = 0x0C;
const F_MIN: u16 = 0x0D;
const D_MIN: u16 = 0x0E;
const B_MUL: u16 = 0x0F;
const S_MUL: u16 = 0x10;
const I_MUL: u16 = 0x11;
const L_MUL: u16 = 0x12;
const F_MUL: u16 = 0x13;
const D_MUL: u16 = 0x14;
const B_DIV: u16 = 0x15;
const S_DIV: u16 = 0x16;
const I_DIV: u16 = 0x17;
const L_DIV: u16 = 0x18;
const F_DIV: u16 = 0x19;
const D_DIV: u16 = 0x1A;




pub struct Operation {

    pub ope_code: u16,

    pub arg: u16,

    pub token_position: Option<Box<TokenPosition>>

}