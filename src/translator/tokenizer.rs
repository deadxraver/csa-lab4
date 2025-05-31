use crate::srs_commands::{parse_string, CompleteToken, KeyWords, TokenType};

#[path = "../util/formater.rs"]
mod formater;

pub fn tokenize(input: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut strings: Vec<String> = Vec::new();
    for line in formater::rethink_newlines(formater::extract_strings(input, &mut strings))
        .trim()
        .split('\n')
    {
        lines.push(
            line.replace("(", " ( ")
                .replace(")", " ) ")
                .replace("[", " [ ")
                .replace("]", " ] ")
                .trim()
                .split(' ')
                .map(|s| s.to_string())
                .filter(|s| s != "")
                .collect(),
        );
    }
    (lines, strings)
}

pub fn strings_to_enum(tokens: &Vec<String>) -> Vec<CompleteToken> {
    let mut commands: Vec<CompleteToken> = Vec::with_capacity(tokens.len());
    let mut args: Vec<String> = Vec::new();
    for i in 0..tokens.len() {
        let mut keyword = parse_string(&tokens[i]);
        let token_type: TokenType = match keyword {
            KeyWords::Const | KeyWords::Function => TokenType::Keyword,
            KeyWords::Input | KeyWords::Output => TokenType::FunctionCall,
            KeyWords::Recur => TokenType::FunctionCall,
            KeyWords::Plus
            | KeyWords::Minus
            | KeyWords::Multiply
            | KeyWords::Divide
            | KeyWords::Remainder
            | KeyWords::Equal
            | KeyWords::NotEqual
            | KeyWords::Greater
            | KeyWords::If
            | KeyWords::Shift => TokenType::Keyword,
            KeyWords::FromMemory => {
                if commands[i - 1].token == KeyWords::Function || commands[i - 1].token == KeyWords::Const {
                    TokenType::Declaration
                } else if commands[i - 1].token == KeyWords::ScopeStart {
                    TokenType::FunctionCall
                } else {
                    TokenType::Const
                }
            }
            KeyWords::StringUse => TokenType::String,
            KeyWords::ImmediateNumber => TokenType::Number,
            KeyWords::ScopeStart | KeyWords::ScopeEnd => TokenType::Scope,
            KeyWords::ArgScopeStart | KeyWords::ArgScopeEnd => TokenType::ArgScope,
            KeyWords::None => TokenType::None,
            KeyWords::ImmediateChar => TokenType::Char,
        };
        let mut final_token = CompleteToken {
            token_type,
            token: keyword,
            numeric_rep: -1,
            string_rep: String::new(),
        };
        if token_type == TokenType::Number {
            final_token.numeric_rep = tokens[i]
                .parse::<i32>()
                .expect("Error while parsing numeric arguments");
        } else if token_type == TokenType::String {
            final_token.numeric_rep = tokens[i].replace('&', "").parse::<i32>().unwrap();
        } else if token_type == TokenType::FunctionCall {
            final_token.string_rep = tokens[i].clone();
        } else if token_type == TokenType::Const {
            let res = args.binary_search(&tokens[i]);
            if res.is_ok() {
                final_token.numeric_rep = res.unwrap() as i32;
            } else {
                final_token.string_rep = tokens[i].clone();
            }
        } else if token_type == TokenType::Declaration {
            final_token.string_rep = tokens[i].clone();
            final_token.token = commands[i - 1].token; 
        }

        commands.push(final_token);
    }
    commands
}
