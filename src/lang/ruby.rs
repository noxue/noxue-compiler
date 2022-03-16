use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp && echo '{}' > test.rb && timeout -v {} ruby test.rb",
        code, timeout
    );

    let image = "ruby";
    exec(image, &cmd, input)
}

#[test]
fn test() {
    // 要执行的代码
    let code = r#"#!/usr/bin/ruby
    puts "hello";
    "#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 2;

    let out = run(code, input, timeout);
    assert_eq!(out.unwrap().stdout, "hello\n");
}
