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
    print!("[{}@{} {}]# ",get_username(),get_hostname(),get_cwd());
    io::stdout().flush()?;
    return io::stdin().read_line(command);    
}
fn command_split(input:&str)->Vec<&str>{
    return input.split(SEP).collect();
}
fn execute(argv:&[&str],redir:Redir,filename:Option<&str>)->io::Result<()>
{
    let mut cmd=Command::new(argv[0]);
    cmd.args(&argv[1..]);
    if let Some(file)=filename{
        match redir{
            Redir::Input=>{
                let input=File::open(file)?;
                cmd.stdin(Stdio::from(input));
            }
            Redir::Output=>{
                let output=File::create(file)?;
                cmd.stdout(Stdio::from(output));
            }
            Redir::Append=>{
                let output=OpenOptions::new().append(true).open(file)?;
                cmd.stdout(Stdio::from(output));
            }
            Redir::None=>{}

        }
    }
    let status=cmd.status()?;
    if !status.success(){
        println!("command fail with status:{}",status);
    }
    return Ok(());
}
fn cd(path:&str)
{
    if let Err(e)=env::set_current_dir(path){
        eprintln!("cd:{}",e);

    }
}
fn do_builtin(argv:&[&str])->bool{
    match argv[0]{
        "cd"=>{
            let path=if argv.len()>1 {argv[1]} else {&homepath()};
            cd(path);
            return true;
        }
        "export"=>{
            if argv.len()>1{
                let key_value:Vec<&str>=argv[1].split('=').collect();
                if key_value.len()==2{
                    env::set_var(key_value[0], key_value[1]);
                }
            }
            return true;
        }
        "echo"=>{
            if argv.len()>1{
                println!("{}",argv[1]);
            }
            else{
                println!();
            }
            return true;
        }
        _ =>{return false;}
    }
}
fn main() {
    //读取命令行
    let mut redir=Redir::default();
    let mut filename:Option<&str> =None;
    loop{
        redir=Redir::None;
        filename=None;
        let mut usercommand=String::new();
        let n = get_user_command(&mut usercommand).unwrap();
        if n==0{
            continue;
        }
        usercommand.pop();
        let argv:Vec<&str>=command_split(&usercommand);
        if do_builtin(&argv){
            continue;
        }
        if let Err(e)=execute(&argv,redir,filename){
            eprintln!("Execute error:{}",e);
        }
    }

    //分割命令行
    //区分内置命令和外部命令
    //执行外部命令（创建子进程调用外部执行程序）和内置命令（直接当前进程执行）
    
}
