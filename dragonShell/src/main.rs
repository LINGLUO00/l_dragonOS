use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const NUM: usize = 1024;
const SIZE: usize = 64;
const SEP: &str = " "; //separator

#[derive(PartialEq)]
enum Redir {
    None,
    Output,
    Append,
    Input,
}
impl Default for Redir {
    fn default() -> Self {
        return Redir::None;
    }
}
fn homepath() -> String {
    return env::var("HOME").unwrap_or_else(|_| ".".to_string());
}
fn get_username() -> String {
    return env::var("USER").unwrap_or_else(|_| "none".to_string());
}
fn get_hostname() -> String {
    return env::var("HOSTNAME").unwrap_or_else(|_| "none".to_string());
}
fn get_cwd() -> String {
    return env::current_dir().unwrap().display().to_string();
}
fn get_user_command(command:&mut String)->io::Result<usize>{
    print!("[{}@{} {}]##,"get_username(),get_hostname(),get_cwd());
    io::stdout().flush()?;
    return io::stdin().read_line(command);    
}
fn command_split(input:&str)->Vec<&str>{
    return input.split(SEP).collect();
}
fn main() {
    //读取命令行
    //分割命令行
    //区分内置命令和外部命令
    //执行外部命令（创建子进程调用外部执行程序）和内置命令（直接当前进程执行）
    
}
