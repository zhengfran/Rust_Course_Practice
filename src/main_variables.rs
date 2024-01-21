// https://zh.practice.rs/variables.html
// 1.修复下面代码的错误并尽可能少的修改
 // fn main() {
 //     let x: i32 = 0; // 未初始化，但被使用
 //     let _y: i32; // 未初始化，也未被使用
 //     println!("x is equal to {}", x); 
 // }


// 2.完形填空，让代码编译
// fn main() {
//     let mut x = 1;
//     x += 2; 
//     
//     println!("x = {}", x); 
// }


// 3.修复下面代码的错误并使用尽可能少的改变
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}", x); 
// }

// 4.修复错误
// fn main() {
//     let x = "hello";
//     println!("{}, world", x); 
// }
//
// fn define_x() {
//     let x = "hello";
// }

// 5.只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }
//
//     assert_eq!(x, 5);
//
//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// 6.修改一行代码以通过编译
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let mut x = x; 
//     x += 3;
//
//
//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!"; 
// }

// 7. Use 2 ways to fix the compiler warnings
// compiler warning: unused variable: `x`
// fn main() {
//     // let x = 1; 
//     // println!("X is {}", x);
//     let _x = 1; 
// }

// 8. 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// 9. 修复下面代码的错误并尽可能少的修改
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3, 2]);
} 
