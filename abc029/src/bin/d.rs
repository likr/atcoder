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
        mut n: Chars,
    }
    n.reverse();
    let n = n
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let m = n.len();
    let mut a = if n[0] == 0 { 0 } else { 1 };
    let mut b = 1;
    let mut c = n[0] + 1;
    for i in 1..m {
        let d = 10usize.pow(i as u32);
        eprintln!("{} {} {} {} {}", n[i], a, b, c, d);
        if n[i] == 1 {
            a = a + b + c;
        } else if n[i] != 0 {
            a = b * n[i] + d + a;
        }
        b = 10 * b + d;
        c += (n[i] + 1) * d;
    }
    println!("{}", a);
}
