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
    }
    let s = s.into_iter().collect::<HashSet<_>>();
    if (s.contains(&'N') && !s.contains(&'S'))
        || (s.contains(&'W') && !s.contains(&'E'))
        || (s.contains(&'S') && !s.contains(&'N'))
        || (s.contains(&'E') && !s.contains(&'W'))
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
