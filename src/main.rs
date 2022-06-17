extern crate core;

use crate::vm::reference::Reference;

mod syntax;
mod vm;

fn main() {
    println!("Hello, world!");

    let mut reference = Reference::new(20);

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
