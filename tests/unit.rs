#[path = "../src/util/formater.rs"]
mod formater;
#[cfg(test)]
mod unit {
    #[test]
    fn test_formater() {
        let source = "(наружу \"What is your name?\\n\")\n(пост name (in)) (\n наружу \"Hello, \" name \"!\\n\")\n\n";
        let expect = "(наружу \"What is your name?\\n\")\n(пост name (in))\n(наружу \"Hello, \" name \"!\\n\")\n\n";
        assert_eq!(expect, crate::formater::rethink_newlines(String::from(source)));
    }
}
