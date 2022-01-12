#[macro_export]
macro_rules! alloc_boxed_array {
    ($x: expr) => {{
        let layout = Layout::new::<[u32; $x]>();
        unsafe {
            let ptr = alloc_zeroed(layout) as *mut [u32; $x];
            Box::from_raw(ptr)
        }
    }}
}

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

#[macro_export]
macro_rules! unroll_2 {
    ($x: expr) => {{
        $x;
        $x;
    }}
}

#[macro_export]
macro_rules! unroll_4 {
    ($x: expr) => {{
        $x;
        $x;
        $x;
        $x;
    }}
}

#[macro_export]
macro_rules! unroll_8 {
    ($x: expr) => {{
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
    }}
}

#[macro_export]
macro_rules! unroll_16 {
    ($x: expr) => {{
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
    }}
}

#[macro_export]
macro_rules! unroll_32 {
    ($x: expr) => {{
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
    }}
}

#[macro_export]
macro_rules! unroll_64 {
    ($x: expr) => {{
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
        $x;
    }}
}