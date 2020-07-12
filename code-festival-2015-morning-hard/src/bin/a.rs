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
        a: [usize; n],
    }
    let mut a = a.into_iter().collect::<VecDeque<_>>();
    let mut front = true;
    let mut result = 0;
    for i in 1..n {
        if i % 2 == 0 {
            if front {
                let a1 = a.pop_front().unwrap();
                let a2 = a.pop_front().unwrap();
                result += a1;
                a.push_front(a1 + a2 + 1);
            } else {
                let a1 = a.pop_back().unwrap();
                let a2 = a.pop_back().unwrap();
                result += a1;
                a.push_back(a1 + a2 + 1);
            }
        } else {
            let af = a.front().unwrap();
            let ab = a.back().unwrap();
            if af < ab {
                let a1 = a.pop_front().unwrap();
                let a2 = a.pop_front().unwrap();
                result += a1;
                a.push_front(a1 + a2 + 1);
                front = true;
            } else {
                let a1 = a.pop_back().unwrap();
                let a2 = a.pop_back().unwrap();
                result += a1;
                a.push_back(a1 + a2 + 1);
                front = false;
            }
        }
    }
    println!("{}", result);
}
