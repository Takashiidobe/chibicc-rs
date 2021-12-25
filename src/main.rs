use std::fmt;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mov(i32),
    Add(i32),
    Sub(i32),
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "{}", '+'),
            Op::Sub => write!(f, "{}", '-'),
        }
    }
}

#[derive(Debug)]
enum Token {
    Op(Op),
    Num(i32),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Op(op) => write!(f, "{}", op),
            Token::Num(num) => write!(f, "{}", num),
        }
    }
}

impl Token {
    fn to_i32(&self) -> i32 {
        match self {
            Token::Num(num) => *num,
            Token::Op(_) => unreachable!(),
        }
    }
}

fn get_number(chars: &Vec<char>, i: &mut usize) -> i32 {
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
            tokens.push(Token::Num(get_number(&chars, &mut i)));
        } else if c == '+' {
            tokens.push(Token::Op(Op::Add));
            i += 1;
        } else if c == '-' {
            tokens.push(Token::Op(Op::Sub));
            i += 1;
        } else {
            eprintln!("unexpected character: '{}'", c);
            i += 1;
        }
    }
    tokens
}

fn emit(tokens: Vec<Token>) -> Vec<Instruction> {
    use Instruction::*;

    let token_len = &tokens.len();
    let mut instructions = vec![];
    let mut i = 0;

    while &i < token_len {
        let token = &tokens[i];
        match token {
            Token::Num(num) => {
                instructions.push(Mov(*num));
                i += 1;
            }
            Token::Op(op) => {
                match op {
                    Op::Add => {
                        if i < token_len - 1 {
                            instructions.push(Add(tokens[i + 1].to_i32()));
                        }
                    }
                    Op::Sub => {
                        if i < token_len - 1 {
                            instructions.push(Sub(tokens[i + 1].to_i32()));
                        }
                    }
                }
                i += 2;
            }
        }
    }
    instructions
}

fn print_instructions(instructions: Vec<Instruction>) {
    use Instruction::*;
    for instruction in instructions {
        match instruction {
            Mov(num) => println!("  mov ${}, %rax", num),
            Sub(num) => println!("  sub ${}, %rax", num),
            Add(num) => println!("  add ${}, %rax", num),
        }
    }
}

fn print_program(chars: Vec<char>) {
    println!("  .globl main");
    println!("main:");
    let instructions = emit(tokenize(&chars));
    print_instructions(instructions);
    println!("  ret");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        std::process::exit(1);
    }

    let chars: Vec<char> = args[1].chars().collect();
    print_program(chars);
}

macro_rules! test_assembly {
    ($($name:ident: $left:expr, $right:expr,)*) => {
        #[cfg(test)]
        mod test {
        use super::*;
        use super::Instruction::*;
        fn to_instructions(s: &str) -> Vec<Instruction> {
            emit(tokenize(&s.chars().collect()))
        }
            $(
                #[test]
                fn $name() {
                    let instructions = to_instructions($left);
                    assert_eq!(
                        instructions,
                        $right,
                    );
                }
            )*
        }
    }
}

test_assembly! {
    twenty_plus_three: "20 + 3", vec![Mov(20), Add(3)],
    twenty_minus_three: "20 - 3", vec![Mov(20), Sub(3)],
}
