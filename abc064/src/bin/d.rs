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
    let mut stack_count = 0;
    let mut left_count = 0;
    for i in 0..n {
        if s[i] == '(' {
            stack_count += 1;
        } else {
            if stack_count == 0 {
                left_count += 1;
            } else {
                stack_count -= 1;
            }
        }
    }
    for _ in 0..left_count {
        print!("(");
    }
    print!("{}", s.iter().collect::<String>());
    for _ in 0..stack_count {
        print!(")");
    }
}
