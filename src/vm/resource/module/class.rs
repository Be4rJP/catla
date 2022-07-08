use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

static CLASS_PROVIDER: Lazy<ClassProvider> = Lazy::new(|| {ClassProvider});

pub struct ClassProvider {
    pub class_pool: RwLock<HashMap<&'static str, Arc<Class>>>
}

impl ClassProvider {
    pub fn new() -> Self {
        Self {
            class_pool: RwLock::new(HashMap::new())
        }
    }
}


pub struct Class {

}