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

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut r = vec![0; n];
    let mut plus = vec![0; n];
    let mut minus = vec![0; n];
    let mut factors = HashSet::new();
    let mut max = 1;
    let s : usize = a.iter().sum();
    for i in 1..s {
        if i * i > s {
            break;
        }
        if s % i == 0 {
            factors.insert(i);
            factors.insert(s / i);
        }
    }
    println!("{:?}", factors);
    for d in factors {
        if s % d != 0 {
            continue;
        }
        for i in 0..n {
            r[i] = a[i] % d;
        }
        r.sort();
        for i in 0..n {
            plus[i] = if r[i] == 0 { 0 } else { d - r[i] };
            minus[i] = r[i];
        }
        for i in 1..n {
            minus[i] += minus[i - 1];
        }
        for i in (0..n - 1).rev() {
            plus[i] += plus[i + 1];
        }
        for i in 1..n {
            if plus[i] == minus[i - 1] && plus[i] <= k {
                if d > max {
                    max = d;
                }
                break;
            }
        }
    }
    println!("{}", max);
}
