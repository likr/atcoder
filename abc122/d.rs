use std::collections::HashMap;

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

fn valid(s: &String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    if s[0] == 'A' && s[1] == 'G' && s[3] == 'C' {
        return false;
    }
    if s[0] == 'A' && s[2] == 'G' && s[3] == 'C' {
        return false;
    }
    if s[1] == 'A' && s[2] == 'G' && s[3] == 'C' {
        return false;
    }
    if s[1] == 'G' && s[2] == 'A' && s[3] == 'C' {
        return false;
    }
    if s[1] == 'A' && s[2] == 'C' && s[3] == 'G' {
        return false;
    }
    true
}

const M : usize = 1000000007;

fn main() {
    input! {
        n: usize,
    }
    let chars = vec!['A', 'C', 'G', 'T'];
    let mut dp = vec![HashMap::new(); n + 1];
    for c1 in &chars {
        for c2 in &chars {
            for c3 in &chars {
                let s = format!("{}{}{}", c1, c2, c3);
                if s == "AGC" || s == "GAC" || s == "ACG" {
                    dp[3].insert(s.clone(), 0);
                } else {
                    dp[3].insert(s.clone(), 1);
                }
                for i in 4..n + 1 {
                    dp[i].insert(s.clone(), 0);
                }
            }
        }
    }
    for i in 4..n + 1 {
        for c1 in &chars {
            for c2 in &chars {
                for c3 in &chars {
                    for c4 in &chars {
                        let s0 = format!("{}{}{}", c1, c2, c3);
                        let s1 = format!("{}{}{}", c2, c3, c4);
                        let s = format!("{}{}{}{}", c1, c2, c3, c4);
                        if valid(&s) {
                            let v = dp[i - 1][&s0];
                            let mut r = dp[i].get_mut(&s1).unwrap();
                            *r = (*r + v) % M;
                        }
                    }
                }
            }
        }
    }
    let mut result = 0;
    for &count in dp[n].values() {
        result = (result + count) % M;
    }
    println!("{}", result);
}
