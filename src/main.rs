#![feature(type_name_of_val)]

use single_borrow::SingleBorrow;

fn main() {
    let t = &&&&&&42;
    dbg!(type_name_of_val(&t));
    let t = t.single_borrow();
    dbg!(type_name_of_val(&t));
    dbg!(t);
}

use std::any::type_name_of_val;
