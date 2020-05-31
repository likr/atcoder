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
        w: [String; n],
    }
    let mut convert = HashMap::new();
    convert.insert('b', '1');
    convert.insert('c', '1');
    convert.insert('d', '2');
    convert.insert('w', '2');
    convert.insert('t', '3');
    convert.insert('j', '3');
    convert.insert('f', '4');
    convert.insert('q', '4');
    convert.insert('l', '5');
    convert.insert('v', '5');
    convert.insert('s', '6');
    convert.insert('x', '6');
    convert.insert('p', '7');
    convert.insert('m', '7');
    convert.insert('h', '8');
    convert.insert('k', '8');
    convert.insert('n', '9');
    convert.insert('g', '9');
    convert.insert('z', '0');
    convert.insert('r', '0');
    println!(
        "{}",
        w.iter()
            .map(|wi| {
                wi.to_lowercase()
                    .chars()
                    .filter(|&c| convert.contains_key(&c))
                    .map(|c| convert[&c])
                    .collect::<String>()
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
