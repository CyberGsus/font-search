use criterion::{criterion_group, criterion_main, Criterion};
use fc_test::{sort_search, unword};
use rand::{self, distributions::Alphanumeric, Rng};

fn bench_sort_str_rand(c: &mut Criterion) {
    let rng = rand::thread_rng();

    let filter: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let mapper = sort_search(&filter); // using random filters

    c.bench_function("sort_search_rand", |b| {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(10..=20);
        let sample: String = rng
            .sample_iter(&Alphanumeric)
            .take(r)
            .map(char::from)
            .collect();

        b.iter(|| mapper(&mut unword(&sample)));
    });
}

fn bench_sort_str(c: &mut Criterion) {
    let sorter = sort_search("caskaydia mono");
    c.bench_function("sort_search", |b| {
        b.iter(|| sorter(&mut unword("hi everyone my name is caskaydia mono")))
    });
}

fn bench_unword(c: &mut Criterion) {
    c.bench_function("unword_rand", |b| {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(10..=40);
        let sample: String = rng
            .sample_iter(&Alphanumeric)
            .take(r)
            .map(char::from)
            .collect();

        b.iter(|| for _ in unword(&sample) {})
    });
}
criterion_group!(bench, bench_sort_str, bench_sort_str_rand, bench_unword);
criterion_main!(bench);
