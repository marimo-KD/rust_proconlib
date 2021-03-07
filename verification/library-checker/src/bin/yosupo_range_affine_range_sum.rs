use algebra::*;
use lazy_segment_tree::*;
use scanner::*;
use static_modint::*;
use std::io::*;

type ModInt = Modint<P998244353>;

fn main() {
    let (n, q) = scan!(usize, usize);
    let a = scan!([ModInt; n]);
    let a: Vec<_> = a.into_iter().map(|x| AddMonoid{ data: x, len: 1 }).collect();
    let out = stdout();
    let mut out = BufWriter::new(out);
    let mut lst: LazySegmentTree<AddMonoid, AffineMonoid> =
        LazySegmentTree::new_with_init(&a, |t, s| AddMonoid {
            data: s.a * t.data + s.b * ModInt::from(t.len),
            len: t.len,
        });
    for _ in 0..q {
        let ty = scan!(usize);
        match ty {
            0 => {
                let (l, r, b, c) = scan!(usize, usize, ModInt, ModInt);
                lst.update(
                    l..r,
                    AffineMonoid {
                        a: b,
                        b: c,
                    },
                )
            }
            1 => {
                let (l, r) = scan!(usize, usize);
                let ans = lst.query(l..r);
                writeln!(out, "{}", ans.data).unwrap();
            }
            _ => {
                unreachable!()
            }
        }
    }
}

def_monoid! {
    derive(Copy),
    pub struct AddMonoid{
        pub data: ModInt ,
        pub len: usize,
    },
    AddMonoid{
        data: ModInt::zero() ,
        len: 1,
    },
    fn op(lhs: AddMonoid, rhs: AddMonoid) -> AddMonoid {
        AddMonoid {
            data: lhs.data + rhs.data,
            len: lhs.len + rhs.len,
        }
    }
}

def_monoid! {
    derive(Copy),
    pub struct AffineMonoid{
        pub a: ModInt,
        pub b: ModInt,
    },
    AffineMonoid{
        a: ModInt::one(),
        b: ModInt::zero(),
    },
    fn op(old: AffineMonoid, new: AffineMonoid) -> AffineMonoid {
        AffineMonoid{
            a: new.a * old.a,
            b: new.a * old.b + new.b,
        }
    }
}
