pub mod console_type;
pub mod command_line;

use command_line::{Command, OperationPropaty};
use crate::IPB_HMJN_FS::FS_execution::executions::Execution;
use crate::IPB_HMJN_FS::general_function;
use crate::IPB_HMJN_FS::command_console::{command_line::{create_cmd}};

use std::io;
use std::io::Write;
use std::collections::HashMap;

type general_type = fn(execution: &Execution, Vec<&str>, &mut OperationPropaty);

fn insert_cmd_data(cmd_name: String, function: general_type, cmd_list: &mut HashMap<String, general_type>){
    cmd_list.insert(cmd_name, function);
}
fn setup_cmd() -> HashMap<String, general_type>{
    let mut cmd_list:HashMap<String, general_type> = Default::default();
    insert_cmd_data("create".to_string(), create_cmd::create, &mut cmd_list);
    cmd_list
}
fn use_cmd(execution: &Execution, cmd_name: &str, argument: Vec<&str>, propaty: &mut OperationPropaty, cmd_list: &HashMap<String, general_type> ) -> Option<()>{
    if cmd_name.trim() == "help"{
        println!("exit");
        println!("end");
        println!("return");
        for func_name in cmd_list.keys(){
            println!("{func_name}");
        }
    }
    else{
        if let Some(function) = cmd_list.get(cmd_name){
            function(execution, argument, propaty);
        }
        else{
            print!("指定したコマンドは存在しません コマンド名 : {}", cmd_name);
        }
    }
    Some(())
}

// fn insert_cmd_data(cmd_name: String, function: Box<dyn Command>, cmd_list: &mut HashMap<String, Box<dyn Command>>){
//     cmd_list.insert(cmd_name, function);
// }
// fn setup_cmd() -> HashMap<String, Box<dyn Command>>{
//     let mut cmd_list: HashMap<String, Box<dyn Command>> = Default::default();
//     insert_cmd_data("create".to_string(), create_cmd::create::execution(&mut self, execution, arguments, propaty), &mut cmd_list);
//     cmd_list
// }
// fn use_cmd(execution: &Execution, cmd_name: &str, argument: Vec<&str>, propaty: &mut OperationPropaty, cmd_list: &mut HashMap<String, Box<dyn Command>>)
// -> Option<()>
// {
//     cmd_list.get_mut(cmd_name)?.execution(execution, argument, propaty);
//     Some(())
// }

pub fn start_command_console(execution: &Execution){
    let mut propaty = OperationPropaty::new();
    let mut input = String::new();
    let mut cmd_list = setup_cmd();
    println!("test");
    while true {
        print!("cmd > ");
        io::stdout().flush().unwrap();
        if let Err(e) = io::stdin().read_line(&mut input){
            eprintln!("{}",e);
            continue;
        }
        if input.trim() == "exit" || input.trim() == "end" || input.trim() == "return"{
            break;
        }
        let cmd = general_function::str_analysis(input.as_str().trim(), " ");
        if let Some(cmd_name) = cmd.get(0).cloned(){
            let argument_line = cmd.len();
            let arguments = cmd[1..argument_line].to_vec();
            use_cmd(execution, cmd_name, arguments, &mut propaty, &cmd_list);
        }
        input.clear();
    }
}