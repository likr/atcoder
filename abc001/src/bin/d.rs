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
        se: [String; n],
    }
    let se = se
        .into_iter()
        .map(|t| {
            let tmp = t.split('-').collect::<Vec<_>>();
            let mut si = tmp[0].parse::<usize>().unwrap();
            let mut ei = tmp[1].parse::<usize>().unwrap();
            si = si / 5 * 5;
            ei = (ei + 4) / 5 * 5;
            if si % 100 == 60 {
                si = (si / 100 + 1) * 100;
            }
            if ei % 100 == 60 {
                ei = (ei / 100 + 1) * 100;
            }
            (si, ei)
        })
        .collect::<Vec<(usize, usize)>>();
    let mut events = BinaryHeap::new();
    for &(si, ei) in &se {
        events.push(Reverse((si, false)));
        events.push(Reverse((ei, true)));
    }
    let mut t = 0;
    let mut c = 0;
    while let Some(Reverse((ti, remove))) = events.pop() {
        if remove {
            c -= 1;
            if c == 0 {
                println!("{:04}-{:04}", t, ti);
            }
        } else {
            if c == 0 {
                t = ti;
            }
            c += 1;
        }
    }
}
