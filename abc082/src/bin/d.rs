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
        x: isize,
        y: isize,
    }
    let n = s.len();
    let mut x_states = vec![vec![false; 2 * n + 1]; n + 1];
    x_states[0][n] = true;
    let mut y_states = vec![vec![false; 2 * n + 1]; n + 1];
    y_states[0][n] = true;

    let mut t_count = 0;
    let mut i = 1;
    while i <= n {
        if s[i - 1] == 'F' {
            let k = i;
            let mut c = 0;
            while i <= n && s[i - 1] == 'F' {
                c += 1;
                i += 1;
            }

            if t_count % 2 == 0 {
                for j in 1..2 * n {
                    if x_states[k - 1][j] {
                        if t_count > 0 {
                            x_states[i - 1][j - c] = true;
                        }
                        x_states[i - 1][j + c] = true;
                    }
                }
                for j in 0..=2 * n {
                    y_states[i - 1][j] = y_states[k - 1][j];
                }
            } else {
                for j in 1..2 * n {
                    if y_states[k - 1][j] {
                        y_states[i - 1][j - c] = true;
                        y_states[i - 1][j + c] = true;
                    }
                }
                for j in 0..=2 * n {
                    x_states[i - 1][j] = x_states[k - 1][j];
                }
            }
        } else {
            for j in 0..=2 * n {
                x_states[i][j] = x_states[i - 1][j];
                y_states[i][j] = y_states[i - 1][j];
            }
            t_count += 1;
            i += 1;
        }
    }

    // eprintln!("{}", n);
    // for i in 1..=n {
    //     eprintln!("{:?}", x_states[i]);
    //     eprintln!("{:?}", y_states[i]);
    //     eprintln!("");
    // }

    let x = n as isize + x;
    let y = n as isize + y;
    if x_states[n][x as usize] && y_states[n][y as usize] {
        println!("Yes");
    } else {
        println!("No");
    }
}
