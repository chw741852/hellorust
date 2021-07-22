use cached::proc_macro::cached;
use std::thread::sleep;
use std::time::Duration;

#[cached(size = 50)]
pub fn keyed(a: String, b: String) -> usize {
    let size = a.len() + b.len();
    sleep(Duration::new(2, 0));
    size
}