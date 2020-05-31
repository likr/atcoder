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
        b: [char; 10],
        n: usize,
        a: [Chars; n],
    }
    let mut d = vec![' '; 10];
    for i in 0..10 {
        d[b[i] as usize - '0' as usize] = (i as u8 + '0' as u8) as char;
    }
    let c = a
        .iter()
        .map(|ai| {
            ai.iter()
                .map(|&c| d[c as usize - '0' as usize])
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| c[i]);
    // eprintln!("{:?}", c);
    // eprintln!("{:?}", indices);
    for &i in &indices {
        println!("{}", a[i].iter().collect::<String>());
    }
}
