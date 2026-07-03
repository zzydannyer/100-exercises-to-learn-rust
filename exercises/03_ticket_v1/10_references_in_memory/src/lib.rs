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
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
