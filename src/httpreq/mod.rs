use std::collections::HashMap;

pub async fn get() {
    let res = reqwest::get("https://httpbin.org/ip").await;
    if res.is_ok() {
        let res = res.unwrap();
        println!("{}", res.status());
        println!("{:?}", res.json::<HashMap<String, String>>().await.unwrap());
    } else {
        println!("Failed to get");
    }
}