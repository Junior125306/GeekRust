use std::env::args;
use std::fs;

// fn fib_loop(n: u8) {
//     let mut a = 1;
//     let mut b = 1;
//     let mut i = 2u8;
//     loop {
//         let c = a + b;
//         a = b;
//         b = c;
//         i += 1;

//         println!("next val is {}", b);
//         if i >= n {
//             break;
//         }
//     }
// }

// fn fib_while(n: u8) {
//     let (mut a, mut b, mut i) = (1, 1, 2);
//     while i < n {
//         let c = a + b;
//         a = b;
//         b = c;
//         i += 1;
//         println!("next val is {}", b);
//     }
// }

// fn fib_for(n: u8) {
//     let (mut a, mut b) = (1, 1);
//     for _i in 2..n {
//         let c = a + b;
//         a = b;
//         b = c;
//         println!("next val is {}", b);
//     }
// }

// #[derive(Debug)]
// enum Gender {
//     Unspecified = 0,
//     Female = 1,
//     Male = 2,
// }

// #[derive(Debug, Copy, Clone)]
// struct UserId(u64);

// #[derive(Debug, Copy, Clone)]
// struct TopicId(u64);

// #[derive(Debug)]
// struct User {
//     id: UserId,
//     name: String,
//     gender: Gender,
// }

// #[derive(Debug)]
// struct Topic {
//     id: TopicId,
//     name: String,
//     owner: UserId,
// }

// #[derive(Debug)]
// enum Event {
//     Join((UserId, TopicId)),
//     Leave((UserId, TopicId)),
//     Message((UserId, TopicId, String)),
// }

fn main() {
    // for arg in std::env::args() {
    //     println!("{}", arg);
    // }
    // let n = 10;
    // fib_loop(n);
    // fib_while(n);
    // fib_for(n);
    // let alice = User {
    //     id: UserId(1),
    //     name: "Alice".into(),
    //     gender: Gender::Female,
    // };
    // let bob = User {
    //     id: UserId(2),
    //     name: "Bob".into(),
    //     gender: Gender::Male,
    // };

    // let topic = Topic {
    //     id: TopicId(1),
    //     name: "Rust".into(),
    //     owner: alice.id,
    // };
    // let event1 = Event::Join((alice.id, topic.id));
    // let event2 = Event::Join((bob.id, topic.id));
    // let event3 = Event::Message((bob.id, topic.id, "Hello".into()));
    // println!(
    //     "event1 is {:?},event2 is {:?},event3: {:?}",
    //     event1, event2, event3
    // );
    // let is_pi = pi();
    // let is_unit = unit();
    // let is_unit2 = {
    //     pi();
    // };
    // 获取第一个std参数
    let url = args().nth(1).expect("url is required");
    let output = &args().nth(2).expect("output is required");

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
    // println!("pi is {}", is_pi);
    // println!("unit is {:?}", is_unit);
    // println!("unit is {:?}", is_unit2);
}

// 此时返回 unit 类型
// fn unit() {
//     3.1;
// }

// 此时返回3.14 f64类型
// fn pi() -> f64 {
//     3.14
// }
