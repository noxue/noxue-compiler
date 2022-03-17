use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use uuid::Uuid;

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
    // 生成全局唯一的 container 名字，用于定时结束
    let container_name = format!("{}", Uuid::new_v4());
    let container_name1 = container_name.clone();

    // 开线程定时结束容器
    thread::spawn(move || {
        // 等待指定时间
        thread::sleep(Duration::from_secs(10));

        // 执行强制结束docker容器的命令
        Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(container_name1)
            .output()
            .unwrap();
    });

    println!("{}", container_name);
    let mut child = match Command::new("docker")
        .arg("run")
        .arg(format!("--name={}", container_name))
        .arg("--rm")
        .arg("--network=none") // 禁止网络
        .arg("--memory=10MB") // 限制内存
        .arg("--cpus=1")
        .arg("--memory-swap=-1")
        .arg("-i")
        .arg(image)
        .arg("/bin/bash")
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
    let input = format!("{}\n{}", cmd, input);

    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    println!("exec out:{:#?}", output);

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
