//https://zh.practice.rs/pattern-match/patterns.html
//8.2.1
//
// fn main() {
//     match_number(45);
// }
// fn match_number(n: i32) {
//     match n {
//         // 匹配一个单独的值
//         1 => println!("One!"),
//         // 使用 `|` 填空，不要使用 `..` 或 `..=`
//         2..=5 => println!("match 2 -> 5"),
//         // 匹配一个闭区间的数值序列
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match 11 -> +infinite")
//         }
//     }
// }

//8.2.2
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     // 填空，让 p 匹配第二个分支
//     let p = Point { x: 0, y: 10 };
//
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // 第二个分支
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }

//8.2.3
// 修复错误
// enum Message {
//     Hello { id: i32 },
// }
//
// fn main() {
//     let msg = Message::Hello { id: 5 };
//
//     match msg {
//         Message::Hello { id: id@ 3..=7 } => println!("id 值的范围在 [3, 7] 之间: {}", id),
//         Message::Hello {
//             id: newid @ (10 | 11 | 12),
//         } => {
//             println!("id 值的范围在 [10, 12] 之间: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }

//8.2.4
// 填空让代码工作，必须使用 `split`
// fn main() {
//     let num = Some(4);
//     let split = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
// }

//8.2.5
// 填空，让代码工作
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//
//     match numbers {
//         (first,..,last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }
// }

//8.2.6
// 修复错误，尽量少地修改代码
// 不要移除任何代码行
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!") 
    }
}
