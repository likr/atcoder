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
        a: Usize1,
        k: Chars,
        b: [Usize1; n],
    }
    let mut path = vec![];
    let mut height = HashMap::new();
    let mut u = a;
    while !height.contains_key(&u) {
        path.push(u);
        height.insert(u, path.len());
        u = b[u];
    }
    eprintln!("{:?}", path);
    eprintln!("{:?}", height);
    let cycle_size = path.len() + 1 - height[&u];

    let mut l = 0;
    let mut base = 1;
    for i in 0..k.len() {
        let d = k[i] as usize - '0' as usize;
        l = (l + base * d) % cycle_size;
        base = (base * 10) % cycle_size;
    }
    eprintln!("{} {}", l, cycle_size);
    l = (l + cycle_size - height[&u]) % cycle_size;

    println!("{}", path[l + height[&u]]);
}
