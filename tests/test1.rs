#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        // тут пока ниче не происходит
        assert!(1 == 1);
    }

    #[test]
    fn test2() {
        // а этот падает
        assert_eq!(1, 2);
    }
}
