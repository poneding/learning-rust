use adder::add;

// cargo test 将会运行集成测试
// 指定运行集成测试
// cargo test --test integration-test
// integration-test 为 tests 目录下的文件名

mod common; // 调用测试之前的初始化函数

#[test]
fn it_adds() {
    common::setup();
    assert_eq!(add(1, 1), 2)
}
