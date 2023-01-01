use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn replace_std(text: &str) -> String {
    text.replace("span", "pans")
}

fn replace_quickreplace(text: &mut str) -> &str {
    unsafe { quickreplace::replace_exact_string_unchecked(text, "span", "pans") };
    text
}

fn bench_replace(c: &mut Criterion) {
    let ted_turner = include_str!("../data/ted_turner.html");
    let uiguren = include_str!("../data/uiguren.html");

    for (title, text) in [("Ted Turner", ted_turner), ("Uiguren", uiguren)].into_iter() {
        let mut group = c.benchmark_group(title);

        group.bench_function(BenchmarkId::new("std", title), |b| {
            b.iter_batched_ref(
                || -> String { text.to_string() },
                |s| {
                    replace_std(s.as_mut_str());
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("quickreplace", title), |b| {
            b.iter_batched_ref(
                || -> String { text.to_string() },
                |s| {
                    replace_quickreplace(s.as_mut_str());
                },
                criterion::BatchSize::SmallInput,
            )
        });

        group.finish()
    }
}

criterion_group!(benches, bench_replace);
criterion_main!(benches);
