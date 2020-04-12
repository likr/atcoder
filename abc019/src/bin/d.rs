#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use std::io::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn read() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn main() {
    let n = read().parse::<usize>().unwrap();
    let mut w = 0;
    let mut d = 0;
    for v in 2..=n {
        println!("? 1 {}", v);
        stdout().flush().ok();
        let e = read().parse::<usize>().unwrap();
        if e > d {
            w = v;
            d = e;
        }
    }

    let mut d = 0;
    for v in 1..=n {
        if v != w {
            println!("? {} {}", w, v);
            stdout().flush().ok();
            let e = read().parse::<usize>().unwrap();
            if e > d {
                d = e;
            }
        }
    }
    println!("! {}", d);
}
