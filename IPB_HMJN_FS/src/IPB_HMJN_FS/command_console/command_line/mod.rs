pub mod create_cmd;

use crate::IPB_HMJN_FS::FS_execution::executions::{Execution};

#[derive(Clone)]
pub struct OperationPropaty{
    now_dir_path: String
}
impl OperationPropaty{
    pub fn new() -> Self{
        Self{
            now_dir_path: String::new()
        }
    }
}

pub trait Command {
    fn execution(&mut self, execution: &Execution, arguments: Vec<&str>, propaty: &mut OperationPropaty);
}
