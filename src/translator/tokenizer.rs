use crate::srs_commands::{parse_string, CompleteToken, KeyWords, TokenType};

#[path = "../util/formater.rs"]
mod formater;
#[derive(Debug)]
pub struct Program {
    pub strings: Vec<String>,
    pub functions: Vec<Vec<CompleteToken>>,
    pub constants: Vec<Vec<CompleteToken>>,
    pub code: Vec<Vec<CompleteToken>>,
}

pub fn arrange_tokens(tokens: Vec<Vec<CompleteToken>>, strings: Vec<String>) -> Program {
    let mut program: Program = Program {
        strings: strings,
        functions: Vec::new(),
        constants: Vec::new(),
        code: Vec::new(),
    };
    for line in tokens {
        let mut used = false;
        for token in &line {
            if token.token == KeyWords::Const {
                program.constants.push(line.clone());
                used = true;
                break;
            }
            if token.token == KeyWords::Function {
                program.functions.push(line.clone());
                used = true;
                break;
            }
        }
        if !used {
            program.code.push(line);
        }
    }
    program
}
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
    let mut argscope_started = false;
    for i in 0..tokens.len() {
        if argscope_started && tokens[i] != "]" {
            args.push(tokens[i].clone());
            let final_token = CompleteToken {
                token_type: TokenType::None,
                token: KeyWords::None,
                numeric_rep: -1,
                string_rep: tokens[i].clone(),
            };
            commands.push(final_token);
            continue;
        }
        let keyword = parse_string(&tokens[i]);
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
                if commands[i - 1].token == KeyWords::Function
                    || commands[i - 1].token == KeyWords::Const
                {
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
            KeyWords::ArgScopeStart => {
                argscope_started = true;
                TokenType::ArgScope
            },
            KeyWords::ArgScopeEnd => {
                argscope_started = false;
                TokenType::ArgScope
            },
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
