use std::time::Instant;

pub mod cache1;
pub mod cache2;

pub fn test() {
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
}