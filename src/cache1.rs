use cached::proc_macro::cached;
use std::thread::sleep;
use std::time::Duration;

#[cached(size = 50)]
pub fn keyed(a: String, b: String) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(1, 0));
    size
}

pub fn keys() {
    let cache = KEYED.lock().unwrap();
    cache.key_order().for_each(|key| println!("{:?}", key));
}

pub fn invalidate(a: String, b: String) {
    use cached::Cached;

    let mut cache = KEYED.lock().unwrap();
    let key = (a, b);
    cache.cache_remove(&key);
}