use scapi::domain::fetch::config::FetchConfig;
use scapi::infra::http::HttpClient;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://palimyanmarpitaka.blogspot.com/2021/04/blog-post.html";
    let config = FetchConfig::default();
    let client = HttpClient::new()?;

    println!("Benchmarking fetch for URL: {}", url);
    println!("--------------------------------------------------");

    // 1. New Implementation (Streaming)
    // We run new first to warm up connection/dns if possible, or maybe we should run separate clients?
    // Let's create separate clients to be fair.
    let client1 = HttpClient::new()?;
    let start = Instant::now();
    let content_new = client1.fetch(url, &config).await?;
    let duration_new = start.elapsed();
    println!(
        "New (Streaming):     {:?} (Size: {} bytes)",
        duration_new,
        content_new.len()
    );

    // 2. Previous Implementation (Simulated via client.get().text())
    let client2 = HttpClient::new()?;
    let start = Instant::now();
    let response = client2.get(url).await?;
    let content_old = response.text().await?;
    let duration_old = start.elapsed();
    println!(
        "Previous (Buffered): {:?} (Size: {} bytes)",
        duration_old,
        content_old.len()
    );

    println!("--------------------------------------------------");

    if duration_new < duration_old {
        println!("Streaming was FASTER by {:?}", duration_old - duration_new);
    } else {
        println!("Streaming was SLOWER by {:?}", duration_new - duration_old);
    }

    Ok(())
}
