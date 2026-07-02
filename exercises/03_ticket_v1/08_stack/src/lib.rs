// TODO: based on what you learned in this section, replace `todo!()` with
// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为相应类型的正确**栈大小**。
//  the correct **stack size** for the respective type.
// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为相应类型的正确**栈大小**。
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), 1);
    }
}
