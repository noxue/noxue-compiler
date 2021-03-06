use std::{
    io::Write,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
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

/// ## 使用指定的image执行命令
///
/// *  `image`      - docker image 名字
/// *  `cmd`        - 要在容器中执行的命令
/// *  `input`      - 用户执行程序是要输入的内容
/// *  `timeout`    - 容器超时时间，单位为秒
/// *  `memory`     - 容器运行时可以使用的内存大小，例如：50MB
/// *  `cpuset`     - 运行时可以使用的cpu，比如：0-3，表示使用4个cpu
pub fn exec(
    image: &str,
    cmd: &str,
    input: &str,
    timeout: i32,
    memory: &str,
    cpuset: &str,
) -> Result<Output, String> {

    log::debug!("cmd:{}", cmd);

    // 生成全局唯一的 container 名字，用于定时结束
    let container_name = format!("{}", Uuid::new_v4());
    let container_name1 = container_name.clone();

    // 记录是否超时
    let is_timeout = Arc::new(Mutex::new(false));

    let is_timeout1 = is_timeout.clone();
    // 开线程定时结束容器
    thread::spawn(move || {
        // 等待指定时间
        thread::sleep(Duration::from_secs(timeout as u64));

        // 执行强制结束docker容器的命令
        Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(container_name1)
            .output()
            .unwrap();

        let mut is_timeout = is_timeout1.lock().unwrap();
        // 超时设置为 true
        *is_timeout = true;
    });

    let mut child = match Command::new("docker")
        .arg("run")
        .arg(format!("--name={}", container_name))
        .arg("--rm")
        .arg("--network=none") // 禁止网络
        .arg(format!("--memory={}", memory)) // 限制内存
        .arg("--cpus=1")
        .arg("--memory-swap=-1")
        .arg(format!("--cpuset-cpus={}", cpuset))
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

    // 处理超时的情况，返回友好的提示信息
    if *is_timeout.clone().lock().unwrap() {
        return Err("运行超时".to_string());
    }

    Ok(Output::new(
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}
