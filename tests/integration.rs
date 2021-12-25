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
                    cmd.assert().success().stdout(format!(
                        "  .globl main\nmain:\n  mov ${}, %%rax\n  ret\n",
                        $right
                    ));

                    Ok(())
                }
            )*
        }
    }
}

test_stdout! {
    test_10: "10", "10",
}
