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
    input! {
        n: usize,
        m: usize,
        v: usize,
        p: usize,
        a: [usize; n],
    }
    let mut a = a;
    a.sort();
    a.reverse();

    let mut acc = vec![0; n];
    for i in p..n {
        acc[i] = acc[i - 1] + a[i - 1];
    }
    // println!("{:?}", acc);

    let mut result = 0;
    for i in 0..n {
        if a[i] >= a[p - 1] {
            result += 1;
        } else if a[i] + m < a[p - 1] {
            break;
        } else {
            let left = p - 1;
            let right = n - i;
            // println!(" {} {} {}", v, left, right);
            if v > left + right {
                let remains = v - left - right;
                let target = a[i] + m;
                let d = (i - p + 1) * target - acc[i];
                // println!(" {} {}", d, remains * m);
                if d >= remains * m {
                    result += 1;
                }
            } else {
                result += 1;
            }
        }
        // println!("{} {}", i, result);
    }
    println!("{}", result);
}
