//use density_rs::*;

use density_rs::{max_2, max_3};
//use std::cmp;

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

fn main() {
    let x = 100;
    let y = 101;
    let z = 102;
    let res = max_3!(x, y, z);
    println!("Max of (x, y): {:?}", res);
    println!("Max of (x, y): {:?}", max_2!(x,y));
}