#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyWords {
    Const,      // `пост`
    Function,   // `объяв`
    Input,      // `внутрь`
    Output,     // `наружу`
    TailRec,    // `@хр`
    Recur,      // `себя`
    Plus,       // `+`/`слож`
    Minus,      // `-`/`вычит`
    Multiply,   // `*`/`произв`
    Divide,     // `/`/`частн`
    Remainder,  // `%`/`остат`
    Equal,      // `=`/`равны`
    NotEqual,   // `!=`/`неравны`
    Greater,    // `>`/`больше`
    If,         // `?`/`если`
    Shift,      // `<<`/`сдвг`
    ScopeStart, // `(`
    ScopeEnd,   // `)`
    None,       // if nothing matched
}
#[allow(dead_code)]
pub fn parse_string(s: &str) -> KeyWords {
    match s {
        "пост" => KeyWords::Const,
        "объяв" => KeyWords::Function,
        "внутрь" => KeyWords::Input,
        "наружу" => KeyWords::Output,
        "@хр" => KeyWords::TailRec,
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
        _ => KeyWords::None,
    }
}
