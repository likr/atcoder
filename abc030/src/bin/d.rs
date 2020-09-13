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
        mut k: Chars,
        b: [Usize1; n],
    }
    k.reverse();

    let mut path = vec![];
    let mut height = HashMap::new();
    let mut u = a;
    while !height.contains_key(&u) {
        height.insert(u, path.len());
        path.push(u);
        u = b[u];
    }
    // eprintln!("{:?}", path);
    // eprintln!("{:?}", height);
    let cycle_size = path.len() - height[&u];

    let short_length = 8;
    if k.len() < short_length {
        let mut m = 0;
        for i in 0..min(short_length, k.len()) {
            m += (k[i] as usize - '0' as usize) * 10usize.pow(i as u32);
        }
        let mut s = a;
        for _ in 0..m {
            s = b[s];
        }
        println!("{}", s + 1);
    } else {
        let mut l = 0;
        let mut base = 1;
        for i in 0..k.len() {
            let d = k[i] as usize - '0' as usize;
            l = (l + base * d) % cycle_size;
            base = (base * 10) % cycle_size;
        }
        eprintln!("{} {} {}", height[&u], l, cycle_size);

        l = (l + cycle_size - (height[&u] % cycle_size)) % cycle_size;

        let mut s = u;
        for _ in 0..l {
            s = b[s];
        }

        println!("{}", s + 1);
    }
}
