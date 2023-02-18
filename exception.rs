
// 程序中一般会出现两种错误：可恢复错误和不可恢复错误。
// 可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。
// 但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。
// Rust中没有Exception 对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。



fn main() {

}