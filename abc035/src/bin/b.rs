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
        t: usize,
    }
    let n = s.len();
    let mut count = HashMap::new();
    count.insert('L', 0);
    count.insert('R', 0);
    count.insert('U', 0);
    count.insert('D', 0);
    count.insert('?', 0);
    for i in 0..n {
        *count.get_mut(&s[i]).unwrap() += 1;
    }

    let q = count[&'?'];
    let dx = max(count[&'L'], count[&'R']) - min(count[&'L'], count[&'R']);
    let dy = max(count[&'U'], count[&'D']) - min(count[&'U'], count[&'D']);
    if t == 1 {
        println!("{}", dx + dy + q);
    } else {
        if q > dx + dy {
            println!("{}", (q - dx - dy) % 2);
        } else {
            println!("{}", dx + dy - q);
        }
    }
}
