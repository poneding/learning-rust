// Preludes 模式，公开你想要的私有模块中的类型或者私有类型
mod struct4_1; // 私有的

pub use struct4_1::Struct4_1; // 将私有的公开，那么外部可以通过 mod4::Struct4_1 访问

pub struct Struct4 {}
