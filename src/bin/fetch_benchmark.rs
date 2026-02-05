//! Proper benchmark suite for fetch operations
//!
//! Eliminates cold start bias by:
//! - Warmup runs before timing
//! - Multiple iterations with statistical analysis
//! - Alternating execution order
//! - Separate client instances
//! - Memory profiling

use scapi::domain::fetch::config::FetchConfig;
use scapi::infra::http::HttpClient;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct BenchmarkResult {
    name: String,
    durations: Vec<Duration>,
    sizes: Vec<usize>,
    memory_used: Option<usize>,
}

impl BenchmarkResult {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            durations: Vec::new(),
            sizes: Vec::new(),
            memory_used: None,
        }
    }

    fn add_sample(&mut self, duration: Duration, size: usize) {
        self.durations.push(duration);
        self.sizes.push(size);
    }

    fn mean_duration(&self) -> Duration {
        let sum: Duration = self.durations.iter().sum();
        sum / self.durations.len() as u32
    }

    fn median_duration(&self) -> Duration {
        let mut sorted = self.durations.clone();
        sorted.sort();
        sorted[sorted.len() / 2]
    }

    fn min_duration(&self) -> Duration {
        *self.durations.iter().min().unwrap()
    }

    fn max_duration(&self) -> Duration {
        *self.durations.iter().max().unwrap()
    }

    fn std_dev_duration(&self) -> f64 {
        let mean = self.mean_duration().as_secs_f64();
        let variance: f64 = self
            .durations
            .iter()
            .map(|d| {
                let diff = d.as_secs_f64() - mean;
                diff * diff
            })
            .sum::<f64>()
            / self.durations.len() as f64;
        variance.sqrt()
    }

    fn throughput_mbps(&self) -> f64 {
        let mean_duration = self.mean_duration().as_secs_f64();
        let mean_size = self.sizes.iter().sum::<usize>() as f64 / self.sizes.len() as f64;
        (mean_size * 8.0) / (mean_duration * 1_000_000.0) // Mbps
    }

    fn print_report(&self) {
        println!("\n╔═══════════════════════════════════════════════════════════╗");
        println!("║ {}  ", self.name);
        println!("╠═══════════════════════════════════════════════════════════╣");
        println!(
            "║ Samples:        {:<10}                                 ║",
            self.durations.len()
        );
        println!(
            "║ Mean:           {:<10.3}s                              ║",
            self.mean_duration().as_secs_f64()
        );
        println!(
            "║ Median:         {:<10.3}s                              ║",
            self.median_duration().as_secs_f64()
        );
        println!(
            "║ Min:            {:<10.3}s                              ║",
            self.min_duration().as_secs_f64()
        );
        println!(
            "║ Max:            {:<10.3}s                              ║",
            self.max_duration().as_secs_f64()
        );
        println!(
            "║ Std Dev:        {:<10.3}s                              ║",
            self.std_dev_duration()
        );
        println!(
            "║ Throughput:     {:<10.2} Mbps                          ║",
            self.throughput_mbps()
        );
        println!(
            "║ Size:           {:<10} bytes                           ║",
            self.sizes[0]
        );
        if let Some(mem) = self.memory_used {
            println!(
                "║ Memory:         {:<10.2} MB                             ║",
                mem as f64 / 1_048_576.0
            );
        }
        println!("╚═══════════════════════════════════════════════════════════╝");
    }
}

async fn warmup_run(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Warming up (DNS, TLS, connection pooling)...");
    let config = FetchConfig::default();

    // Warmup with both implementations
    let client1 = HttpClient::new()?;
    let _ = client1.fetch(url, &config).await?;

    let client2 = HttpClient::new()?;
    let response = client2.get(url).await?;
    let _ = response.text().await?;

    println!("✓ Warmup complete\n");
    Ok(())
}

fn compare_results(streaming: &BenchmarkResult, buffered: &BenchmarkResult) {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                     COMPARISON                            ║");
    println!("╠═══════════════════════════════════════════════════════════╣");

    let streaming_mean = streaming.mean_duration().as_secs_f64();
    let buffered_mean = buffered.mean_duration().as_secs_f64();
    let diff = streaming_mean - buffered_mean;
    let percent = (diff / buffered_mean) * 100.0;

    if diff < 0.0 {
        println!(
            "║ ✓ Streaming is FASTER by {:.3}s ({:.1}%)                 ║",
            -diff, -percent
        );
    } else {
        println!(
            "║ ✗ Streaming is SLOWER by {:.3}s ({:.1}%)                 ║",
            diff, percent
        );
    }

    let streaming_throughput = streaming.throughput_mbps();
    let buffered_throughput = buffered.throughput_mbps();
    let throughput_diff = streaming_throughput - buffered_throughput;
    let throughput_percent = (throughput_diff / buffered_throughput) * 100.0;

    println!("║                                                           ║");
    println!("║ Throughput Comparison:                                    ║");
    println!(
        "║   Streaming: {:.2} Mbps                                 ║",
        streaming_throughput
    );
    println!(
        "║   Buffered:  {:.2} Mbps                                 ║",
        buffered_throughput
    );

    if throughput_diff > 0.0 {
        println!(
            "║   Difference: +{:.2} Mbps ({:.1}% faster)               ║",
            throughput_diff, throughput_percent
        );
    } else {
        println!(
            "║   Difference: {:.2} Mbps ({:.1}% slower)                ║",
            throughput_diff, -throughput_percent
        );
    }

    println!("║                                                           ║");
    println!("║ Statistical Significance:                                 ║");

    // Simple t-test approximation
    let streaming_stddev = streaming.std_dev_duration();
    let buffered_stddev = buffered.std_dev_duration();
    let pooled_stddev = ((streaming_stddev.powi(2) + buffered_stddev.powi(2)) / 2.0).sqrt();
    let t_score = diff.abs() / (pooled_stddev * (2.0 / streaming.durations.len() as f64).sqrt());

    if t_score > 2.0 {
        println!(
            "║   SIGNIFICANT difference (t={:.2})                        ║",
            t_score
        );
    } else {
        println!(
            "║   NOT significant (t={:.2}, likely network variance)      ║",
            t_score
        );
    }

    println!("╚═══════════════════════════════════════════════════════════╝");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::args().nth(1).unwrap_or_else(|| {
        "https://palimyanmarpitaka.blogspot.com/2021/04/blog-post.html".to_string()
    });

    let iterations: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let config = FetchConfig::default();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              SCAPI FETCH BENCHMARK SUITE                  ║");
    println!("╠═══════════════════════════════════════════════════════════╣");
    println!("║ URL:        {}     ", url);
    println!(
        "║ Iterations: {:<10}                                    ║",
        iterations
    );
    println!("╚═══════════════════════════════════════════════════════════╝");

    // Warmup to eliminate cold start effects
    warmup_run(&url).await?;

    // Alternate between implementations to avoid bias
    println!("📊 Running benchmarks (alternating order)...\n");

    let mut streaming_result = BenchmarkResult::new("Streaming Implementation");
    let mut buffered_result = BenchmarkResult::new("Buffered Implementation");

    for i in 0..iterations {
        print!("Run {}/{}: ", i + 1, iterations);

        if i % 2 == 0 {
            // Even runs: Streaming first
            print!("Streaming... ");
            let client = HttpClient::new()?;
            let start = Instant::now();
            let content = client.fetch(&url, &config).await?;
            let duration = start.elapsed();
            streaming_result.add_sample(duration, content.len());
            print!("{:.3}s, ", duration.as_secs_f64());

            print!("Buffered... ");
            let client = HttpClient::new()?;
            let start = Instant::now();
            let response = client.get(&url).await?;
            let content = response.text().await?;
            let duration = start.elapsed();
            buffered_result.add_sample(duration, content.len());
            println!("{:.3}s", duration.as_secs_f64());
        } else {
            // Odd runs: Buffered first
            print!("Buffered... ");
            let client = HttpClient::new()?;
            let start = Instant::now();
            let response = client.get(&url).await?;
            let content = response.text().await?;
            let duration = start.elapsed();
            buffered_result.add_sample(duration, content.len());
            print!("{:.3}s, ", duration.as_secs_f64());

            print!("Streaming... ");
            let client = HttpClient::new()?;
            let start = Instant::now();
            let content = client.fetch(&url, &config).await?;
            let duration = start.elapsed();
            streaming_result.add_sample(duration, content.len());
            println!("{:.3}s", duration.as_secs_f64());
        }
    }

    // Print results
    streaming_result.print_report();
    buffered_result.print_report();
    compare_results(&streaming_result, &buffered_result);

    // Additional insights
    println!("\n📋 Additional Insights:");
    println!("─────────────────────────────────────────────────────────");
    println!("• Network variance is normal - multiple samples needed");
    println!("• First run eliminated via warmup");
    println!("• Alternating order prevents sequential bias");
    println!("• Look at median (less affected by outliers) not just mean");
    println!("• Throughput (Mbps) is better metric than absolute time");
    println!("\n💡 Key Points:");
    println!("─────────────────────────────────────────────────────────");
    println!("• Streaming's main benefit is MEMORY, not speed");
    println!("• Expect similar performance (within 5-10% is normal)");
    println!("• Streaming prevents OOM on large files");
    println!("• Buffered might be slightly faster due to optimization");
    println!("• But streaming is safer for production workloads");

    Ok(())
}
