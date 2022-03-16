use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        r#"cd /tmp
            echo  '{}' > test.c
            gcc test.c -o test
            if test -f "./test"; then
                timeout -v {} ./test 
            fi"#,
        code,
        timeout
    );

    // let cmd = format!("{:?}", cmd);
    println!("cmd:{}", cmd);

    let image = "gcc";
    exec(image, &cmd, input)
}


#[test]
fn test_c() {
    // 要执行的代码
    let code = "#include <stdio.h>\r\nint main(){\r\n   printf(\"hello\\n\");\r\n    return 0;\r\n}";

    // 标准输入的内容
    let input = r#"hello"#;

    // 超时时间，单位:秒
    let timeout = 20;

    let out = run(code, input, timeout);
    println!("{:?}", out);
    // assert_eq!(out.unwrap().stderr, "n:hello");
}

#[test]
fn test_c_timeout() {
    // 要执行的代码
    let code = r#"
        int main()
        {
            for(;;);
            return 0;
        }
    "#;

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 2;

    let out = run(code, input, timeout);
    assert!(out.is_err());
}