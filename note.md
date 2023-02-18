# cargo

## 基本命令

Cargo 是 Rust 的构建系统和包管理器, 常用命令有cargo build, cargo run
cargo clippy: 类似eslint，lint工具检查代码可以优化的地方
cargo fmt: 类似go fmt，代码格式化
cargo tree: 查看第三方库的版本和依赖关系
cargo bench: 运行benchmark(基准测试,性能测试)
cargo udeps(第三方): 检查项目中未使用的依赖
cargo build/run --release 使用 release 编译会比默认的 debug 编译性能提升 10 倍以上，但是 release 缺点是编译速度较慢，而且不会显示 panic backtrace 的具体行号