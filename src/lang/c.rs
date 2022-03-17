use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        r#"cd /tmp
            cat>test.c<<NOXUE_EOFFFFFFF
{}
NOXUE_EOFFFFFFF
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
    let code = "// ' \r\n#include <stdio.h>\r\n#include<stdlib.h>\r\nint main(){\r\n  int i=1000;for(;i;i--) {int *p = (int*)malloc(1024*1024);} printf(\"hello\\n\");\r\n for(;;); return 0;\r\n}";

    // 标准输入的内容
    let input = r#""#;

    // 超时时间，单位:秒
    let timeout = 120;

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