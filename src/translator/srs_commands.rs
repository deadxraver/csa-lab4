#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyWords {
    Const,           // `пост`
    Function,        // `объяв`
    Input,           // `внутрь`
    Output,          // `наружу`
    Recur,           // `себя`
    Plus,            // `+`/`слож`
    Minus,           // `-`/`вычит`
    Multiply,        // `*`/`произв`
    Divide,          // `/`/`частн`
    Remainder,       // `%`/`остат`
    Equal,           // `=`/`равны`
    NotEqual,        // `!=`/`неравны`
    Greater,         // `>`/`больше`
    If,              // `?`/`если`
    Shift,           // `<<`/`сдвг`
    FromMemory,      // function call / constant use
    StringUse,       // e.g. (print "something")
    ImmediateNumber, // e.g. (print 5)
    ImmediateChar,   // e.g. (print '\n')
    ScopeStart,      // `(`
    ScopeEnd,        // `)`
    ArgScopeStart,   // `[`
    ArgScopeEnd,     // `]`
    None,            // if nothing matched
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    Keyword,
    Number,
    Char,
    Const,
    FunctionCall,
    String,
    Scope,
    ArgScope,
    None,
    Declaration,
}

#[derive(Debug)]
pub struct CompleteToken {
    pub token_type: TokenType,
    pub token: KeyWords,
    pub numeric_rep: i32,   // immediate value / string number
    pub string_rep: String, // function name / constant name
}
#[allow(dead_code)]
pub fn parse_string(s: &str) -> KeyWords {
    if s.starts_with('&') {
        KeyWords::StringUse
    } else if s.starts_with(char::is_numeric) {
        KeyWords::ImmediateNumber
    } else if s.starts_with(char::is_alphabetic) {
        let result = find_match_with_enum(s);
        if result == KeyWords::None {
            KeyWords::FromMemory
        } else {
            result
        }
    } else {
        find_match_with_enum(s)
    }
}

fn find_match_with_enum(s: &str) -> KeyWords {
    if s.starts_with("'") {
        KeyWords::ImmediateChar
    } else {
        match s {
            "пост" => KeyWords::Const,
            "объяв" => KeyWords::Function,
            "внутрь" => KeyWords::Input,
            "наружу" => KeyWords::Output,
            "себя" => KeyWords::Recur,
            "+" | "слож" => KeyWords::Plus,
            "-" | "вычит" => KeyWords::Minus,
            "*" | "произв" => KeyWords::Multiply,
            "/" | "частн" => KeyWords::Divide,
            "%" | "остат" => KeyWords::Remainder,
            "=" | "равны" => KeyWords::Equal,
            "!=" | "неравны" => KeyWords::NotEqual,
            ">" | "больше" => KeyWords::Greater,
            "?" | "если" => KeyWords::If,
            "<<" | "сдвг" => KeyWords::Shift,
            "(" => KeyWords::ScopeStart,
            ")" => KeyWords::ScopeEnd,
            "[" => KeyWords::ArgScopeStart,
            "]" => KeyWords::ArgScopeEnd,
            _ => KeyWords::None,
        }
    }
}
