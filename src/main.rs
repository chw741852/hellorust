use std::thread::sleep;
use std::time::{Duration, Instant};

use cached::proc_macro::cached;
use hellorust::{cache1, cache2};

// use cached::SizedCache;

// cached_key! {
//     SLOW_FN: UnboundCache<u32, String> = SizedCache::with_size(50);
//     Timeout = 3;
//     fn slow_fn(n: u32) -> String = {
//         if n == 0 { return "done".to_string(); }
//         sleep(Duration::new(1, 0));
//         slow_fn(n-1)
//     }
// }

#[cached(size = 50)]
fn slow_fn(n: u32) -> String {
    if n == 0 { return "done".to_string(); }
    sleep(Duration::new(1, 0));
    slow_fn(n-1)
}

fn main() {
    let now = Instant::now();
    let a1 = cache1::keyed("Hello".into(), "World".into());
    println!("a1 is {} elapsed {}", a1, now.elapsed().as_secs());

    let now = Instant::now();
    let a2 = cache2::keyed("Hello".into(), "World".into());
    println!("a2 is {} elapsed {}", a2, now.elapsed().as_secs());


    let now = Instant::now();
    let a1 = cache1::keyed("Hello".into(), "World".into());
    println!("a1 is {} elapsed {}", a1, now.elapsed().as_secs());

    let now = Instant::now();
    let a2 = cache2::keyed("Hello".into(), "World".into());
    println!("a2 is {} elapsed {}", a2, now.elapsed().as_secs());

    cache1::keys();
    cache1::invalidate("Hello".into(), "World".into());

    let now = Instant::now();
    let a1 = cache1::keyed("Hello".into(), "World".into());
    println!("a1 is {} elapsed {}", a1, now.elapsed().as_secs());

    let now = Instant::now();
    let a2 = cache2::keyed("Hello".into(), "World".into());
    println!("a2 is {} elapsed {}", a2, now.elapsed().as_secs());

    // println!("Initial run...");
    // let now = Instant::now();
    // let _ = slow_fn(10);
    // println!("Elapsed: {}\n", now.elapsed().as_secs());

    // println!("Cached run...");
    // let now = Instant::now();
    // let _ = slow_fn(8);
    // println!("Elapsed: {}\n", now.elapsed().as_secs());

    // Inspect the cache
    // {
    //     use cached::Cached; // must be in scope to access cache

    //     println!(" ** Cache info **");
    //     let mut cache = SLOW_FN.lock().unwrap();
    //     assert_eq!(cache.cache_hits().unwrap(), 1);
    //     println!("hits=1 -> {:?}", cache.cache_hits().unwrap() == 1);
    //     assert_eq!(cache.cache_misses().unwrap(), 11);
    //     println!("misses=11 -> {:?}", cache.cache_misses().unwrap() == 11);
    //     // make sure the cache-lock is dropped

    //     let key = 8;
    //     let res = cache.cache_remove(&key);
    //     match res {
    //         Some(res) => println!("remove success: {}", res),
    //         None => println!("failed to remove cache")
    //     }
    // }

    println!("done!");
}