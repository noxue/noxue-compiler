use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "echo '{}' > Test.java && javac -encoding utf8 Test.java && if test -f \"./Test.class\"; then\n timeout {} java Test \nfi",
        code,
        timeout
    );

    let image = "openjdk:19";
    exec(image, &cmd, input)
}

// #[test]
fn test() {
    // 要执行的代码
    let code =include_str!("./Test.java");// r#"public class Test {public static void main(String[] args) {System.out.print("hello");}}"#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 10;

    
    let out = run(code, input, timeout).unwrap();
    println!("stdout:{}", out.stderr);
    assert_eq!(out.stdout, "hello");
}
