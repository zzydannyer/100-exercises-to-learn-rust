// TODO: based on what you learned in this section, replace `todo!()` with
// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为转换后的正确值。
//  the correct value after the conversion.
// TODO: 根据你在本节中学到的知识，将 `todo!()` 替换为转换后的正确值。

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // 编译器足够聪明，知道值 255 无法放入 i8，因此会发出硬错误。
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // 无法放入 i8，因此会发出硬错误。我们有意禁用了
        // this guardrail to make this (bad) conversion possible.
        // 我们有意禁用了这个保护机制，以使这个（不好的）转换成为可能。
        // The compiler is only able to pick on this because the value is a
        // 编译器之所以能发现这个，是因为该值是一个字面量。
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // 字面量。如果我们使用变量，编译器将无法
        // catch this at compile time.
        // 如果我们使用变量，编译器将无法在编译时捕捉到这一点。
        // 255 是 u8 范围的字面量，强制转 i8 会变成-1
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // You could solve this by using exactly the same expression as above,
        // 你可以通过使用与上面完全相同的表达式来解决这个问题，
        // but that would defeat the purpose of the exercise. Instead, use a genuine
        // 但那会违背练习的目的。相反，请使用一个在转换为 `u8` 时等同于 `255` 的真实 `i8` 值。
        // `i8` value that is equivalent to `255` when converted to `u8`.
        // 但那会违背练习的目的。相反，请使用一个在转换为 `u8` 时等同于 `255` 的真实 `i8` 值。
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
