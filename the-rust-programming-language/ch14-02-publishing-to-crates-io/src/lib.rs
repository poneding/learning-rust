// 发布 crate 到 crates.io

// cargo doc --open

// crate 注释 //！

//! # ch14_02_publishing_to_crates_io
//!
//! `ch14_02_publishing_to_crates_io` 提供一些列运行基础计算的工具

// 函数注释 ///

/// 两数相加
///
/// # 示例
///
/// ```
/// let result = ch14_02_publishing_to_crates_io::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 重新导出

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
        // --snip--
    }
}
