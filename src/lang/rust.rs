use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp & echo '{}' > test.rs && rustc test.rs -o test && if test -f \"./test\"; then\n timeout -v {} ./test \nfi",
        code,
        timeout
    );

    let image = "rust";
    exec(image, &cmd, input)
}


#[test]
fn test_rust() {
    // 要执行的代码
    let code = r#"
    fn main(){
        print!("hello world");
    }
    "#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 2;

    let out = run(code, input, timeout);
    assert_eq!(out.unwrap().stdout, "hello world");
}