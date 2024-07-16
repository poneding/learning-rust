pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 使用 #[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if add(1, 1) == 2 {
            Ok(())
        } else {
            Err(String::from("1 + 1 没有得到结果 2"))
        }
    }

    #[test]
    #[should_panic] // 应该 panic
    fn should_panic_test() {
        panic!("oops~");
    }

    #[test]
    #[should_panic(expected = "oops~~~")] // panic 信息必须包含 expected 的文本内容
    fn should_panic_with_reason_test() {
        panic!("oops~~~")
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(1, 1), 2);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

#[allow(unused)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.width > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
}
