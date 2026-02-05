//! Criterion benchmark for fetch operations
//!
//! Run with: cargo bench --bench criterion_fetch_bench

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use scapi::domain::fetch::config::FetchConfig;
use scapi::infra::http::HttpClient;
use tokio::runtime::Runtime;

fn bench_streaming_fetch(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let url = "https://httpbin.org/bytes/1048576"; // 1MB

    let mut group = c.benchmark_group("fetch_streaming");
    group.sample_size(20); // More samples for statistical significance

    group.bench_function("streaming_1mb", |b| {
        b.to_async(&rt).iter(|| async {
            let client = HttpClient::new().unwrap();
            let config = FetchConfig::default();
            let content = client.fetch(black_box(url), &config).await.unwrap();
            black_box(content.len())
        });
    });

    group.finish();
}

fn bench_buffered_fetch(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let url = "https://httpbin.org/bytes/1048576"; // 1MB

    let mut group = c.benchmark_group("fetch_buffered");
    group.sample_size(20);

    group.bench_function("buffered_1mb", |b| {
        b.to_async(&rt).iter(|| async {
            let client = HttpClient::new().unwrap();
            let response = client.get(black_box(url)).await.unwrap();
            let content = response.text().await.unwrap();
            black_box(content.len())
        });
    });

    group.finish();
}

fn bench_different_sizes(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let sizes = vec![
        (1024, "1KB"),
        (10240, "10KB"),
        (102400, "100KB"),
        (1048576, "1MB"),
        (5242880, "5MB"),
    ];

    let mut group = c.benchmark_group("fetch_by_size");

    for (size, label) in sizes {
        let url = format!("https://httpbin.org/bytes/{}", size);

        group.bench_with_input(BenchmarkId::new("streaming", label), &url, |b, url| {
            b.to_async(&rt).iter(|| async {
                let client = HttpClient::new().unwrap();
                let config = FetchConfig::default();
                let content = client.fetch(black_box(url), &config).await.unwrap();
                black_box(content.len())
            });
        });

        group.bench_with_input(BenchmarkId::new("buffered", label), &url, |b, url| {
            b.to_async(&rt).iter(|| async {
                let client = HttpClient::new().unwrap();
                let response = client.get(black_box(url)).await.unwrap();
                let content = response.text().await.unwrap();
                black_box(content.len())
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_streaming_fetch,
    bench_buffered_fetch,
    bench_different_sizes
);
criterion_main!(benches);
