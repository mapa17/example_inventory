use std::collections::HashMap;
use std::sync::Arc;

use inventory;

mod item_a;
mod item_b;

pub trait ItemTrait {
    fn get_name(&self) -> String;
    fn execute(&self, text: String) -> String;
}
pub struct WrappedItem {
    pub item: Arc<dyn ItemTrait + Send + Sync>,
}
inventory::collect!(WrappedItem);

pub struct Catalog {
    items: HashMap<String, Arc<dyn ItemTrait + Send + Sync>>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            items: HashMap::new(),
        }
    }
    
    pub fn register_item(&mut self, name: String, addon: Arc<dyn ItemTrait + Send + Sync>) {
        println!("Register {} ...", name);
        self.items.insert(name, addon);
    }
    
    pub fn execute_all(&self) {
        println!("Executing all registered items {}...", self.items.len());

        // TODO: Execute items in parallel using Rayon

        for (name, item) in &self.items {
            println!("Executing item: {}", name);
            
            let response = item.execute(format!("Hello {}", name));

            println!("Result {}", response);
            
        }
    }

    pub fn init(&mut self) {
        println!("Initializing the catalog ...");
        for wrapped_item in inventory::iter::<WrappedItem> {
            let item = &wrapped_item.item;
            let name = item.get_name();
            self.register_item(name.clone(), item.clone());            
        }
    }
}