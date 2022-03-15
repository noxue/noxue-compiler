use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp && echo {:?} > test.py && timeout -v {} python test.py",
        code, timeout
    );

    let image = "python:3";
    exec(image, &cmd, input)
}

#[test]
fn test() {
    // 要执行的代码
    let code = r#"print("hello", end="")
    "#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 2;

    let out = run(code, input, timeout);
    assert_eq!(out.unwrap().stdout, "hello");
}
