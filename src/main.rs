fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
    }

    let chars: Vec<char> = args[1].chars().collect();

    println!("  .globl main");
    println!("main:");

    let first_arg = chars
        .iter()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    println!("  mov ${}, %rax", first_arg);

    let mut i = first_arg.len();

    while i < chars.len() {
        let c = chars[i];
        if c == '+' {
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                s.push(chars[i]);
                i += 1;
            }
            println!("  add ${}, %rax", s.parse::<i32>().unwrap());
        } else if c == '-' {
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                s.push(chars[i]);
                i += 1;
            }
            println!("  sub ${}, %rax", s.parse::<i32>().unwrap());
        } else {
            eprintln!("unexpected character: '{}'", c);
            i += 1;
        }
    }

    println!("  ret");
}
