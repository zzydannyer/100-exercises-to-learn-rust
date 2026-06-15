pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为相应类型的正确**栈大小**。
//  the correct **stack size** for the respective type.
// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为相应类型的正确**栈大小**。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // 这是一个棘手的问题！
        // The "intuitive" answer happens to be the correct answer this time,
        // "直观"的答案这次碰巧是正确的答案，
        // but, in general, the memory layout of structs is a more complex topic.
        // 但一般来说，结构体的内存布局是一个更复杂的话题。
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // 如果你感到好奇，请查看《Rust 参考手册》中的"Type layout"一节
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        // https://doc.rust-lang.org/reference/type-layout.html 了解更多信息。
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
