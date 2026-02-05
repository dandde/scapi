use futures::stream::StreamExt;
use scapi::domain::fetch::config::FetchConfig;
use scapi::infra::http::HttpClient;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://palimyanmarpitaka.blogspot.com/2021/04/blog-post.html";
    let config = FetchConfig::default();
    let _client = HttpClient::new()?;

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
        "Buffered (String):   {:?} (Size: {} bytes)",
        duration_new,
        content_new.len()
    );

    // 2. True Streaming (Chunk by Chunk)
    let client2 = HttpClient::new()?;
    let start = Instant::now();
    let mut stream_result = client2.streaming().fetch_stream(url, &config).await?;
    let ttfb = start.elapsed();

    let mut total_bytes = 0;
    while let Some(chunk) = stream_result.stream.next().await {
        let chunk = chunk?;
        total_bytes += chunk.len();
    }
    let duration_streaming = start.elapsed();

    println!(
        "True Streaming:      {:?} (TTFB: {:?}, Size: {} bytes)",
        duration_streaming, ttfb, total_bytes
    );

    // 2. Previous Implementation (Simulated via client.get().text())
    let client2 = HttpClient::new()?;
    let start = Instant::now();
    let response = client2.get(url).await?;
    let content_old = response.text().await?;
    let duration_old = start.elapsed();
    println!(
        "Reqwest (Get+Text):  {:?} (Size: {} bytes)",
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
