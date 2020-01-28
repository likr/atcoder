use std::cmp::Ordering;

#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn cmp(x1: &Vec<usize>, x2: &Vec<usize>) -> Ordering {
    if x1 == x2 {
        Ordering::Equal
    } else {
        let s1 = x1.iter().sum::<usize>();
        let s2 = x2.iter().sum::<usize>();
        if s1 > s2 {
            Ordering::Greater
        } else if s1 < s2 {
            Ordering::Less
        } else {
            for (&x1i, &x2i) in x1.iter().zip(x2) {
                if x1i > x2i {
                    return Ordering::Greater;
                } else if x1i < x2i {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    a.sort();
    a.reverse();
    let cost = vec![2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut dp0 = vec![(false, vec![0; m]); n + 1];
    let mut dp1 = vec![(false, vec![0; m]); n + 1];
    dp0[0].0 = true;
    for i in 0..m {
        let ai = a[i];
        let bi = cost[ai - 1];
        for j in 0..n + 1 {
            if j < bi {
                if dp0[j].0 {
                    dp1[j].0 = dp0[j].0;
                    for k in 0..m {
                        dp1[j].1[k] = dp0[j].1[k];
                    }
                }
            } else {
                if dp1[j - bi].0 {
                    dp1[j].0 = dp1[j - bi].0;
                    for k in 0..m {
                        dp1[j].1[k] = dp1[j - bi].1[k];
                    }
                    dp1[j].1[i] += 1;
                }
                if dp0[j].0 {
                    let r = cmp(&dp1[j].1, &dp0[j].1);
                    if !dp1[j].0 || r == Ordering::Less {
                        dp1[j].0 = dp0[j].0;
                        for k in 0..m {
                            dp1[j].1[k] = dp0[j].1[k];
                        }
                    }
                }
            }
        }
        for j in 0..n + 1 {
            dp0[j].0 = dp1[j].0;
            for k in 0..m {
                dp0[j].1[k] = dp1[j].1[k];
            }
        }
    }
    // println!("{:?}", dp0[n].1);
    for i in 0..m {
        for _ in 0..dp0[n].1[i] {
            print!("{}", a[i]);
        }
    }
    println!("");
}
