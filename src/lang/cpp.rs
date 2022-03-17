use crate::exec::{exec, Output};

pub fn run(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp & echo '{}' > test.cpp && g++ test.cpp -o test && if test -f \"./test\"; then\n timeout -v {} ./test \nfi",
        code,
        timeout
    );

    let image = "gcc";
    exec(image, &cmd, input)
}


#[test]
    fn test_cpp() {
        // 要执行的代码
        let code = r#"#include <iostream>

            int main()
            {
                std::cerr << "hello";
                std::cout << "world";
                return 0;
            }
        "#;

        // 标准输入的内容
        let input = r#""#;

        // 超时时间，单位:秒
        let timeout = 2;

        let out = run(code, input, timeout).unwrap();
        assert_eq!(out.stderr, "hello");
        assert_eq!(out.stdout, "world");
    }