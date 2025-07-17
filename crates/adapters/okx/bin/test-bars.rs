// Test script to debug OKX candlestick query parameter issues

use chrono::Utc;
use nautilus_okx::http::query::GetCandlesticksParamsBuilder;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    // Test 1: No start/end parameters (should work according to feedback)
    println!("=== Test 1: No start/end parameters ===");
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.limit(100);

    let params = builder.build()?;
    let query = serde_urlencoded::to_string(&params)?;
    println!("Query string: {query}");
    println!("Expected result: 100 bars (or default)");
    println!();

    // Test 2: With start/end parameters (problematic according to feedback)
    println!("=== Test 2: With start/end parameters ===");
    let now = Utc::now();
    let start_time = now - chrono::Duration::hours(2);
    let end_time = now - chrono::Duration::hours(1);

    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.after_ms(start_time.timestamp_millis());
    builder.before_ms(end_time.timestamp_millis());
    builder.limit(100);

    match builder.build() {
        Ok(params) => {
            let query = serde_urlencoded::to_string(&params)?;
            println!("Query string: {query}");
            println!("Expected result: Zero bars (problem case)");
        }
        Err(e) => {
            println!("Build error: {e}");
            println!("This shows our validation is working - both cursors rejected");
        }
    }
    println!();

    // Test 3: With only start parameter (after)
    println!("=== Test 3: With only start parameter (after) ===");
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.after_ms(start_time.timestamp_millis());
    builder.limit(100);

    let params = builder.build()?;
    let query = serde_urlencoded::to_string(&params)?;
    println!("Query string: {query}");
    println!("Expected result: Bars after start time");
    println!();

    // Test 4: With only end parameter (before)
    println!("=== Test 4: With only end parameter (before) ===");
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.before_ms(end_time.timestamp_millis());
    builder.limit(100);

    let params = builder.build()?;
    let query = serde_urlencoded::to_string(&params)?;
    println!("Query string: {query}");
    println!("Expected result: Bars before end time");
    println!();

    // Test 5: Test maximum limit of 300
    println!("=== Test 5: Test maximum limit of 300 ===");
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.limit(300);

    let params = builder.build()?;
    let query = serde_urlencoded::to_string(&params)?;
    println!("Query string: {query}");
    println!("Should work with limit=300 for regular endpoint");
    println!();

    // Test 6: Test limit > 300 (should fail)
    println!("=== Test 6: Test limit > 300 (should fail) ===");
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.limit(301);

    match builder.build() {
        Ok(params) => {
            let query = serde_urlencoded::to_string(&params)?;
            println!("Query string: {query}");
            println!("ERROR: This should have failed!");
        }
        Err(e) => {
            println!("Build error: {e}");
            println!("This is expected - limit > 300 rejected");
        }
    }
    println!();

    // Test 7: Verify None omission
    println!("=== Test 7: Verify None omission ===");
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    // No after, before, or limit set

    let params = builder.build()?;
    let query = serde_urlencoded::to_string(&params)?;
    println!("Query string: {query}");
    println!("Should only have instId and bar parameters");
    println!();

    Ok(())
}
