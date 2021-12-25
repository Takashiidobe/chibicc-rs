use assert_cmd::prelude::*;
use std::process::Command;

#[macro_export]
macro_rules! test_stdout {
    ($($name:ident: $left:expr, $right:expr,)*) => {
        #[cfg(test)]
        mod test {
        use super::*;
            $(
                #[test]
                fn $name() -> Result<(), Box<dyn std::error::Error>> {
                    let mut cmd = Command::cargo_bin("chibicc")?;

                    cmd.arg($left);
                    cmd.assert().success().stdout($right);

                    Ok(())
                }
            )*
        }
    }
}

test_stdout! {
    test_10_plus_20: "10+20", "  .globl main\nmain:\n  mov $10, %rax\n  add $20, %rax\n  ret\n",
    test_10: "10", "  .globl main\nmain:\n  mov $10, %rax\n  ret\n",
    test_add_and_sub: "5+20+4-20-50+30+40", "  .globl main\nmain:\n  mov $5, %rax\n  add $20, %rax\n  add $4, %rax\n  sub $20, %rax\n  sub $50, %rax\n  add $30, %rax\n  add $40, %rax\n  ret\n",
}
