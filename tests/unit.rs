#[path = "../src/util/formater.rs"]
mod formater;
#[path = "../src/translator/srs_commands.rs"]
mod srs_commands;
#[cfg(test)]
mod unit {
    use crate::srs_commands::KeyWords;

    #[test]
    fn test_formater() {
        let source = "(наружу \"What is your name?\\n\")\n(пост name (in)) (\n наружу \"Hello, \" name \"!\\n\")\n\n";
        let expect = "(наружу \"What is your name?\\n\")\n(пост name (in))\n(наружу \"Hello, \" name \"!\\n\")\n\n";
        assert_eq!(
            expect,
            crate::formater::rethink_newlines(String::from(source))
        );
    }

    #[test]
    fn test_command_parser() {
        let source = ["+", "слож", "@хр", "(", "unknown"];
        let expect = [
            KeyWords::Plus,
            KeyWords::Plus,
            KeyWords::TailRec,
            KeyWords::ScopeStart,
            KeyWords::None,
        ];
        let mut result = [KeyWords::None; 5];
        for i in 0..source.len() {
            result[i] = crate::srs_commands::parse_string(source[i]);
        }
        assert_eq!(result, expect);
    }
}
