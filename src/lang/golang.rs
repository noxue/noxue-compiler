use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp && echo {:?} > test.go && go run test.go && if test -f \"./test\"; then\n timeout -v {} ./test \nfi",
        code,
        timeout
    );

    let image = "golang";
    exec(image, &cmd, input)
}

#[test]
fn test() {
    // 要执行的代码
    let code = r#"
    package main

    import "fmt"

    func main() {
        fmt.Print("hello")
    }
    "#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 2;

    let out = run(code, input, timeout);
    assert_eq!(out.unwrap().stdout, "hello");
}
