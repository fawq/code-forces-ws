fn dummy_result() -> i32 {
    24
}

fn main() {
    println!("Hello, world!");
    dummy_result();
}

#[cfg(test)]
mod tests {
    use crate::dummy_result;

    #[test]
    fn test_assert() {
        let result = dummy_result() + 2;
        assert_eq!(result, 26);
    }
}
