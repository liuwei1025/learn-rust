mod front_of_house;

// pub use crate::front_of_house::hosting;
// re-exporting
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolte path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn change_table() -> u32 {
    let mut table = front_of_house::hosting::Table::new(10, 10);
    table.width = 100;
    table.dimension()
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}
