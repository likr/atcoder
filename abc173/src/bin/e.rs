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
const M: isize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [isize; n],
    }

    let mut a_plus = vec![];
    let mut a_minus = vec![];
    for i in 0..n {
        let ai = a[i];
        if ai > 0 {
            a_plus.push(ai);
        } else if ai < 0 {
            a_minus.push(ai);
        }
    }

    let np = a_plus.len();
    let nm = a_minus.len();

    if np + nm < k {
        eprintln!("zero");
        println!("0");
        return;
    }
    if (np == 0 && k % 2 == 1) || (np + nm == k && nm % 2 == 1) {
        eprintln!("minus");
        a.sort_by_key(|&ai| ai.abs());
        let mut result = 1;
        for i in 0..k {
            result = result * a[i].abs() % M;
        }
        println!("{}", (M - result) % M);
        return;
    }

    a_plus.sort();
    a_plus.reverse();
    a_minus.sort();
    a_plus.push(0);
    a_plus.push(0);
    a_minus.push(0);
    a_minus.push(0);

    let mut result = vec![];
    let mut i = 0;
    let mut j = 0;
    while result.len() < k {
        let x = a_minus[j] * a_minus[j + 1];
        let y = a_plus[i] * a_plus[i + 1];
        if y > x || k - result.len() == 1 {
            result.push(a_plus[i]);
            i += 1;
        } else {
            result.push(a_minus[j]);
            result.push(a_minus[j + 1]);
            j += 2;
        }
    }

    let mut s = 1;
    for &ai in &result {
        s = s * ai.abs() % M;
    }
    println!("{}", s);
}
