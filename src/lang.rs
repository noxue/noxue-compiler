use serde::{Deserialize, Serialize};

use crate::exec::{exec, Output};

#[derive(Serialize, Deserialize)]
struct RunTpl {
    image: String,  // docker iamge 名字
    file: String,   // 代码要保存的文件路径
    cmd: String,    // 保存代码之后要执行的命令
    timeout: i32,   // 容器执行超时时间
    memory: String, // 允许容器使用的内存,例如:20MB
    cpuset: String, // 使用的cpu核心
}

/// 根据语言选择特定的执行模板来编译运行代码
///
/// ## 参数
///
/// * `tpl` - 指定的运行模板内容,使用模板更加方便扩展任意编程语言
/// * `code` - 要编译的代码
/// * `input` - 标准输入，用户给程序的输入数据
///
/// ## 模板格式：
/// ```json
/// {
///     "image": "gcc",  // 要使用那个docker imgage
///     "file": "test.c", // 代码保存的文件名
///     "cmd": "gcc test.c -o test\nif test -f \"./test\"; then\n./test\nfi",  // 代码保存后要执行的命令
///     "timeout": 10, // 超时时间，是从启动docker开始计算
///     "memory": "20MB", // 允许占用的内存
///     "cpuset":"0-3", // 使用的cpu核心
/// }
/// ```
///
/// ## 调用方式
///
/// ```
/// use noxue_compiler::lang::run;
/// fn main(){
///     let code = r#"
/// #include <stdio.h>
///
/// int main(){
///     printf("hello");
///     return 0;
/// }"#;
///     let tpl = r#"
/// {
/// "image": "gcc",
/// "file": "test.c",
/// "cmd": "gcc test.c -o test\nif test -f \"./test\"; then\n./test\nfi",
/// "timeout": 10,
/// "memory":"20MB",
/// "cpuset":"0-3"
/// }
/// "#;
///     let out = run(tpl, code, "");
///     assert_eq!(out.unwrap().stdout, "hello");
/// }
/// ```
///
pub fn run(tpl: &str, code: &str, input: &str) -> Result<Output, String> {
    /***
     * 根据参数生成命令
     *
     * 1. 根据模板json数据，解析成结构体 RunTpl
     * 2. 拼接 写入文件 编译运行文件 命令
     * 3. 调用exec函数执行 以上命令
     */

    let run_tpl: RunTpl = match serde_json::from_str(&tpl) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("运行模板不正确：{}", e));
        }
    };

    // 开始结束字符串，随机生成防止内容中包含该字符串导致输入结束
    let eof = format!("{}", uuid::Uuid::new_v4());

    let cmd = format!(
        "cat>{}<<{}\n{}\n{}\n{}",
        run_tpl.file, eof, code, eof, run_tpl.cmd
    );

    exec(
        &run_tpl.image,
        &cmd,
        input,
        run_tpl.timeout,
        &run_tpl.memory,
        &run_tpl.cpuset,
    )
}
