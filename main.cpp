#include <iostream>
#include "lib/format_libs.hpp"

using namespace std;

int main(){
    // cout << sizeof(IPB_HMJN_FS::FS_TYPES::dir_entry) << endl;
    IPB_HMJN_FS::PARTITION_HEADER fs_header = IPB_HMJN_FS::SET_UP::setup_header_datas();
    IPB_HMJN_FS::FORMAT::format_fs(fs_header);
    uint8_t name[256] = "hello";
    IPB_HMJN_FS::EXECUTION::DIRECTORY_TREE::FS_DIRECTORY directory(fs_header);
    fstream file(IPB_HMJN_FS::output_filename, ios::in | ios::out | ios::binary);
    IPB_HMJN_FS::FS_TYPES::DIR_ENTRY dir_entry;
    cout << "start dir entry test0" << endl;
    dir_entry.data_type = IPB_HMJN_FS::FS_TYPES::DATA_TYPE::dir;
    dir_entry.ID = 1;
    dir_entry.parent_ID = 0;
    dir_entry.next_sibling_ID = 0;
    dir_entry.prev_sibling_ID = 0;
    dir_entry.another_type_data.dir.first_child_ID = 0;
    dir_entry.mode = 0;
    dir_entry.last_updata_time = 0;
    // dir_entry.name_size = 255;
    dir_entry.name_size = 1;
    IPB_HMJN_FS::FS_TYPES::DIR_ENTRY_INTERFACE dir_entry_data(dir_entry);
    dir_entry_data.set_name(name);
    IPB_HMJN_FS::EXECUTION::DIRECTORY_TREE::TREE_STRUCT dir_ID = directory.scan_dir_tree(file, fs_header.directory_tree_cluster_num, fs_header.directory_tree_depth);
    cout << "get ID : " << dir_ID.dir_tree_cluster << endl;
    directory.write_directory_data(dir_entry, file);
    directory.write_directory_data(dir_entry, file);
    IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION bitmap_exe(fs_header, file);
    IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA test = bitmap_exe.get_bitmap_data(10);
}