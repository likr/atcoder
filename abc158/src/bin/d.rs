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
        s: Chars,
        q: usize,
    }
    let mut s = s.into_iter().collect::<VecDeque<_>>();
    let mut reverse = false;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            reverse = !reverse;
        } else {
            input! {
                f: usize,
                c: char,
            }
            if !reverse && f == 1 || reverse && f == 2 {
                s.push_front(c);
            } else {
                s.push_back(c);
            }
        }
    }
    if reverse {
        println!("{}", s.iter().rev().collect::<String>());
    } else {
        println!("{}", s.iter().collect::<String>());
    }
}
