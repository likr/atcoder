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
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize,
    }
    for _ in 0..tx - sx {
        print!("R");
    }
    for _ in 0..ty - sy {
        print!("U");
    }
    print!("R");
    for _ in 0..ty - sy + 1 {
        print!("D");
    }
    for _ in 0..tx - sx + 1 {
        print!("L");
    }
    print!("U");
    for _ in 0..ty - sy {
        print!("U");
    }
    for _ in 0..tx - sx {
        print!("R");
    }
    print!("U");
    for _ in 0..tx - sx + 1 {
        print!("L");
    }
    for _ in 0..ty - sy + 1 {
        print!("D");
    }
    print!("R");
    println!("");
}
