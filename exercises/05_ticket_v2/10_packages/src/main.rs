// This is a `main.rs` file, therefore `cargo` interprets this as the root of a binary target.
// 这是一个 `main.rs` 文件，因此 `cargo` 将其解释为二进制目标的根文件。

// TODO: fix this broken import. Create a new library target in the `src` directory.
// TODO: 修复这个损坏的导入。在 `src` 目录中创建一个新的库目标。
//   The library target should expose a public function named `hello_world` that takes no arguments
//   该库目标应暴露一个名为 `hello_world` 的公有函数，该函数不接受参数
//   and returns nothing.
//   且不返回任何内容。
mod packages;
use packages::hello_world;

// This is the entrypoint of the binary.
// 这是二进制文件的入口点。
fn main() {
    hello_world();
}
