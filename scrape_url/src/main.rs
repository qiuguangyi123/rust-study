extern crate qgy;
use std::fs;
use std::env;
// 拉取内容生产 md 文件
// fn main() {
//     // let url: &str = "https://www.rust-lang.org";
//     let url = env::args().nth(1).expect("Argument must be a string");
//     println!("url:{}", url);
//     let output: &str = "rust.md";

//     println!("Fetching url: {}", url);
//     let body = reqwest::blocking::get(url).unwrap().text().unwrap();

//     println!("Converting html to markdown...");
//     let md = html2md::parse_html(&body);

//     fs::write(output,md.as_bytes()).unwrap();
//     println!("Converted markdown has been saved in {}.", output);
// }

// 函数的返回值
// fn pi() -> f64 { 3.1415926}
// fn not_pi() { 3.1415926;}

// fn main() { let is_pi = pi(); let is_unit1 = not_pi(); let is_unit2 = { pi(); }; println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);}

// 数据结构以及接口
// #[derive(Debug)]
// enum Gender {
//   Unspecified = 0,
//   Female = 1,
//   Male = 2,
// }

// #[derive(Debug, Copy, Clone)]
// struct UserId(u64);

// #[derive(Debug, Copy, Clone)]
// struct TopicId(u64);

// #[derive(Debug)]
// struct User {
//   id: UserId,
//   name: String,
//   gender: Gender,
// }

// #[derive(Debug)]
// struct Topic {
//   id: TopicId,
//   name: String,
//   owner: UserId,
// }

// // 定义聊天室中可能发生的事件
// #[derive(Debug)]
// enum Event {
//   Join((UserId, TopicId)),
//   Leave((UserId, TopicId)),
//   Message((UserId, TopicId, String)),
// }

// fn main() {
//     let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
//     let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

//     let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
//     let event1 = Event::Join((alice.id, topic.id));
//     let event2 = Event::Join((bob.id, topic.id));
//     let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

//     println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
// }

// 循环

// fn loop_num(a:u8,b:u8)->[u8;2]{
//     let mut c:u8;
//     c = a+b;
//     [b,c]
// }

// fn fib_loop(n:u8){
//   let mut a = 1;
//   let mut b = 1;
//   let mut i = 2u8;

//   loop{
//     [a,b] = loop_num(a,b);
//     i+=1;
//     println!("{}",b);
//     if i>=n{
//       break;
//     }
//   }
// }

// fn fib_while(n:u8){
//   let mut a = 1;
//   let mut b =1;
//   let mut i = 2u8;
//   while i<n{
//     [a,b] = loop_num(a,b);
//     println!("{}",b);
//     i += 1;
//   }
// }

// fn fib_for(n:u8){
//  let (mut a,mut b) = (1,1);
//  for index in 3..=n{
//   [a,b] = loop_num(a,b);
//   println!("{},index:{}",b,index);
//  }
// }

// fn main(){
//   fib_loop(10);
//   fib_while(10);
//   fib_for(10)
// //   let arr = [1,2,3,4,5];
// //   assert_eq!(arr[..], [1,2,3,4,5]);
// //   // assert_eq!(arr[1..], [1,2,3,4,5]);
// //   assert_eq!(arr[0..arr.len()-1], [1,2,3,4,5])
// }

// 模式匹配
// #[derive(Debug, Copy, Clone)]
// struct UserId(u64);

// #[derive(Debug, Copy, Clone)]
// struct TopicId(u64);

// // 定义聊天室中可能发生的事件
// #[derive(Debug)]
// enum Event {
//     Join((UserId, TopicId)),
//     Leave((UserId, TopicId)),
//     Message((UserId, TopicId, String)),
// }

// fn process_env(event: Event) {
//     // match event {
//     //     Event::Join((user, _tid)) => println!("user {:?} join the chat room", user),
//     //     Event::Leave((user, _tid)) => println!("user {:?} leave the chat room", user),
//     //     Event::Message((_, _, msg)) => println!("broadcast:{}", msg),
//     // }
//     if let Event::Join((user, _tid)) = event {
//         println!("user {:?}", user);
//     }
// }

// fn main() {
//     println!("number:{}", qgy::add(1, 2))
//     // process_env(Event::Join((UserId(1), TopicId(1))));
//     // process_env(Event::Leave((UserId(2), TopicId(1))));
//     // process_env(Event::Message((
//     //     UserId(1),
//     //     TopicId(1),
//     //     String::from("Hello, world!"),
//     // )));
// }

// 参数获取
fn main() {
    // let args: Vec<String> = env::args().collect();
    // for (arg,i) in args.iter().enumerate() {
    //     println!("{},{}", arg,i);
    // }
    for arg in env::args() {
        println!("{},111", arg);
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_add(){
        assert_eq!(qgy::add(1,2),3)
    }
}