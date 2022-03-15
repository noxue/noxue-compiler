use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn compile_and_run_c(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp & echo {:?} > test.c && gcc test.c -o test && if test -f \"./test\"; then\n timeout -v {} ./test \nfi",
        code,
        timeout
    );

    let image = "gcc";
    exec(image, &cmd, input)
}

pub fn compile_and_run_rust(code: &str, input: &str, timeout: i32) -> Result<Output, String> {
    let cmd = format!(
        "cd /tmp & echo {:?} > test.rs && rustc test.rs -o test && if test -f \"./test\"; then\n timeout -v {} ./test \nfi",
        code,
        timeout
    );

    let image = "rust";
    exec(image, &cmd, input)
}

#[derive(Debug)]
pub struct Output {
    pub stdout: String,
    pub stderr: String,
}

impl Output {
    pub fn new(stdout: String, stderr: String) -> Self {
        Output { stdout, stderr }
    }
}

pub fn exec(image: &str, cmd: &str, input: &str) -> Result<Output, String> {
    let mut child = match Command::new("docker")
        .arg("run")
        .arg("--rm")
        // .arg("--network none") // 禁止网络
        .arg("-i")
        .arg(image)
        .arg("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(v) => v,
        Err(e) => {
            println!("error:{}", e);
            return Err(e.to_string());
        }
    };

    // 标准输入
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let input = input.to_string();

    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    // 处理超时的情况，返回友好的提示信息，这里默认执行命令是使用 timeout 来指定超时
    if String::from_utf8_lossy(&output.stderr)
        .to_string()
        // 标准错误输出中 包含这串字符串就算超时，这个字符串是 timeout 超时才有的信息
        .contains("timeout: sending signal TERM to command")
    {
        return Err("运行超时".to_string());
    }

    Ok(Output::new(
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}
