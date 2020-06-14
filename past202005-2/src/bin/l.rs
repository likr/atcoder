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
        mut t: [[usize]; n],
        m: usize,
        a: [usize; m],
    }
    for i in 0..n {
        t[i].reverse();
    }

    let mut first = BTreeSet::new();
    let mut second = BTreeSet::new();
    let mut second_item = vec![INF; n];
    for i in 0..n {
        if let Some(tj) = t[i].pop() {
            first.insert(Reverse((tj, i)));
        }
        if let Some(tj) = t[i].pop() {
            second_item[i] = tj;
            second.insert(Reverse((tj, i)));
        }
    }

    let mut result = vec![];
    for k in 0..m {
        if a[k] == 1 || second.is_empty() {
            let Reverse((tj1, i1)) = *first.range(..).nth(0).unwrap();
            first.remove(&Reverse((tj1, i1)));
            result.push(tj1);
            if second_item[i1] != INF {
                first.insert(Reverse((second_item[i1], i1)));
                second.remove(&Reverse((second_item[i1], i1)));
                if let Some(tj) = t[i1].pop() {
                    second.insert(Reverse((tj, i1)));
                    second_item[i1] = tj;
                } else {
                    second_item[i1] = INF;
                }
            }
        } else {
            let Reverse((tj1, i1)) = *first.range(..).nth(0).unwrap();
            let Reverse((tj2, i2)) = *second.range(..).nth(0).unwrap();
            if tj1 > tj2 {
                first.remove(&Reverse((tj1, i1)));
                result.push(tj1);
                if second_item[i1] != INF {
                    first.insert(Reverse((second_item[i1], i1)));
                    second.remove(&Reverse((second_item[i1], i1)));
                    if let Some(tj) = t[i1].pop() {
                        second.insert(Reverse((tj, i1)));
                        second_item[i1] = tj;
                    } else {
                        second_item[i1] = INF;
                    }
                }
            } else {
                second.remove(&Reverse((tj2, i2)));
                result.push(tj2);
                if let Some(tj) = t[i2].pop() {
                    second.insert(Reverse((tj, i2)));
                    second_item[i2] = tj;
                } else {
                    second_item[i2] = INF;
                }
            }
        }
    }
    for i in 0..m {
        println!("{}", result[i]);
    }
}
