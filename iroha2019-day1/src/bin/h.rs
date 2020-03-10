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
    }
    let mut m = n;
    let mut d = vec![];
    while m > 0 {
        d.push(m % 10);
        m /= 10;
    }
    d.reverse();
    eprintln!("{:?}", d);

    let mut f = d.iter().sum::<usize>();

    let mut e = vec![];
    while f > 9 {
        e.push(9);
        f -= 9;
    }
    e.push(f);
    e.reverse();
    eprintln!("{:?}", e);

    if d.len() == e.len() && (0..e.len()).all(|i| d[i] == e[i]) {
        if e[0] == 9 || e.len() == 1 {
            e[0] -= 1;
            e.reverse();
            e.push(1);
            e.reverse();
        } else {
            e[0] += 1;
            e[1] -= 1;
        }
    }

    for &ei in &e {
        print!("{}", ei);
    }
    println!("");
}
