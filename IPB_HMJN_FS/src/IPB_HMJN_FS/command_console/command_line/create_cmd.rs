use std::path;

use crate::{API::Create, IPB_HMJN_FS::{FS_core_types::DirectoryType, command_console::command_line::{Command, OperationPropaty}}};
use crate::IPB_HMJN_FS::Constant;
use crate::IPB_HMJN_FS::FS_core_types::{SuperBlockData, Entry, EntryStructType, EntryType};
use std::fs;

pub fn create(execution: &crate::IPB_HMJN_FS::FS_execution::executions::Execution, arguments: Vec<&str>, propaty: &mut OperationPropaty) {
    let path = propaty.now_dir_path.as_str().clone().to_owned() + arguments[0];
    let mut output_file = if let Ok(file) = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(Constant::OUTPUT_FILE_NAME){
            file
        }
        else{
            println!("出力ファイルを開けませんでした");
            return;
        };
    let entry = Entry::new(EntryStructType::Directory, 0, 0, 0, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, arguments[0]);
    let _ = execution.create(&path, entry, &mut output_file);
}

pub struct create{
    
}
impl Command for create {
    fn execution(&mut self, execution: &crate::IPB_HMJN_FS::FS_execution::executions::Execution, arguments: Vec<&str>, propaty: &mut OperationPropaty) {
        let path = propaty.now_dir_path.as_str().clone().to_owned() + arguments[0];
        let mut output_file = if let Ok(file) = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(Constant::OUTPUT_FILE_NAME){
                file
            }
            else{
                println!("出力ファイルを開けませんでした");
                return;
            };
        let entry = Entry::new(EntryStructType::Directory, 0, 0, 0, 0, 0, 0, 0, EntryType::Directory(DirectoryType::new(0)), 0, arguments[0]);
        let _ = execution.create(&path, entry, &mut output_file);
    }
}

