use std::default;

use crate::IPB_HMJN_FS::FS_core_types::{self, SuperBlockData};

struct execution{
    super_block: SuperBlockData,
}
impl execution{
    fn new(super_block: SuperBlockData) -> Self{
        Self{
            super_block,

        }
    }


}