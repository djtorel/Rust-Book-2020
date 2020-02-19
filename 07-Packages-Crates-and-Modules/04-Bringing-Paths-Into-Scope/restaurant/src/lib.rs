use std::{
    cmp::{Eq, Ordering},
    fmt::Result,
    io::{self, Result as IoResult, Write},
};

use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use self::front_of_house::hosting;
// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
