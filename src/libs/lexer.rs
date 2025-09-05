use once_cell::sync::Lazy;
use regex::Regex;
use itertools::izip;

#[derive(Debug)]
pub enum TokenKinds {
    LeftParen,
    RightParen,
    PlusOp,
    MinusOp,
    MultOp,
    DivOp,
    Word,
    Unknown,
}

struct Re {
    word: Regex,
}
static RE: Lazy<Re> = Lazy::new(|| Re {
    word: Regex::new(r"[^\s()]+").unwrap(),
});

pub fn classify_token(token_str: &str) -> TokenKinds {
    match token_str {
        "(" => TokenKinds::LeftParen,
        ")" => TokenKinds::RightParen,
        "+" => TokenKinds::PlusOp,
        "-" => TokenKinds::MinusOp,
        "*" => TokenKinds::MultOp,
        "/" => TokenKinds::DivOp,
        _ if RE.word.is_match(token_str) => TokenKinds::Word,
        _ => TokenKinds::Unknown,
    }
}

pub fn sexprid_giver(all_kinds: &Vec<TokenKinds>) -> Vec<usize> {
    let mut stack: Vec<usize> = Vec::new();
    let mut all_sexprid: Vec<usize> = Vec::with_capacity(all_kinds.len());
    

    let mut next_id: usize = 1;
    for token in all_kinds.iter() {
        let current_id = match token {
            TokenKinds::LeftParen => {
                let id = next_id;
                stack.push(id);
                next_id += 1;
                id
            },
            TokenKinds::RightParen => {
                let id = *stack.last().unwrap_or(&0); // ID 0 S-exp is the S-Expression of all
                                                      // S-Expressions
                stack.pop();
                id
            },
            _ => {
                *stack.last().unwrap_or(&0)
            }
        };
        
        all_sexprid.push(current_id);
    }
    
    all_sexprid
}

pub fn tokens_depth(all_kinds: &Vec<TokenKinds>) -> Vec<usize> {
    let mut all_depths: Vec<usize> = Vec::with_capacity(all_kinds.len());
    let mut current_depth: usize = 0;

    for kind in all_kinds.iter() {
        if let TokenKinds::RightParen = kind {
            current_depth = current_depth.saturating_sub(1);
        }

        all_depths.push(current_depth);

        if let TokenKinds::LeftParen = kind {
            current_depth += 1;
        }
    }

    all_depths
}

pub fn lexer(program: &str) -> Vec<(TokenKinds, String, usize, usize, usize)> {
    let replaced_program = program
        .replace('(', " ( ")
        .replace(')', " ) ");

    let tokens: Vec<&str> = replaced_program
        .split_whitespace()
        .collect();

    let all_kinds: Vec<TokenKinds> = tokens
        .iter()
        .map(|&s| classify_token(s))
        .collect();

    let all_lexemes: Vec<String> = tokens
        .iter()
        .map(|&s| String::from(s))
        .collect();

    let all_spans: Vec<usize> = tokens
        .iter()
        .map(|s| s.len())
        .collect();

    let all_sexprid = sexprid_giver(&all_kinds);
    let all_depths = tokens_depth(&all_kinds);

    izip!(all_kinds, all_lexemes, all_spans, all_depths, all_sexprid)
        .collect()
}
