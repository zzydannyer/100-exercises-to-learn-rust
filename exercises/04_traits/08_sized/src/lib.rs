pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // 尝试获取 str（或任何其他 DST）的大小
    // via `std::mem::size_of` will result in a compile-time error.
    // 通过 `std::mem::size_of` 会导致编译时错误。
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // TODO: 注释掉下面这行，然后继续下一个练习。
    std::mem::size_of::<str>();
}
