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

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n],
    }
    let mut indices = (0..n).collect::<Vec<usize>>();
    indices.sort_by_key(|&i| xy[i].0);
    let mut left = Segtree::<Additive<usize>>::new(n);
    let mut right = Segtree::<Additive<usize>>::new(n);
    for &(_, yi) in &xy {
        right.set(yi, right.get(yi) + 1);
    }
    debug!((0..n).map(|i| left.get(i)).collect::<Vec<_>>());
    debug!((0..n).map(|i| right.get(i)).collect::<Vec<_>>());
    let mut sol = vec![];
    for &i in &indices {
        let (xi, yi) = xy[i];
        left.set(yi, left.get(yi) + 1);
        right.set(yi, right.get(yi) - 1);
        let left_y_min = left.max_right(0, |&v| v == 0);
        debug!(left_y_min);
        if right.prod(left_y_min, n) <= 0 {
            sol.push(xi);
        }
    }
    debug!(sol);
    let mut components = Dsu::new(n);
    sol.reverse();
    let mut k = 0;
    for i in 0..n {
        let (xi, _) = xy[indices[i]];
        if sol[sol.len() - 1] == xi {
            for j in k..=i {
                components.merge(indices[i], indices[j]);
            }
            sol.pop();
            k = i + 1;
        }
    }
    for i in 0..n {
        println!("{}", components.size(i));
    }
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
pub mod internal_bit {
    // Skipped:
    //
    // - `bsf` = `__builtin_ctz`: is equivalent to `{integer}::trailing_zeros`

    #[allow(dead_code)]
    pub(crate) fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn ceil_pow2() {
            // https://github.com/atcoder/ac-library/blob/2088c8e2431c3f4d29a2cfabc6529fe0a0586c48/test/unittest/bit_test.cpp
            assert_eq!(0, super::ceil_pow2(0));
            assert_eq!(0, super::ceil_pow2(1));
            assert_eq!(1, super::ceil_pow2(2));
            assert_eq!(2, super::ceil_pow2(3));
            assert_eq!(2, super::ceil_pow2(4));
            assert_eq!(3, super::ceil_pow2(5));
            assert_eq!(3, super::ceil_pow2(6));
            assert_eq!(3, super::ceil_pow2(7));
            assert_eq!(3, super::ceil_pow2(8));
            assert_eq!(4, super::ceil_pow2(9));
            assert_eq!(30, super::ceil_pow2(1 << 30));
            assert_eq!(31, super::ceil_pow2((1 << 30) + 1));

            assert_eq!(32, super::ceil_pow2(u32::max_value()));
        }
    }
}
pub mod internal_type_traits {
    use std::{
        fmt,
        iter::{Product, Sum},
        ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        },
    };

    // Skipped:
    //
    // - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
    // - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
    // - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

    /// Corresponds to `std::is_integral` in C++.
    // We will remove unnecessary bounds later.
    //
    // Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
    // same way as the original ACL.
    pub trait Integral:
        'static
        + Send
        + Sync
        + Copy
        + Ord
        + Not<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + BitXor<Output = Self>
        + BitOrAssign
        + BitAndAssign
        + BitXorAssign
        + Shl<Output = Self>
        + Shr<Output = Self>
        + ShlAssign
        + ShrAssign
        + fmt::Display
        + fmt::Debug
        + fmt::Binary
        + fmt::Octal
        + Zero
        + One
        + BoundedBelow
        + BoundedAbove
    {
    }

    /// Class that has additive identity element
    pub trait Zero {
        /// The additive identity element
        fn zero() -> Self;
    }

    /// Class that has multiplicative identity element
    pub trait One {
        /// The multiplicative identity element
        fn one() -> Self;
    }

    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

    impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
pub mod segtree {
    use crate::internal_bit::ceil_pow2;
    use crate::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
    use std::cmp::{max, min};
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul};

    // TODO Should I split monoid-related traits to another module?
    pub trait Monoid {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
    }

    pub struct Max<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Max<S>
    where
        S: Copy + Ord + BoundedBelow,
    {
        type S = S;
        fn identity() -> Self::S {
            S::min_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            max(*a, *b)
        }
    }

    pub struct Min<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Min<S>
    where
        S: Copy + Ord + BoundedAbove,
    {
        type S = S;
        fn identity() -> Self::S {
            S::max_value()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            min(*a, *b)
        }
    }

    pub struct Additive<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Additive<S>
    where
        S: Copy + Add<Output = S> + Zero,
    {
        type S = S;
        fn identity() -> Self::S {
            S::zero()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }

    pub struct Multiplicative<S>(Infallible, PhantomData<fn() -> S>);
    impl<S> Monoid for Multiplicative<S>
    where
        S: Copy + Mul<Output = S> + One,
    {
        type S = S;
        fn identity() -> Self::S {
            S::one()
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a * *b
        }
    }

    impl<M: Monoid> Default for Segtree<M> {
        fn default() -> Self {
            Segtree::new(0)
        }
    }
    impl<M: Monoid> Segtree<M> {
        pub fn new(n: usize) -> Segtree<M> {
            vec![M::identity(); n].into()
        }
    }
    impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
        fn from(v: Vec<M::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![M::identity(); 2 * size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = Segtree { n, size, log, d };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<M: Monoid> Segtree<M> {
        pub fn set(&mut self, mut p: usize, x: M::S) {
            assert!(p < self.n);
            p += self.size;
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }

        pub fn get(&self, p: usize) -> M::S {
            assert!(p < self.n);
            self.d[p + self.size].clone()
        }

        pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
            assert!(l <= r && r <= self.n);
            let mut sml = M::identity();
            let mut smr = M::identity();
            l += self.size;
            r += self.size;

            while l < r {
                if l & 1 != 0 {
                    sml = M::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = M::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }

            M::binary_operation(&sml, &smr)
        }

        pub fn all_prod(&self) -> M::S {
            self.d[1].clone()
        }

        pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(f(&M::identity()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            let mut sm = M::identity();
            while {
                // do
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !f(&M::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        l *= 2;
                        let res = M::binary_operation(&sm, &self.d[l]);
                        if f(&res) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = M::binary_operation(&sm, &self.d[l]);
                l += 1;
                // while
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }

        pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
        where
            F: Fn(&M::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(f(&M::identity()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            let mut sm = M::identity();
            while {
                // do
                r -= 1;
                while r > 1 && r % 2 == 1 {
                    r >>= 1;
                }
                if !f(&M::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        r = 2 * r + 1;
                        let res = M::binary_operation(&self.d[r], &sm);
                        if f(&res) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = M::binary_operation(&self.d[r], &sm);
                // while
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }

        fn update(&mut self, k: usize) {
            self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
    }

    // Maybe we can use this someday
    // ```
    // for i in 0..=self.log {
    //     for j in 0..1 << i {
    //         print!("{}\t", self.d[(1 << i) + j]);
    //     }
    //     println!();
    // }
    // ```

    pub struct Segtree<M>
    where
        M: Monoid,
    {
        // variable name is _n in original library
        n: usize,
        size: usize,
        log: usize,
        d: Vec<M::S>,
    }

    #[cfg(test)]
    mod tests {
        use crate::segtree::Max;
        use crate::Segtree;

        #[test]
        fn test_max_segtree() {
            let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
            let n = base.len();
            let segtree: Segtree<Max<_>> = base.clone().into();
            check_segtree(&base, &segtree);

            let mut segtree = Segtree::<Max<_>>::new(n);
            let mut internal = vec![i32::min_value(); n];
            for i in 0..n {
                segtree.set(i, base[i]);
                internal[i] = base[i];
                check_segtree(&internal, &segtree);
            }

            segtree.set(6, 5);
            internal[6] = 5;
            check_segtree(&internal, &segtree);

            segtree.set(6, 0);
            internal[6] = 0;
            check_segtree(&internal, &segtree);
        }

        //noinspection DuplicatedCode
        fn check_segtree(base: &[i32], segtree: &Segtree<Max<i32>>) {
            let n = base.len();
            #[allow(clippy::needless_range_loop)]
            for i in 0..n {
                assert_eq!(segtree.get(i), base[i]);
            }
            for i in 0..=n {
                for j in i..=n {
                    assert_eq!(
                        segtree.prod(i, j),
                        base[i..j].iter().max().copied().unwrap_or(i32::min_value())
                    );
                }
            }
            assert_eq!(
                segtree.all_prod(),
                base.iter().max().copied().unwrap_or(i32::min_value())
            );
            for k in 0..=10 {
                let f = |&x: &i32| x < k;
                for i in 0..=n {
                    assert_eq!(
                        Some(segtree.max_right(i, f)),
                        (i..=n)
                            .filter(|&j| f(&base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .max()
                    );
                }
                for j in 0..=n {
                    assert_eq!(
                        Some(segtree.min_left(j, f)),
                        (0..=j)
                            .filter(|&i| f(&base[i..j]
                                .iter()
                                .max()
                                .copied()
                                .unwrap_or(i32::min_value())))
                            .min()
                    );
                }
            }
        }
    }
}
use dsu::*;
use segtree::*;
