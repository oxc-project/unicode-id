extern crate criterion;
extern crate unicode_id;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use unicode_id::UnicodeID;

fn bench_unicode_id(c: &mut Criterion) {
    let unicode_chars = chars(1..0x3000);
    let ascii_chars = chars(1..0x80);

    let mut group = c.benchmark_group("UnicodeID");
    group.throughput(Throughput::Bytes(unicode_chars.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("is_id_start", "unicode"),
        &unicode_chars,
        |b, chars| b.iter(|| chars.iter().rev().map(|ch| UnicodeID::is_id_start(*ch)).next()),
    );
    group.throughput(Throughput::Bytes(ascii_chars.len() as u64));
    group.bench_with_input(BenchmarkId::new("is_id_start", "ascii"), &ascii_chars, |b, chars| {
        b.iter(|| chars.iter().rev().map(|ch| UnicodeID::is_id_start(*ch)).next())
    });
    group.throughput(Throughput::Bytes(unicode_chars.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("is_id_continue", "unicode"),
        &unicode_chars,
        |b, chars| b.iter(|| chars.iter().rev().map(|ch| UnicodeID::is_id_continue(*ch)).next()),
    );
    group.throughput(Throughput::Bytes(ascii_chars.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("is_id_continue", "ascii"),
        &ascii_chars,
        |b, chars| b.iter(|| chars.iter().rev().map(|ch| UnicodeID::is_id_continue(*ch)).next()),
    );
    group.finish();
}

fn chars(range: std::ops::Range<u32>) -> Vec<char> {
    range.filter_map(std::char::from_u32).collect()
}

criterion_group!(benches, bench_unicode_id);
criterion_main!(benches);
