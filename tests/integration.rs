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
    test_10: "10", "
  .globl main
main:
  mov $10, %rax
  ret",
    test_add_and_sub: "5+20+4-20-50+30+40", "
  .globl main
main:
  mov $5, %rax
  add $20, %rax
  add $4, %rax
  sub $20, %rax
  sub $50, %rax
  add $30, %rax
  add $40, %rax
  ret",
}
