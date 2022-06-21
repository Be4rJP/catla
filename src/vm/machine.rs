use crate::UnsafeReference;
use crate::vm::operator::Operation;

pub struct VirtualMachine {


}

impl VirtualMachine {

    pub fn test(operations: &Vec<Operation>) {

        let mut stack: Vec<u64> = Vec::new();

        for operation in operations.iter() {

        }

    }

}


pub struct Environment {

    pub stack: Vec<Option<Box<StackContent>>>,

}

pub enum StackContent {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    HeepReference()
}