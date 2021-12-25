fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
    }

    println!("  .globl main");
    println!("main:");
    println!("  mov ${}, %rax", args[1].parse::<i32>().unwrap());
    println!("  ret");
}
