//! TODO: get the code to compile by **re-ordering** the statements
//! TODO: 通过**重新排序** `example` 函数中的语句来使代码编译通过
//!  in the `example` function. You're not allowed to change the
//!  你不允许更改 `spawner` 函数或 `example` 中每行语句的功能。
//!  `spawner` function nor what each line does in `example`.
//!  如果需要，你可以将现有语句包裹在块 `{}` 中。
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
