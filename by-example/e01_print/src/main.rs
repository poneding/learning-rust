mod print;
mod print_debug;
mod print_display;
mod print_fmt;

// fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用，标准库中类型都实现了该 trait，自定义类型可以使用 derive 属性自动创建所需的实现。
// fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本，标准库中类型大多也提供了实现，自定义类型需要手动实现。

fn main() {
    print::run_print();
    print_debug::run_print_debug();
    print_display::run_print_display();
    print_fmt::run_print_fmt();
}
