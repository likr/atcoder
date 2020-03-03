use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }
    print!(
        "{}",
        (s1[0] as usize - 'a' as usize + 'A' as usize) as u8 as char
    );
    print!(
        "{}",
        (s2[0] as usize - 'a' as usize + 'A' as usize) as u8 as char
    );
    print!(
        "{}",
        (s3[0] as usize - 'a' as usize + 'A' as usize) as u8 as char
    );
    println!("");
}
