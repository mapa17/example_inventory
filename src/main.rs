mod catalog;
use catalog::Catalog;

fn main() {
    let mut catalog = Catalog::new();
    catalog.init();
    catalog.execute_all();
}
