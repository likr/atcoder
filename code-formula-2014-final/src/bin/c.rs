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
    let mut row = String::new();
    std::io::stdin().read_line(&mut row).ok();
    let mut row = row.chars().collect::<Vec<_>>();
    if row[row.len() - 1] == '\n' {
        row.pop();
    }

    let mut result = HashSet::new();
    let n = row.len();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        if row[i] == '@' {
            while j < n && row[j] != '@' && row[j] != ' ' {
                j += 1;
            }
            if j > i + 1 {
                result.insert((i + 1..j).map(|k| row[k]).collect::<String>());
            }
        }
        i = j;
    }
    let mut result = result.iter().collect::<Vec<_>>();
    result.sort();
    for ti in result {
        if !ti.is_empty() {
            println!("{}", ti);
        }
    }
}
