// TODO: You don't actually have to change anything in the library itself!
// TODO: 你实际上不需要改动库本身！
//  We mostly had to **remove** code (the client type, the launch function, the command enum)
//  我们主要需要**删除**不再需要的代码（客户端类型、启动函数、命令枚举）
//  that's no longer necessary.
//  Fix the `todo!()` in the testing code and see how the new design can be used.
//  修复测试代码中的 `todo!()`，看看如何使用新设计。

pub mod data;
pub mod store;
