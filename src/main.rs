use hellorust::{cache, httpreq, chrono_test};

fn main() {
    // cache
    // cache::test();

    // httpreq
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(httpreq::get());

    // chrono
    chrono_test::current_time_slice();

    println!("done!");

    let foo = Foo {
        name: "jack".into(),
        age: 10,
    };

    let name = foo.name;
    println!("name is {}", name);
    // println!("{}", foo.name);    // foo.name has moved
    println!("age is {}", foo.age);

    let hour = 11;
    for i in hour..24 {
        println!("{}", i);
    }

    println!("{}", 6 % 10);
}

struct Foo {
    name: String,
    age: usize
}