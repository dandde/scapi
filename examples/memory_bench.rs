use scapi::domain::fetch::config::FetchConfig;
use scapi::infra::http::HttpClient;
use std::time::Instant;

#[cfg(target_os = "linux")]
fn get_memory_usage() -> usize {
    use std::fs;
    if let Ok(content) = fs::read_to_string("/proc/self/statm") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if let Some(resident) = parts.get(1) {
            if let Ok(pages) = resident.parse::<usize>() {
                return pages * 4096;
            }
        }
    }
    0
}

#[cfg(target_os = "macos")]
fn get_memory_usage() -> usize {
    use std::process::Command;

    let pid = std::process::id();
    let output = Command::new("ps")
        .args(&["-o", "rss=", "-p", &pid.to_string()])
        .output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(kb) = output_str.trim().parse::<usize>() {
            return kb * 1024; // KB to Bytes
        }
    }
    0
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn get_memory_usage() -> usize {
    0 // Not implemented
}

// Rename original profiling to "Safe Buffering"
async fn profile_safe_buffering(
    url: &str,
    config: &FetchConfig,
) -> Result<(usize, usize, f64), Box<dyn std::error::Error>> {
    let client = HttpClient::new()?;

    // Force GC/Cleanup
    std::hint::black_box(vec![0u8; 1024 * 1024]);
    let mem_before = get_memory_usage();
    let start = Instant::now();

    // specific scapi::fetch which buffers safely
    let content = client.fetch(url, config).await?;
    let size = content.len();

    let mem_after = get_memory_usage();
    let duration = start.elapsed().as_secs_f64();
    drop(content);
    let mem_used = mem_after.saturating_sub(mem_before);

    Ok((size, mem_used, duration))
}

async fn profile_raw_buffered(
    url: &str,
    _config: &FetchConfig,
) -> Result<(usize, usize, f64), Box<dyn std::error::Error>> {
    let client = HttpClient::new()?;

    std::hint::black_box(vec![0u8; 1024 * 1024]);
    let mem_before = get_memory_usage();
    let start = Instant::now();

    let response = client.get(url).await?;
    let content = response.text().await?;
    let size = content.len();

    let mem_after = get_memory_usage();
    let duration = start.elapsed().as_secs_f64();
    drop(content);
    let mem_used = mem_after.saturating_sub(mem_before);

    Ok((size, mem_used, duration))
}

async fn profile_true_streaming(
    url: &str,
    config: &FetchConfig,
) -> Result<(usize, usize, f64), Box<dyn std::error::Error>> {
    let client = HttpClient::new()?;

    std::hint::black_box(vec![0u8; 1024 * 1024]);
    let mem_before = get_memory_usage();
    let start = Instant::now();

    // Use fetch_to_writer with a sink (discarding writer)
    let mut sink = tokio::io::sink();
    let metadata = client
        .streaming()
        .fetch_to_writer(url, config, &mut sink)
        .await?;
    let size = metadata.length;

    let mem_after = get_memory_usage();
    let duration = start.elapsed().as_secs_f64();
    let mem_used = mem_after.saturating_sub(mem_before);

    Ok((size, mem_used, duration))
}

fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_urls = vec![
        (
            "~1MB (React DOM)",
            "https://cdnjs.cloudflare.com/ajax/libs/react-dom/18.2.0/umd/react-dom.development.js",
        ),
        (
            "~10MB (TypeScript)",
            "https://cdnjs.cloudflare.com/ajax/libs/typescript/5.3.3/typescript.js",
        ),
    ];

    let config = FetchConfig::default();

    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                    SCAPI MEMORY PROFILING BENCHMARK                       ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║ Three modes tested:                                                       ║");
    println!("║ 1. Safe Buffering: scapi::fetch (Streams into a String with limits)       ║");
    println!("║ 2. Raw Buffering:  reqwest::text (Directly into String)                   ║");
    println!("║ 3. True Streaming: pipe to sink (Process chunks without holding)          ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");

    for (label, url) in test_urls {
        println!("\n╔═══════════════════════════════════════════════════════════════════════════╗");
        println!("║ Testing: {:<56} ║", label);
        println!("╠═══════════════════════════════════════════════════════════════════════════╣");

        // Warmup
        let client = HttpClient::new()?;
        let _ = client.fetch(url, &config).await?;

        // 1. Safe Buffering
        print!("║ Safe Buffer... ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let (size_s, mem_s, time_s) = profile_safe_buffering(url, &config).await?;
        println!(
            "{:<15} {} in {:.3}s         ",
            format_bytes(mem_s),
            format_bytes(size_s),
            time_s
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 2. Raw Buffered
        print!("║ Raw Buffer...  ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let (size_b, mem_b, time_b) = profile_raw_buffered(url, &config).await?;
        println!(
            "{:<15} {} in {:.3}s         ",
            format_bytes(mem_b),
            format_bytes(size_b),
            time_b
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // 3. True Streaming
        print!("║ True Stream... ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let (size_t, mem_t, time_t) = profile_true_streaming(url, &config).await?;
        println!(
            "{:<15} {} in {:.3}s         ",
            format_bytes(mem_t),
            format_bytes(size_t),
            time_t
        );

        println!("╠═══════════════════════════════════════════════════════════════════════════╣");
        println!("║ Analysis:                                                                 ║");

        // Compare Raw Buffer vs True Stream
        if mem_b > 0 && mem_t > 0 {
            let reduction = ((mem_b as i64 - mem_t as i64) as f64 / mem_b as f64) * 100.0;
            if reduction > 0.0 {
                println!(
                    "║   ✓ True Streaming saves {:.1}% memory ({}) vs Raw Buffer",
                    reduction,
                    format_bytes(mem_b.saturating_sub(mem_t))
                );
            } else {
                println!("║   ⚠ No memory savings detected (OS noise?)");
            }
        }

        // Explain Safe Buffer Overhead
        if mem_s > mem_b {
            let overhead = ((mem_s - mem_b) as f64 / mem_b as f64) * 100.0;
            println!(
                "║   ℹ Safe Buffering uses {:.1}% MORE memory than Raw (Safety Overhead)",
                overhead
            );
        }

        println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    }
    Ok(())
}
