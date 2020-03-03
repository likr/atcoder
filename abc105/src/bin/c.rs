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
        n: isize,
    }
    let m = 16;

    let mut a = vec![];
    for x in 0..2usize.pow(m as u32) {
        let mut s = 0;
        let mut b = 1;
        for i in 0..m {
            if x & 1 << i > 0 {
                s += b;
            }
            b *= 4;
        }
        a.push(s);
    }
    // eprintln!("{:?}", a);

    let b = a.iter().map(|&ai| -2 * ai).collect::<HashSet<_>>();

    for &ai in &a {
        let bi = n - ai;
        if b.contains(&bi) {
            eprintln!("{} {}", ai, bi);
            let mut s = String::new();
            for k in 0..m {
                let ak = ai >> 2 * k & 1;
                let bk = -bi / 2 >> 2 * k & 1;
                s += &format!("{}{}", ak, bk);
            }
            let s = s
                .chars()
                .rev()
                .skip_while(|&c| c == '0')
                .collect::<String>();
            if s.is_empty() {
                println!("0");
            } else {
                println!("{}", s);
            }
            return;
        }
    }
}
