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
        s: Chars,
    }
    let mut t = VecDeque::new();
    t.push_back('b');
    let mut i = 0;
    while t.len() < n {
        i += 1;
        match i % 3 {
            1 => {
                t.push_front('a');
                t.push_back('c')
            }
            2 => {
                t.push_front('c');
                t.push_back('a')
            }
            _ => {
                t.push_front('b');
                t.push_back('b')
            }
        }
    }
    if s.iter().collect::<String>() == t.iter().collect::<String>() {
        println!("{}", i);
    } else {
        println!("-1");
    }
}
