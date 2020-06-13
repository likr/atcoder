use std::io::Read;

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

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap() + 1;
    let mut c = vec![];
    c.push(0);
    for _ in 1..n {
        c.push(iter.next().unwrap().parse::<usize>().unwrap());
    }
    let q = iter.next().unwrap().parse::<usize>().unwrap();
    let mut odd_min = 10000000000;
    let mut all_min = 10000000000;
    for i in 1..n {
        if i % 2 == 1 {
            if c[i] < odd_min {
                odd_min = c[i];
            }
        }
        if c[i] < all_min {
            all_min = c[i];
        }
    }

    let mut s = vec![0; n];
    let mut s_odd = 0;
    let mut s_all = 0;

    let mut result = 0;
    for _ in 0..q {
        // println!(" {} {} {}", result, odd_min, all_min);
        let t = iter.next().unwrap().parse::<usize>().unwrap();
        if t == 1 {
            let x = iter.next().unwrap().parse::<usize>().unwrap();
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let t = if x % 2 == 1 {
                s[x] + s_odd + s_all + a
            } else {
                s[x] + s_all + a
            };
            if c[x] >= t {
                result += a;
                let d = c[x] - t;
                s[x] += a;
                if x % 2 == 1 {
                    if d < odd_min {
                        odd_min = d;
                    }
                }
                if d < all_min {
                    all_min = d;
                }
            }
        } else if t == 2 {
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            if a <= odd_min {
                result += a * (n / 2);
                s_odd += a;
                odd_min -= a;
                if odd_min < all_min {
                    all_min = odd_min;
                }
            }
        } else {
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            if a <= all_min {
                result += a * (n - 1);
                s_all += a;
                all_min -= a;
                odd_min -= a;
            }
        }
    }
    println!("{}", result);
}
