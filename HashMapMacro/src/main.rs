#[macro_use]
extern crate std;

use std::collections::HashMap;

macro_rules! hash_map {
    ($($key:expr => $val:expr), *) => {
        {
            let mut tmp = HashMap::new();
            $(
                tmp.insert($key, $val);
            )*
            tmp
        }
    };
}

fn main() {
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };

    println!("{:?}", map);
}
