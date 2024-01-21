//https://zh.practice.rs/basic-types/statements-expressions.html
// 4.3.0 example
// fn main() {
//     let x = 5u32;
//
//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;
//
//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };
//
//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };
//
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// 4.3.1 使用两种方法让代码工作起来
// 1. remove = in x +=2
// 2. change 3 to ()
// fn main() {
//    let v = {
//        let mut x = 1;
//        x += 2
//    };
//
//    assert_eq!(v, ());
// }

// 4.3.2
fn main() {
   let v = {
     let x = 3;
     x
   };

   assert!(v == 3);
}

// 4.3.3
// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }
//
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

