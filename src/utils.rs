

#[macro_export]
macro_rules! max_3 {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        use std::cmp;
        let y = max!($($z),*);
        cmp::max($x,y)
    }}
}

#[macro_export]
macro_rules! max_2 {
    ($x: expr, $y: expr) => {{
        use std::cmp;
        cmp::max($x,$y)
    }}
}