use std::sync::atomic::{AtomicU64, Ordering};

pub struct Reference<T> {

    reference: Option<*mut T>,

    counter: Box<AtomicU64>

}

impl<T> Reference<T> {

    pub fn new(value: T) -> Self {
        let raw_pointer = Box::into_raw(Box::new(value));
        let option = Option::Some(raw_pointer);

        let counter = Box::new(AtomicU64::new(0));

        Self {
            reference: option,
            counter
        }
    }

    pub fn hold(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn drop(&mut self) {
        let previous = self.counter.fetch_sub(1, Ordering::SeqCst);

        //Drop from heap memory.
        if previous == 1 {
            if self.reference == Option::None {
                panic!("Unexpected heap memory release.")
            }

            let pointer_optional = self.reference.unwrap();

            unsafe {
                //Drop
                let _boxed = Box::from_raw(pointer_optional);
            }
            self.reference = Option::None;
        }
    }

    pub fn get(&self) -> *mut T {
        if self.reference == Option::None {
            panic!("An attempt was made to access a freed memory area.")
        }

        return self.reference.unwrap();
    }

}