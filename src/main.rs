extern crate core;

use crate::vm::reference::UnsafeReference;

mod syntax;
mod vm;

fn main() {
    println!("Hello, world!");


    let mut reference = UnsafeReference::new(20);

    unsafe {
        reference.hold();
        println!("1 : {}", *reference.get());

        reference.hold();
        println!("2 : {}", *reference.get());

        reference.drop();
        println!("1 : {}", *reference.get());

        reference.drop();
        println!("0 : {}", *reference.get());
    }
}


