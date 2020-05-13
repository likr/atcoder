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
        a: usize,
        b: usize,
        c: usize,
    }
    let mut visited = HashSet::new();
    let mut state = (a, b, c);
    let mut count = 0;
    while state.0 % 2 == 0 && state.1 % 2 == 0 && state.2 % 2 == 0 {
        visited.insert(state);
        let a = state.1 / 2 + state.2 / 2;
        let b = state.0 / 2 + state.2 / 2;
        let c = state.0 / 2 + state.1 / 2;
        state = (a, b, c);
        if visited.contains(&state) {
            println!("-1");
            return;
        }
        count += 1;
    }
    println!("{}", count);
}
