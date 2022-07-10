use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;

static mut CLASS_PROVIDER: Lazy<ClassProvider> = Lazy::new(|| {ClassProvider});

struct ClassProvider {
    pub class_pool: RwLock<HashMap<&'static str, Arc<ClassAccessor>>>
}

impl ClassProvider {
    pub fn new() -> Self {
        Self {
            class_pool: RwLock::new(HashMap::new())
        }
    }

    pub fn register(&mut self, class: Arc<ClassAccessor>) {
        self.class_pool.write().unwrap().insert(class.name, class);
    }
}

fn register_class(class: Arc<ClassAccessor>) {
    unsafe {
        CLASS_PROVIDER.register(class);
    }
}


pub struct ClassAccessor {
    pub name: &'static str
}

impl ClassAccessor {
    pub fn new(name: &'static str) -> Arc<Self> {
        let st = Self {
            name
        };
        let instance = Arc::new(st);
        register_class(instance.clone());
        return instance;
    }
}