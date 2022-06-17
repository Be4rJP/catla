



pub struct Operation {

    pub code_position_line: i32,

    pub code_position_column: i32,

    pub code_length: i32,

    //VM operation code
    pub ope_code: OpeCode,
    //VM operation argument
    pub ope_arg: u64

}

pub enum OpeCode {
    Push,
    Pop,
    HoldHeapMem,
    DropHeapMem,
    IAdd,
    LAdd,
    FAdd,
    DAdd,
    IMin,
    LMin,
    FMin,
    DMin,
    IMul,
    LMul,
    FMul,
    DMul,
    IDiv,
    LDiv,
    FDiv,
    DDiv,
}