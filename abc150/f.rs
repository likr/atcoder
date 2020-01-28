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

const B1 : usize = 1009;
const B2 : usize = 1007;
const M1 : usize = 1000000007;
const M2 : usize = 1000000009;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut p = vec![0; n];
    let mut q = vec![0; 2 * n];
    for i in 0..n {
        p[i] = b[i] ^ b[(i + 1) % n];
        q[i] = a[i] ^ a[(i + 1) % n];
        q[i + n] = q[i];
    }
    // println!("{:?}", p);
    // println!("{:?}", q);

    let mut q_hash1 = vec![0; 2 * n + 1];
    let mut q_hash2 = vec![0; 2 * n + 1];
    let mut q_pow1 = vec![1; 2 * n + 1];
    let mut q_pow2 = vec![1; 2 * n + 1];
    for i in 0..2 * n {
        q_hash1[i + 1] = (q_hash1[i] + q[i]) * B1 % M1;
        q_hash2[i + 1] = (q_hash2[i] + q[i]) * B2 % M2;
        q_pow1[i + 1] = q_pow1[i] * B1 % M1;
        q_pow2[i + 1] = q_pow2[i] * B2 % M2;
    }
    let mut p_hash1 = vec![0; n + 1];
    let mut p_hash2 = vec![0; n + 1];
    for i in 0..n {
        p_hash1[i + 1] = (p_hash1[i] + p[i]) * B1 % M1;
        p_hash2[i + 1] = (p_hash2[i] + p[i]) * B2 % M2;
    }

    let p_hash = (p_hash1[n], p_hash2[n]);
    for k in 0..n {
        let q_hash = (
            ((q_hash1[n + k] + M1 - q_hash1[k] * q_pow1[n] % M1) % M1 + M1) % M1,
            ((q_hash2[n + k] + M2 - q_hash2[k] * q_pow2[n] % M2) % M2 + M2) % M2
        );
        // println!("{:?} {:?}", p_hash, q_hash);
        if p_hash == q_hash {
            println!("{} {}", k, a[k] ^ b[0]);
        }
    }
}
