use byteperm::{AffinePermutation, BlowfishPermutation, Permutation, XorPermutation};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let p = XorPermutation::new(0xDEAD_BEEF);
    let mut i = 0;
    c.bench_function("xor", |b| {
        b.iter(|| {
            i += 1;
            p.unapply(black_box(p.apply(black_box(i))))
        })
    });

    let p = AffinePermutation::new(0xDEAD_BEEF, 0xCAFE_BABE);
    let mut i = 0;
    c.bench_function("affine", |b| {
        b.iter(|| {
            i += 1;
            p.unapply(black_box(p.apply(black_box(i))))
        })
    });

    let p = BlowfishPermutation::new(0xDEAD_BEEF);
    let mut i = 0;
    c.bench_function("blowfish", |b| {
        b.iter(|| {
            i += 1;
            p.unapply(black_box(p.apply(black_box(i))))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
