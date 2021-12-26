use std::fmt;

// Node should have a kind (op or number) and point to a left or right. If it is num, it should
// have i32.

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mov(i32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add(left, right) => write!(f, "{} + {}", left.eval(), right.eval()),
            Op::Sub(left, right) => write!(f, "{} - {}", left.eval(), right.eval()),
            Op::Mul(left, right) => write!(f, "{} * {}", left.eval(), right.eval()),
            Op::Div(left, right) => write!(f, "{} / {}", left.eval(), right.eval()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Node {
    Op(Op),
    Num(i32),
}

impl Node {
    fn eval(&self) -> i32 {
        match self {
            Node::Op(op) => match op {
                Op::Add(left, right) => left.eval() + right.eval(),
                Op::Sub(left, right) => left.eval() - right.eval(),
                Op::Mul(left, right) => left.eval() * right.eval(),
                Op::Div(left, right) => left.eval() / right.eval(),
            },
            Node::Num(num) => *num,
        }
    }
}

fn get_number(chars: &Vec<char>, i: &mut usize) -> i32 {
    let mut s = String::new();
    while chars[*i].is_ascii_whitespace() {
        *i += 1;
    }
    while *i < chars.len() && chars[*i].is_ascii_digit() {
        s.push(chars[*i]);
        *i += 1;
    }
    s.parse::<i32>().unwrap()
}

fn tokenize(chars: &Vec<char>) -> Vec<Node> {
    let mut nodes = vec![];

    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            nodes.push(Node::Num(get_number(&chars, &mut i)));
        } else if c == '+' {
            let last = nodes.pop().unwrap();
            i += 1;
            nodes.push(Node::Op(Op::Add(
                Box::new(last),
                Box::new(Node::Num(get_number(&chars, &mut i))),
            )));
        } else if c == '-' {
            let last = nodes.pop().unwrap();
            i += 1;
            nodes.push(Node::Op(Op::Sub(
                Box::new(last),
                Box::new(Node::Num(get_number(&chars, &mut i))),
            )));
        } else if c == '*' {
            let last = nodes.pop().unwrap();
            i += 1;
            nodes.push(Node::Op(Op::Mul(
                Box::new(last),
                Box::new(Node::Num(get_number(&chars, &mut i))),
            )));
        } else if c == '/' {
            let last = nodes.pop().unwrap();
            i += 1;
            nodes.push(Node::Op(Op::Div(
                Box::new(last),
                Box::new(Node::Num(get_number(&chars, &mut i))),
            )));
        } else {
            eprintln!("unexpected character: '{}'", c);
            i += 1;
        }
    }
    nodes
}

fn emit(tokens: Vec<Node>) -> Vec<Instruction> {
    use Instruction::*;

    let mut instructions = vec![];

    for token in tokens {
        match token {
            Node::Num(num) => {
                instructions.push(Mov(num));
            }
            Node::Op(op) => match op {
                Op::Add(left, right) => {
                    instructions.push(Add(left.clone(), right.clone()));
                }
                Op::Sub(left, right) => {
                    instructions.push(Sub(left.clone(), right.clone()));
                }
                Op::Mul(left, right) => {
                    instructions.push(Mul(left.clone(), right.clone()));
                }
                Op::Div(left, right) => {
                    instructions.push(Div(left.clone(), right.clone()));
                }
            },
        }
    }
    instructions
}

fn print_instructions(instructions: Vec<Instruction>) {
    use Instruction::*;
    for instruction in instructions {
        match instruction {
            Mov(num) => println!("  mov ${}, %rax", num),
            Add(left, right) => {
                println!("  mov ${}, %rax", left.eval() + right.eval());
            }
            Sub(left, right) => {
                println!("  mov ${}, %rax", left.eval() - right.eval());
            }
            Mul(left, right) => {
                println!("  mov ${}, %rax", left.eval() * right.eval());
            }
            Div(left, right) => {
                println!("  mov ${}, %rax", left.eval() / right.eval());
            }
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
    twenty_plus_three: "20 + 3", vec![Add(Box::new(Node::Num(20)), Box::new(Node::Num(3)))],
    twenty_minus_three: "20 - 3", vec![Sub(Box::new(Node::Num(20)), Box::new(Node::Num(3)))],
}
