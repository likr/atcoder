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
        w: [String; n],
    }
    let mut words = HashSet::new();
    words.insert(w[0].clone());
    for i in 1..n {
        if words.contains(&w[i]) || w[i - 1].chars().last().unwrap() != w[i].chars().nth(0).unwrap()
        {
            eprintln!("{}", i);
            if i % 2 == 0 {
                println!("LOSE");
            } else {
                println!("WIN");
            }
            return;
        }
        words.insert(w[i].clone());
    }
    println!("DRAW");
}
