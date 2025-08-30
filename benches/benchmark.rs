use criterion::{criterion_group, criterion_main, Criterion};
use palindromeda::{IsPalindrome, Palindrome, PalindromeIter};
use std::hint::black_box;

fn closest_bench(c: &mut Criterion) {
    c.bench_function("closest 100", |b| {
        b.iter(|| black_box(Palindrome::closest(289734)))
    });
}

fn nth_bench(c: &mut Criterion) {
    c.bench_function("nth 100", |b| {
        b.iter(|| black_box(Palindrome::nth(2837498)))
    });
}

fn to_n_bench(c: &mut Criterion) {
    const P: Palindrome = Palindrome::closest(100080001);
    c.bench_function("to_n 100", |b| b.iter(|| black_box(Palindrome::to_n(&P))));
}

fn previous_bench(c: &mut Criterion) {
    const P: Palindrome = Palindrome::closest(100080001);
    c.bench_function("previous 100", |b| b.iter(|| black_box(P.previous())));
}

fn next_bench(c: &mut Criterion) {
    const P: Palindrome = Palindrome::closest(23347574332);
    c.bench_function("next 100", |b| b.iter(|| black_box(P.next())));
}

fn le_bench(c: &mut Criterion) {
    c.bench_function("le 100", |b| {
        b.iter(|| black_box(Palindrome::le(928374923)))
    });
}

fn ge_bench(c: &mut Criterion) {
    c.bench_function("ge 100", |b| {
        b.iter(|| black_box(Palindrome::ge(928374923)))
    });
}

fn is_palindrome_bench(c: &mut Criterion) {
    c.bench_function("is_palindrome 100", |b| {
        b.iter(|| {
            for x in 0..100_000u64 {
                black_box(x.is_palindrome());
            }
        })
    });
}

fn iter_from_p_bench(c: &mut Criterion) {
    const START: Palindrome = Palindrome::closest(289734);
    const END: Palindrome = Palindrome::closest(2894545734);
    c.bench_function("iter_from 100", |b| {
        b.iter(|| black_box(PalindromeIter::from(START, END)))
    });
}

fn iter_from_u64_bench(c: &mut Criterion) {
    c.bench_function("iter_from_u64 100", |b| {
        b.iter(|| black_box(PalindromeIter::from(289734u64, 2894545734u64)))
    });
}

fn iter_first_n_bench(c: &mut Criterion) {
    c.bench_function("iter_first_n 100", |b| {
        b.iter(|| black_box(PalindromeIter::first_n(987324)))
    });
}

fn iter_first_n_from_bench(c: &mut Criterion) {
    const P: Palindrome = Palindrome::closest(9734);
    c.bench_function("iter_first_n_from 100", |b| {
        b.iter(|| black_box(PalindromeIter::first_n_from(987324, P)))
    });
}

fn iter_len_bench(c: &mut Criterion) {
    const P: PalindromeIter = PalindromeIter::first_n_from(83345654, Palindrome::closest(98723));
    c.bench_function("iter_len 100", |b| b.iter(|| black_box(P.len())));
}

fn iter_iterate_bench(c: &mut Criterion) {
    const P: PalindromeIter = PalindromeIter::first_n_from(1_000, Palindrome::closest(1_000_000));
    c.bench_function("iter_iterate 100", |b| {
        b.iter(|| black_box(P.for_each(|_| ())))
    });
}

criterion_group!(
    benches,
    closest_bench,
    nth_bench,
    to_n_bench,
    previous_bench,
    next_bench,
    le_bench,
    ge_bench,
    is_palindrome_bench,
    iter_from_p_bench,
    iter_from_u64_bench,
    iter_first_n_bench,
    iter_first_n_from_bench,
    iter_len_bench,
    iter_iterate_bench,
);
criterion_main!(benches);
