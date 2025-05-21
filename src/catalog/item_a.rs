use std::sync::{Arc, LazyLock};

use super::{ItemTrait, WrappedItem};

struct ItemA {
    number: u32,
    name: &'static str,
}

impl ItemTrait for ItemA {
    fn get_name(&self) -> String {
        return self.name.to_string();
    }

    fn execute(&self, text: String) -> String {
        return format!("Executing ItemA with {0}", text);
    }
}


inventory::submit!(
    WrappedItem {item: Arc::new(
        ItemA { number: 42, name: "Bob"}
    )}
);