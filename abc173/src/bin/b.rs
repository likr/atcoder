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
        n: usize,
        s: [String; n],
    }
    let mut count = HashMap::new();
    count.insert(String::from("AC"), 0);
    count.insert(String::from("WA"), 0);
    count.insert(String::from("TLE"), 0);
    count.insert(String::from("RE"), 0);
    for si in s {
        *count.entry(si).or_insert(0) += 1;
    }
    println!("AC x {}", count[&String::from("AC")]);
    println!("WA x {}", count[&String::from("WA")]);
    println!("TLE x {}", count[&String::from("TLE")]);
    println!("RE x {}", count[&String::from("RE")]);
}
