use chrono::Local;

pub fn current_time_slice() {
    let now = Local::now();
    let today_start = Local::today().and_hms(0, 0, 0);
    println!("{}", now);
    println!("{}", today_start);
    println!("current time slice is {}", (now.timestamp() - today_start.timestamp()) / 60 / 10);
}