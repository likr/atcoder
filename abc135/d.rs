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

const B : usize = 13;
const M : usize = 1000000007;

fn main() {
    input! {
        s: chars,
    }
    let n = s.len();
    let mut s = s;
    s.reverse();
    let mut count = vec![0; B];
    let mut count2 = vec![0; B];

    if s[0] == '?' {
        for d in 0..10 {
            count[d] += 1;
        }
    } else {
        count[s[0] as usize - '0' as usize] += 1;
    }

    let mut base = 10;
    for i in 1..n {
        for j in 0..B {
            count2[j] = 0;
        }
        if s[i] == '?' {
            for d in 0..10 {
                for j in 0..B {
                    let k = (d * base + j) % B;
                    count2[k] = (count2[k] + count[j]) % M;
                }
            }
        } else {
            let d = s[i] as usize - '0' as usize;
            for j in 0..B {
                let k = (d * base + j) % B;
                count2[k] = (count2[k] + count[j]) % M;
            }
        }
        for j in 0..B {
            count[j] = count2[j];
        }
        base = base * 10 % B;
    }
    println!("{}", count[5]);
}
