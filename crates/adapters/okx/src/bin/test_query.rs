use nautilus_okx::http::query::GetCandlesticksParamsBuilder;

fn main() {
    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.limit(100);

    let params = builder.build().unwrap();
    let query = serde_urlencoded::to_string(&params).unwrap();

    println!("Query without after/before: {query}");

    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");
    builder.after_ms(1234567890);
    builder.limit(100);

    let params = builder.build().unwrap();
    let query = serde_urlencoded::to_string(&params).unwrap();

    println!("Query with after: {query}");

    let mut builder = GetCandlesticksParamsBuilder::default();
    builder.inst_id("BTC-USDT-SWAP");
    builder.bar("1m");

    let params = builder.build().unwrap();
    let query = serde_urlencoded::to_string(&params).unwrap();

    println!("Query without limit: {query}");
}
