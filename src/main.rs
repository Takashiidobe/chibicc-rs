use std::fmt;

#[derive(Debug)]
enum Token {
    Punct(char),
    Num(i32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Punct(sign) => write!(f, "{}", sign),
            Token::Num(num) => write!(f, "{}", num),
        }
    }
}

impl Token {
    fn to_i32(&self) -> i32 {
        match self {
            Token::Num(num) => *num,
            Token::Punct(_) => 0,
        }
    }
}

fn to_digit(chars: &Vec<char>, i: &mut usize) -> i32 {
    let mut s = String::new();
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        s.push(chars[*i]);
        *i += 1;
    }
    s.parse::<i32>().unwrap()
}

fn tokenize(chars: &Vec<char>) -> Vec<Token> {
    let mut tokens = vec![];

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            tokens.push(Token::Num(to_digit(&chars, &mut i)));
        } else if c == '+' || c == '-' {
            tokens.push(Token::Punct(c));
            i += 1;
        } else {
            i += 1;
            eprintln!("unexpected character: '{}'", c);
        }
    }
    tokens
}

fn emit(tokens: Vec<Token>) -> Vec<Instruction> {
    let token_len = &tokens.len();

    let mut instructions = vec![];

    let mut i = 0;

    while &i < token_len {
        let token = &tokens[i];
        match token {
            Token::Num(num) => {
                instructions.push(Instruction::Mov(*num));
                i += 1;
            }
            Token::Punct(sign) => {
                if *sign == '+' && i < token_len - 1 {
                    instructions.push(Instruction::Add(tokens[i + 1].to_i32()));
                }
                if *sign == '-' && i < token_len - 1 {
                    instructions.push(Instruction::Sub(tokens[i + 1].to_i32()));
                }
                i += 2;
            }
        }
    }
    instructions
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
    }

    let chars: Vec<char> = args[1].chars().collect();

    println!("  .globl main");
    println!("main:");
    let instructions = emit(tokenize(&chars));
    for instruction in instructions {
        match instruction {
            Instruction::Mov(num) => println!("  mov ${}, %rax", num),
            Instruction::Sub(num) => println!("  sub ${}, %rax", num),
            Instruction::Add(num) => println!("  add ${}, %rax", num),
        }
    }
    println!("  ret");
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mov(i32),
    Add(i32),
    Sub(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_instructions(s: &str) -> Vec<Instruction> {
        emit(tokenize(&s.chars().collect()))
    }

    #[test]
    fn test_1() {
        let instructions = to_instructions("20 + 3");
        assert_eq!(
            instructions,
            vec![Instruction::Mov(20), Instruction::Add(3)]
        );
    }
}
