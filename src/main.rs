use hellorust::{cache, httpreq};

fn main() {
    cache::test();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(httpreq::get());
    println!("done!");
}