use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp && echo {:?} > test.php && timeout {} php test.php",
        code, timeout
    );

    let image = "php:7";
    exec(image, &cmd, input)
}

#[test]
fn test_php7() {
    // 要执行的代码
    let code = r#"<?php
    echo "hello";
    "#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 2;

    let out = run(code, input, timeout);
    assert_eq!(out.unwrap().stdout, "hello");
}
