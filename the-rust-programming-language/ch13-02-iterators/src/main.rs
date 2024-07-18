fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // map
    let v2_iter = v1.iter();
    let map_result = v2_iter.map(|x| x + 1);
    // for val in map_result.collect::<Vec<i32>>() {
    for val in map_result {
        println!("Got map_result item: {val}");
    }

    // filter
    let v3_iter = v1.iter();
    let filter_result = v3_iter.filter(|&x| x % 2 == 1);
    for val in filter_result {
        println!("Got filter_result item: {val}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum(); // sum() 会获取 v1_iter 的所有权，之后无法使用 v1_iter
        assert_eq!(total, 6);
    }
}
