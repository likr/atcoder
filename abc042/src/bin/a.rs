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
        mut abc: [usize; 3],
    }
    abc.sort();
    if abc[0] == 5 && abc[1] == 5 && abc[2] == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
