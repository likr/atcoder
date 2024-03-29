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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn longest_path_rec(graph: &Vec<Vec<usize>>, u: usize, height: &mut Vec<Option<usize>>) -> usize {
    if let Some(h) = height[u] {
        return h;
    }
    let h = graph[u]
        .iter()
        .map(|&v| longest_path_rec(graph, v, height) + 1)
        .max()
        .unwrap_or(0);
    height[u] = Some(h);
    h
}

fn longest_path(graph: &Vec<Vec<usize>>) -> usize {
    let n = graph.len();
    let mut height = vec![None; n];
    (0..n)
        .map(|i| longest_path_rec(graph, i, &mut height))
        .max()
        .unwrap()
}

fn has_cycle(graph: &Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut in_degree = vec![0; n];
    for u in 0..n {
        for &v in &graph[u] {
            in_degree[v] += 1;
        }
    }
    for u in 0..n {
        if in_degree[u] == 0 {
            let mut queue = VecDeque::new();
            visited[u] = true;
            queue.push_back(u);
            while let Some(u) = queue.pop_front() {
                for &v in &graph[u] {
                    if visited[v] {
                        return true;
                    }
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
    }
    return (0..n).any(|u| !visited[u]);
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    let s = if n < m {
        let s0 = s;
        let mut s = vec![];
        while s.len() < m {
            for i in 0..n {
                s.push(s0[i]);
            }
        }
        s
    } else {
        s
    };
    let n = s.len();

    let mut tss = vec![];
    for i in 0..m {
        tss.push(t[i]);
    }
    for i in 0..n {
        tss.push(s[i]);
    }
    for i in 0..n {
        tss.push(s[i]);
    }
    let lcp = z_algorithm_arbitrary(&tss);
    // debug!(lcp);

    let mut graph = vec![vec![]; n];
    for i in 0..n {
        // debug!(i, lcp[m + i]);
        if lcp[m + i] >= m {
            graph[i].push((i + m) % n);
        }
    }
    if has_cycle(&graph) {
        println!("-1");
        return;
    }
    println!("{}", longest_path(&graph));
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod string {
    #![allow(clippy::many_single_char_names)]

    fn sa_naive<T: Ord>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        let mut sa: Vec<usize> = (0..n).collect();
        sa.sort_by(|&(mut l), &(mut r)| {
            if l == r {
                return std::cmp::Ordering::Equal;
            }
            while l < n && r < n {
                if s[l] != s[r] {
                    return s[l].cmp(&s[r]);
                }
                l += 1;
                r += 1;
            }
            if l == n {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        sa
    }

    fn sa_doubling(s: &[i32]) -> Vec<usize> {
        let n = s.len();
        let mut sa: Vec<usize> = (0..n).collect();
        let mut rnk: Vec<i32> = s.to_vec();
        let mut tmp = vec![0; n];
        let mut k = 1;
        while k < n {
            let cmp = |&x: &usize, &y: &usize| {
                if rnk[x] != rnk[y] {
                    return rnk[x].cmp(&rnk[y]);
                }
                let rx = if x + k < n { rnk[x + k] } else { -1 };
                let ry = if y + k < n { rnk[y + k] } else { -1 };
                rx.cmp(&ry)
            };
            sa.sort_by(cmp);
            tmp[sa[0]] = 0;
            for i in 1..n {
                tmp[sa[i]] = tmp[sa[i - 1]]
                    + if cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less {
                        1
                    } else {
                        0
                    };
            }
            std::mem::swap(&mut tmp, &mut rnk);
            k *= 2;
        }
        sa
    }

    trait Threshold {
        fn threshold_naive() -> usize;
        fn threshold_doubling() -> usize;
    }

    enum DefaultThreshold {}
    impl Threshold for DefaultThreshold {
        fn threshold_naive() -> usize {
            10
        }
        fn threshold_doubling() -> usize {
            40
        }
    }

    #[allow(clippy::cognitive_complexity)]
    fn sa_is<T: Threshold>(s: &[usize], upper: usize) -> Vec<usize> {
        let n = s.len();
        match n {
            0 => return vec![],
            1 => return vec![0],
            2 => return if s[0] < s[1] { vec![0, 1] } else { vec![1, 0] },
            _ => (),
        }
        if n < T::threshold_naive() {
            return sa_naive(s);
        }
        if n < T::threshold_doubling() {
            let s: Vec<i32> = s.iter().map(|&x| x as i32).collect();
            return sa_doubling(&s);
        }
        let mut sa = vec![0; n];
        let mut ls = vec![false; n];
        for i in (0..n - 1).rev() {
            ls[i] = if s[i] == s[i + 1] {
                ls[i + 1]
            } else {
                s[i] < s[i + 1]
            };
        }
        let mut sum_l = vec![0; upper + 1];
        let mut sum_s = vec![0; upper + 1];
        for i in 0..n {
            if !ls[i] {
                sum_s[s[i]] += 1;
            } else {
                sum_l[s[i] + 1] += 1;
            }
        }
        for i in 0..=upper {
            sum_s[i] += sum_l[i];
            if i < upper {
                sum_l[i + 1] += sum_s[i];
            }
        }

        // sa's origin is 1.
        let induce = |sa: &mut [usize], lms: &[usize]| {
            for elem in sa.iter_mut() {
                *elem = 0;
            }
            let mut buf = sum_s.clone();
            for &d in lms {
                if d == n {
                    continue;
                }
                let old = buf[s[d]];
                buf[s[d]] += 1;
                sa[old] = d + 1;
            }
            buf.copy_from_slice(&sum_l);
            let old = buf[s[n - 1]];
            buf[s[n - 1]] += 1;
            sa[old] = n;
            for i in 0..n {
                let v = sa[i];
                if v >= 2 && !ls[v - 2] {
                    let old = buf[s[v - 2]];
                    buf[s[v - 2]] += 1;
                    sa[old] = v - 1;
                }
            }
            buf.copy_from_slice(&sum_l);
            for i in (0..n).rev() {
                let v = sa[i];
                if v >= 2 && ls[v - 2] {
                    buf[s[v - 2] + 1] -= 1;
                    sa[buf[s[v - 2] + 1]] = v - 1;
                }
            }
        };
        // origin: 1
        let mut lms_map = vec![0; n + 1];
        let mut m = 0;
        for i in 1..n {
            if !ls[i - 1] && ls[i] {
                lms_map[i] = m + 1;
                m += 1;
            }
        }
        let mut lms = Vec::with_capacity(m);
        for i in 1..n {
            if !ls[i - 1] && ls[i] {
                lms.push(i);
            }
        }
        assert_eq!(lms.len(), m);
        induce(&mut sa, &lms);

        if m > 0 {
            let mut sorted_lms = Vec::with_capacity(m);
            for &v in &sa {
                if lms_map[v - 1] != 0 {
                    sorted_lms.push(v - 1);
                }
            }
            let mut rec_s = vec![0; m];
            let mut rec_upper = 0;
            rec_s[lms_map[sorted_lms[0]] - 1] = 0;
            for i in 1..m {
                let mut l = sorted_lms[i - 1];
                let mut r = sorted_lms[i];
                let end_l = if lms_map[l] < m { lms[lms_map[l]] } else { n };
                let end_r = if lms_map[r] < m { lms[lms_map[r]] } else { n };
                let same = if end_l - l != end_r - r {
                    false
                } else {
                    while l < end_l {
                        if s[l] != s[r] {
                            break;
                        }
                        l += 1;
                        r += 1;
                    }
                    l != n && s[l] == s[r]
                };
                if !same {
                    rec_upper += 1;
                }
                rec_s[lms_map[sorted_lms[i]] - 1] = rec_upper;
            }

            let rec_sa = sa_is::<T>(&rec_s, rec_upper);
            for i in 0..m {
                sorted_lms[i] = lms[rec_sa[i]];
            }
            induce(&mut sa, &mut sorted_lms);
        }
        for elem in sa.iter_mut() {
            *elem -= 1;
        }
        sa
    }

    fn sa_is_i32<T: Threshold>(s: &[i32], upper: i32) -> Vec<usize> {
        let s: Vec<usize> = s.iter().map(|&x| x as usize).collect();
        sa_is::<T>(&s, upper as usize)
    }

    pub fn suffix_array_manual(s: &[i32], upper: i32) -> Vec<usize> {
        assert!(upper >= 0);
        for &elem in s {
            assert!(0 <= elem && elem <= upper);
        }
        sa_is_i32::<DefaultThreshold>(s, upper)
    }

    pub fn suffix_array_arbitrary<T: Ord>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&i| &s[i]);
        let mut s2 = vec![0; n];
        let mut now = 0;
        for i in 0..n {
            if i > 0 && s[idx[i - 1]] != s[idx[i]] {
                now += 1;
            }
            s2[idx[i]] = now;
        }
        sa_is_i32::<DefaultThreshold>(&s2, now)
    }

    pub fn suffix_array(s: &str) -> Vec<usize> {
        let s2: Vec<usize> = s.bytes().map(|x| x as usize).collect();
        sa_is::<DefaultThreshold>(&s2, 255)
    }

    // Reference:
    // T. Kasai, G. Lee, H. Arimura, S. Arikawa, and K. Park,
    // Linear-Time Longest-Common-Prefix Computation in Suffix Arrays and Its
    // Applications
    pub fn lcp_array_arbitrary<T: Ord>(s: &[T], sa: &[usize]) -> Vec<usize> {
        let n = s.len();
        assert!(n >= 1);
        let mut rnk = vec![0; n];
        for i in 0..n {
            rnk[sa[i]] = i;
        }
        let mut lcp = vec![0; n - 1];
        let mut h = 0;
        for i in 0..n - 1 {
            if h > 0 {
                h -= 1;
            }
            if rnk[i] == 0 {
                continue;
            }
            let j = sa[rnk[i] - 1];
            while j + h < n && i + h < n {
                if s[j + h] != s[i + h] {
                    break;
                }
                h += 1;
            }
            lcp[rnk[i] - 1] = h;
        }
        lcp
    }

    pub fn lcp_array(s: &str, sa: &[usize]) -> Vec<usize> {
        let s: &[u8] = s.as_bytes();
        lcp_array_arbitrary(s, sa)
    }

    // Reference:
    // D. Gusfield,
    // Algorithms on Strings, Trees, and Sequences: Computer Science and
    // Computational Biology
    pub fn z_algorithm_arbitrary<T: Ord>(s: &[T]) -> Vec<usize> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }
        let mut z = vec![0; n];
        z[0] = 0;
        let mut j = 0;
        for i in 1..n {
            let mut k = if j + z[j] <= i {
                0
            } else {
                std::cmp::min(j + z[j] - i, z[i - j])
            };
            while i + k < n && s[k] == s[i + k] {
                k += 1;
            }
            z[i] = k;
            if j + z[j] < i + z[i] {
                j = i;
            }
        }
        z[0] = n;
        z
    }

    pub fn z_algorithm(s: &str) -> Vec<usize> {
        let s: &[u8] = s.as_bytes();
        z_algorithm_arbitrary(s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        enum ZeroThreshold {}
        impl Threshold for ZeroThreshold {
            fn threshold_naive() -> usize {
                0
            }
            fn threshold_doubling() -> usize {
                0
            }
        }

        fn verify_all(str: &str, expected_array: &[usize]) {
            let array: Vec<i32> = str.bytes().map(|x| x as i32).collect();
            let sa = sa_doubling(&array);
            assert_eq!(sa, expected_array);
            let sa_naive = sa_naive(&array);
            assert_eq!(sa_naive, expected_array);
            let sa_is = sa_is_i32::<ZeroThreshold>(&array, 255);
            assert_eq!(sa_is, expected_array);

            let sa_str = suffix_array(str);
            assert_eq!(sa_str, expected_array);
        }

        #[test]
        fn test_sa_0() {
            let array = vec![0, 1, 2, 3, 4];
            let sa = sa_doubling(&array);
            assert_eq!(sa, vec![0, 1, 2, 3, 4]);
        }

        #[test]
        fn test_sa_1() {
            let str = "abracadabra";
            verify_all(str, &[10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        }

        #[test]
        fn test_sa_2() {
            let str = "mmiissiissiippii"; // an example taken from https://mametter.hatenablog.com/entry/20180130/p1
            verify_all(str, &[15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4]);
        }

        #[test]
        fn test_lcp_0() {
            let str = "abracadabra";
            let sa = suffix_array(str);
            let lcp = lcp_array(str, &sa);
            assert_eq!(lcp, &[1, 4, 1, 1, 0, 3, 0, 0, 0, 2]);
        }

        #[test]
        fn test_lcp_1() {
            let str = "mmiissiissiippii"; // an example taken from https://mametter.hatenablog.com/entry/20180130/p1
            let sa = suffix_array(str);
            let lcp = lcp_array(str, &sa);
            assert_eq!(lcp, &[1, 2, 2, 6, 1, 1, 5, 0, 1, 0, 1, 0, 3, 1, 4]);
        }

        #[test]
        fn test_z_0() {
            let str = "abracadabra";
            let lcp = z_algorithm(str);
            assert_eq!(lcp, &[11, 0, 0, 1, 0, 1, 0, 4, 0, 0, 1]);
        }

        #[test]
        fn test_z_1() {
            let str = "ababababa";
            let lcp = z_algorithm(str);
            assert_eq!(lcp, &[9, 0, 7, 0, 5, 0, 3, 0, 1]);
        }
    }
}
use string::*;
