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
        mut s: [Chars; n],
    }
    for i in 0..n {
        s[i].reverse();
    }
    s.sort();
    // for i in 0..n {
    //     eprintln!("{} {:?}", i, s[i]);
    // }
    let mut last_index = vec![vec![-1; 26]; n];
    for i in 0..n {
        for (j, &c) in s[i].iter().enumerate() {
            let c = c as usize - 'a' as usize;
            last_index[i][c] = j as isize;
        }
    }
    // for i in 0..n {
    //     eprintln!("{:?}", last_index[i]);
    // }

    let mut prefix = vec![0; n];
    for i in 1..n {
        let mut k = 0;
        while k < s[i - 1].len() - 1 && k < s[i].len() - 1 && s[i - 1][k] == s[i][k] {
            k += 1;
        }
        prefix[i] = k;
    }
    // eprintln!("{:?}", prefix);

    // let mut debug = 0;
    // for i in 0..n {
    //     for j in 0..n {
    //         if s[i].len() >= s[j].len() {
    //             continue;
    //         }
    //         let l = s[i].len() - 1;
    //         if (0..l).all(|k| s[i][k] == s[j][k]) && (l..s[j].len()).any(|k| s[j][k] == s[i][l]) {
    //             eprintln!("{} {}", i, j);
    //             debug += 1;
    //         }
    //     }
    // }
    // eprintln!("{}", debug);

    let mut result = 0usize;
    for i in 0..n - 1 {
        let length = prefix[i + 1];
        if s[i].len() - 1 != length {
            continue;
        }
        let c = s[i][length] as usize - 'a' as usize;
        for j in i + 1..n {
            if prefix[j] < length {
                break;
            }
            if last_index[j][c] >= length as isize {
                // eprintln!("{} {}", i, j);
                result += 1;
            }
        }
    }
    // for i in 0..n {
    //     if s[i].len() == 1 {
    //         let c = s[i][0] as usize - 'a' as usize;
    //         for j in 0..i {
    //             if last_index[j][c] >= 0 {
    //                 eprintln!("{} {}", i, j);
    //                 result += 1;
    //             }
    //         }
    //     }
    // }
    //
    //
    s.reverse();
    last_index.reverse();
    let mut prefix = vec![0; n];
    for i in 1..n {
        let mut k = 0;
        while k < s[i - 1].len() - 1 && k < s[i].len() - 1 && s[i - 1][k] == s[i][k] {
            k += 1;
        }
        prefix[i] = k;
    }
    for i in 0..n - 1 {
        let length = prefix[i + 1];
        if s[i].len() - 1 != length {
            continue;
        }
        let c = s[i][length] as usize - 'a' as usize;
        for j in i + 1..n {
            if prefix[j] < length {
                break;
            }
            if last_index[j][c] >= length as isize {
                // eprintln!("{} {}", i, j);
                result += 1;
            }
        }
    }
    println!("{}", result);
}
