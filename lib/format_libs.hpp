#include<cstdint>
#include<iostream>
#include<filesystem>
#include<fstream>
#include<string>
#include<vector>
#include "setting_macro.hpp"
#include <cmath>

#if OUTPUT_DEBUG_LOGS == 1
#include<bitset>
#endif

namespace IPB_HMJN_FS{
    static const uint8_t FS_VERSION_TOP = 0;
    static const uint8_t FS_VERSION_MID = 0;
    static const uint8_t FS_VERSION_LOW = 0;
    static const uint64_t FS_SECTOR_SIZE = 512;
    static const uint64_t FS_BACK_UP_NUM = 1;
    const uint8_t MAGIC_NUMBER[8] = {'I', 'P', 'B', '_', 'H', 'M', 'J', 'N'};

    std::string output_filename = "partition_disk.bin";

    enum check_sum{
        CRC32
    };

    #pragma pack(push, 1)
    struct PARTITION_HEADER
    {
        uint8_t magic_number[8] = {'I', 'P', 'B', '_', 'H', 'M', 'J', 'N'};
        uint8_t back_up_or_main = 0;
        struct{
            uint8_t top = FS_VERSION_TOP;
            uint8_t mid = FS_VERSION_MID;
            uint8_t low = FS_VERSION_LOW;
        }fs_version;
        uint64_t partition_LBA = 0;
        uint64_t one_sector_size = FS_SECTOR_SIZE;
        uint64_t one_block_sector_num = 0;
        uint64_t one_cluster_block_num = 0;
        uint64_t partition_cluster_size = 0;
        uint64_t bitmap_cluster_num = 0;
        uint64_t bitmap_size = 0;
        uint64_t back_up_num = FS_BACK_UP_NUM;
        uint64_t back_up_list_cluster_num = 0;
        uint64_t directory_tree_cluster_num = 0;
        uint64_t directory_tree_depth = 0;
        uint64_t free_ID_tree_cluster_num = 0;
        uint64_t free_ID_tree_depth = 0;
        uint64_t reservation_space_size = 0;//reservation space size num
        uint64_t read_algorithm_num = 0;
        uint64_t write_algorithm_num = 0;
        uint64_t check_sum = 0;
    };
    #pragma pack(pop)

    struct FS_STANDARD_STRUCT{
        IPB_HMJN_FS::PARTITION_HEADER &header_data;
        FS_STANDARD_STRUCT(PARTITION_HEADER &header_data): header_data(header_data){}
    };

    struct GET_DATA_METHOD_STRUCT : virtual public FS_STANDARD_STRUCT{
        GET_DATA_METHOD_STRUCT(PARTITION_HEADER &header_data): FS_STANDARD_STRUCT(header_data){}
        uint64_t get_one_sector_bytes(){
            return this->header_data.one_sector_size;
        }
        uint64_t get_one_block_bytes(){
            return this->get_one_sector_bytes() * this->header_data.one_block_sector_num;
        }
        uint64_t get_one_cluster_bytes(){
            return this->get_one_block_bytes() * this->header_data.one_cluster_block_num;
        }
        uint64_t get_one_partition_bytes(){
            return this->get_one_cluster_bytes() * this->header_data.partition_cluster_size;
        }
    };

    static uint32_t crc_table[256];

    static void init_crc32_table()
    {
        for (uint32_t i = 0; i < 256; i++)
        {
            uint32_t crc = i;

            for (int j = 0; j < 8; j++)
            {
                if (crc & 1)
                    crc = (crc >> 1) ^ 0xEDB88320;
                else
                    crc >>= 1;
            }

            crc_table[i] = crc;
        }
    }

    static uint32_t crc32(const uint8_t* data, size_t length){
        uint32_t crc = 0xFFFFFFFF;

        for (size_t i = 0; i < length; i++){
            uint8_t index = (crc ^ data[i]) & 0xFF;
            crc = (crc >> 8) ^ crc_table[index];
        }
        return crc ^ 0xFFFFFFFF;
    }

    static uint64_t get_check_sum_data(const PARTITION_HEADER header_data){
        uint64_t value = 0;
        value += header_data.fs_version.top;
        value += header_data.fs_version.mid;
        value += header_data.fs_version.low;
        value += header_data.one_sector_size;
        value += header_data.one_block_sector_num;
        value += header_data.one_cluster_block_num;
        value += header_data.partition_cluster_size;
        value += header_data.bitmap_cluster_num;
        value += header_data.bitmap_size;
        value += header_data.back_up_num;
        value += header_data.back_up_list_cluster_num;
        // value += header_data.directory_tree_cluster_num;
        // value += header_data.directory_tree_depth;
        // value += header_data.free_ID_tree_cluster_num;
        // value += header_data.free_ID_tree_depth;
        return value;
    }

    #if OUTPUT_SETTING_DATA_LOG == 1
    void output_datas(const PARTITION_HEADER &header_data){
        using namespace std;
        cout << "magic number : ";
        for(int i = 0; i <= sizeof(header_data.magic_number); i++){
            cout << header_data.magic_number[i];
        }
        cout << endl;
        cout << "back up or main : " << static_cast<uint64_t>(header_data.back_up_or_main) << endl;
        cout << "fs version : " << static_cast<uint64_t>(header_data.fs_version.top) <<
            "." << static_cast<uint64_t>(header_data.fs_version.mid) <<
            "." << static_cast<uint64_t>(header_data.fs_version.low) << endl;
        cout << "partition LBA : " << static_cast<uint64_t>(header_data.partition_LBA) << endl;
        cout << "one sector size : " << static_cast<uint64_t>(header_data.one_sector_size) << endl;
        cout << "one block sector num : " << static_cast<uint64_t>(header_data.one_block_sector_num) << endl;
        cout << "one cluster block num : " << static_cast<uint64_t>(header_data.one_cluster_block_num) << endl;
        cout << "partition cluster size : " << static_cast<uint64_t>(header_data.partition_cluster_size) << endl;
        cout << "bitmap cluster num : " << static_cast<uint64_t>(header_data.bitmap_cluster_num) << endl;
        cout << "bitmap size : " << static_cast<uint64_t>(header_data.bitmap_size) << endl;
        cout << "back up num : " << static_cast<uint64_t>(header_data.back_up_num) << endl;
        cout << "back up list cluster num : " << static_cast<uint64_t>(header_data.back_up_list_cluster_num) << endl;
        cout << "directory tree cluster num : " << static_cast<uint64_t>(header_data.directory_tree_cluster_num) << endl;
        cout << "directory tree depth : " << static_cast<uint64_t>(header_data.directory_tree_depth) << endl;
        cout << "free ID tree cluster num : " << static_cast<uint64_t>(header_data.free_ID_tree_cluster_num) << endl;
        cout << "free ID tree depth : " << static_cast<uint64_t>(header_data.free_ID_tree_depth) << endl;
        cout << "reservation space size : " << static_cast<uint64_t>(header_data.reservation_space_size) << endl;
        cout << "read algorithm num : " << static_cast<uint64_t>(header_data.read_algorithm_num) << endl;
        cout << "write algorithm num : " << static_cast<uint64_t>(header_data.write_algorithm_num) << endl;
        cout << "check sum : " << static_cast<uint64_t>(header_data.check_sum) << endl;
    }
    #endif

    namespace ERR_CODE{
        typedef uint64_t err_code;

        const err_code NO_PROBLEM = 0x0;
        const err_code NOT_MAGIC_NUMBER = 0x1;
        const err_code NOT_MATCH_CHECK_SUM = 0x2;
    }

    namespace SET_UP{
        struct SETUP_PARTITION_STRUCT : public GET_DATA_METHOD_STRUCT{

            SETUP_PARTITION_STRUCT(PARTITION_HEADER &header_data): GET_DATA_METHOD_STRUCT(header_data), FS_STANDARD_STRUCT(header_data){}

            void set_back_up_or_main(uint8_t back_up_or_main){
                this->header_data.back_up_or_main = back_up_or_main;
            }

            void set_top(uint8_t top){
                this->header_data.fs_version.top = top;
            }
            void set_mid(uint8_t mid){
                this->header_data.fs_version.mid = mid;
            }
            void set_low(uint8_t low){
                this->header_data.fs_version.low = low;
            }

            void set_partition_LBA(uint64_t partition_LBA){
                this->header_data.partition_LBA = partition_LBA;
            }
            void set_one_sector_size(uint64_t one_sector_size){
                this->header_data.one_sector_size = one_sector_size;
            }
            void set_one_block_sector_num(uint64_t one_block_sector_num){
                this->header_data.one_block_sector_num = one_block_sector_num;
            }
            void set_one_cluster_block_num(uint64_t one_cluster_block_num){
                this->header_data.one_cluster_block_num = one_cluster_block_num;
            }
            void set_partition_cluster_size(uint64_t partition_cluster_size){
                this->header_data.partition_cluster_size = partition_cluster_size;
            }
            void set_bitmap_cluster_num(uint64_t bitmap_cluster_num){
                this->header_data.bitmap_cluster_num = bitmap_cluster_num;
            }
            void set_back_up_num(uint8_t back_up_num){
                this->header_data.back_up_num = back_up_num;
            }
            void set_back_up_list_cluster_num(uint64_t back_up_list_cluster_num){
                this->header_data.back_up_list_cluster_num = back_up_list_cluster_num;
            }
            void set_directory_tree_cluster_num(uint64_t directory_tree_cluster_num){
                this->header_data.directory_tree_cluster_num = directory_tree_cluster_num;
            }
            void set_directory_tree_depth(uint64_t directory_tree_depth){
                this->header_data.directory_tree_depth = directory_tree_depth;
            }
            void set_free_ID_tree_cluster_num(uint64_t free_ID_tree_cluster_num){
                this->header_data.free_ID_tree_cluster_num = free_ID_tree_cluster_num;
            }
            void set_free_ID_tree_depth(uint64_t free_ID_tree_depth){
                this->header_data.free_ID_tree_depth = free_ID_tree_depth;
            }
            void set_reservation_space_size(uint64_t reservation_space_size){
                this->header_data.reservation_space_size = reservation_space_size;
            }
            void set_read_algorithm_num(uint64_t read_algorithm_num){
                this->header_data.read_algorithm_num = read_algorithm_num;
            }
            void set_write_algorithm_num(uint64_t write_algorithm_num){
                this->header_data.write_algorithm_num = write_algorithm_num;
            }
            void set_check_sum(uint32_t check_sum){
                this->header_data.check_sum = check_sum;
            }
            
        };

        //set user input data
        static void setup_header_data(SETUP_PARTITION_STRUCT &setup_datas){
            {//set partition lba
                uint64_t partition_lba = 0;
                std::cout << "What start partition LBA number?" << std::endl;
                std::cin >> partition_lba;
                setup_datas.set_partition_LBA(partition_lba);
            }
            while(true){//set one sector size
                uint64_t one_sector_size = 0;
                std::cout << "What one sector size bytes?" << std::endl;
                std::cin >> one_sector_size;
                if(!((one_sector_size % 512) && (one_sector_size % 4096)) && one_sector_size != 0){
                    setup_datas.set_one_sector_size(one_sector_size);
                    break;
                }
                std::cout << "Please make it a number divisible by 512 bytes or 4096 bytes" << std::endl;
            }
            {//one block sector num
                uint64_t one_block_sector_num = 0;
                std::cout << "How many sector a  one block?" << std::endl;
                std::cin >> one_block_sector_num;
                setup_datas.set_one_block_sector_num(one_block_sector_num);
            }
            {//one cluster block num
                uint64_t one_cluster_block_num = 0;
                std::cout << "How many block a  one cluster?" << std::endl;
                std::cin >> one_cluster_block_num;
                setup_datas.set_one_cluster_block_num(one_cluster_block_num);
            }
            while(true){//partition cluster size
                uint64_t partition_cluster_size = 0;
                std::cout << "How many cluster a one partition? (5 ~  only)" << std::endl;
                std::cin >> partition_cluster_size;
                if(partition_cluster_size >= 5){
                    setup_datas.set_partition_cluster_size(partition_cluster_size);
                    break;
                }
            }
            while(true){//set back up nums
                uint32_t back_up_num = 1;
                // uint64_t max_backu_num = setup_datas.get_one_cluster_bytes() / 8 / 8 / 2;
                uint64_t max_backu_num = setup_datas.header_data.partition_cluster_size / 8 / 8 + 1;
                //  uint64_t max_backu_num = setup_datas.get_one_cluster_bytes() / 8;
                //バックアップリストで使うアドレス幅は可変長になるため最大保存可能数も可変となる
                //なのでバックアップリスト系統の処理を255前提で書いてる可能性があるのでチェック推奨
                std::cout << "how many backup? (1 ~ " << max_backu_num << " only)" << std::endl;
                std::cin >> back_up_num;
                if((back_up_num >= 1) && (back_up_num <= max_backu_num)){
                    setup_datas.header_data.back_up_num = back_up_num;
                    break;
                }
                std::cout << "input backup nums " << max_backu_num << " over or 0!!" << std::endl;
            }
        }

        //set bitmap datas
        static void setup_bitmap_data(SETUP_PARTITION_STRUCT &setup_datas){
            setup_datas.header_data.bitmap_cluster_num = 1;
            uint64_t address_bit_size = 8;
            uint64_t one_cluster_bytes = setup_datas.get_one_cluster_bytes() / address_bit_size;
            // setup_datas.header_data.bitmap_size = ((setup_datas.header_data.partition_cluster_size + (8-1)) / 8) / one_cluster_bytes;
            setup_datas.header_data.bitmap_size = ((((setup_datas.header_data.partition_cluster_size + (address_bit_size - 1)) / address_bit_size) + one_cluster_bytes - 1) / one_cluster_bytes);
        }

        //set back up data
        static void setup_back_up_data(SETUP_PARTITION_STRUCT &setup_datas){
            setup_datas.header_data.back_up_list_cluster_num = setup_datas.header_data.bitmap_cluster_num + setup_datas.header_data.bitmap_size;
        }

        //set directory data
        static void setup_directory_tree_data(SETUP_PARTITION_STRUCT &setup_datas){
            setup_datas.header_data.directory_tree_cluster_num = setup_datas.header_data.back_up_list_cluster_num + 1;
            setup_datas.header_data.directory_tree_depth = 0;
        }

        //set free ID data
        static void setup_free_ID_tree_data(SETUP_PARTITION_STRUCT &setup_datas){
            setup_datas.header_data.free_ID_tree_cluster_num = setup_datas.header_data.directory_tree_cluster_num + 1;
            setup_datas.header_data.free_ID_tree_depth = 0;
        }

        //set read algorithm
        static void setup_read_algorithm_data(SETUP_PARTITION_STRUCT &setup_datas, uint64_t algorithm){
            setup_datas.header_data.read_algorithm_num = algorithm;
        }

        //set write algorithm
        static void setup_write_algorithm_data(SETUP_PARTITION_STRUCT &setup_datas, uint64_t algorithm){
            setup_datas.header_data.write_algorithm_num = algorithm;
        }

        //set reservation size
        static void setup_reservation_data(SETUP_PARTITION_STRUCT &setup_datas){
            uint64_t end_data_size = sizeof(setup_datas.header_data.read_algorithm_num) +
                sizeof(setup_datas.header_data.write_algorithm_num) +
                sizeof(setup_datas.header_data.check_sum);
            uint64_t beg_size_size = sizeof(PARTITION_HEADER) -
                sizeof(setup_datas.header_data.reservation_space_size) -
                end_data_size;
            #if OUTPUT_SETTING_DATA_LOG == 1
            std::cout << "----------reservation----------" << std::endl;
            std::cout << "header size : " << sizeof(setup_datas.header_data) << std::endl;
            std::cout << "header beg size : " << beg_size_size << std::endl;
            std::cout << "header end size : " << end_data_size << std::endl << std::endl;
            std::cout << "-------------------------------" << std::endl;
            #endif
            setup_datas.header_data.reservation_space_size = setup_datas.get_one_cluster_bytes() - beg_size_size - end_data_size;
        }

        //set check sum data
        static void setup_check_sum_data(PARTITION_HEADER &header_data, check_sum check_sum_type){
            init_crc32_table();
            uint64_t value = get_check_sum_data(header_data);
            switch (check_sum_type){
            case check_sum(CRC32):
                header_data.check_sum = crc32(reinterpret_cast<const uint8_t*>(&value), sizeof(value));
                break;
            default:
                break;
            }
        }

        PARTITION_HEADER setup_header_datas(){
            PARTITION_HEADER header_data;
            SETUP_PARTITION_STRUCT setup_datas(header_data);
            setup_header_data(setup_datas);
            setup_bitmap_data(setup_datas);
            setup_back_up_data(setup_datas);
            setup_directory_tree_data(setup_datas);
            setup_free_ID_tree_data(setup_datas);
            
            setup_read_algorithm_data(setup_datas, 1);
            setup_write_algorithm_data(setup_datas, 1);
            
            setup_reservation_data(setup_datas);
            
            setup_check_sum_data(setup_datas.header_data, CRC32);
            
            #if OUTPUT_SETTING_DATA_LOG == 1
            output_datas(setup_datas.header_data);
            
            #endif


            return header_data;
        }
    }

    namespace FORMAT{
        using namespace std;
        using namespace ERR_CODE;
        namespace fs = filesystem;

        struct FORMAT_IPB_HMJN_FS : virtual public GET_DATA_METHOD_STRUCT {

            FORMAT_IPB_HMJN_FS(PARTITION_HEADER &header_data): GET_DATA_METHOD_STRUCT(header_data), FS_STANDARD_STRUCT(header_data){}

            err_code check_magich_number(){
                if( sizeof(MAGIC_NUMBER) == sizeof(this->header_data.magic_number)){
                    for(int i = 0; i < sizeof(MAGIC_NUMBER); i++){
                        if (!(MAGIC_NUMBER[i] == this->header_data.magic_number[i]))return NOT_MAGIC_NUMBER;
                    }
                }
                return NO_PROBLEM;
            }
            err_code check_sum(){
                uint64_t check_sum_data = get_check_sum_data(header_data);
                uint32_t crc32_check_sum = crc32(reinterpret_cast<uint8_t*>(&check_sum_data), sizeof(check_sum_data));
                if(this->header_data.check_sum == crc32_check_sum)return NO_PROBLEM;
                return NOT_MATCH_CHECK_SUM;
            }



            void write_magic_number(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.magic_number), sizeof(this->header_data.magic_number));
            }
            void write_back_up_or_main(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.back_up_or_main), sizeof(this->header_data.back_up_or_main));
            }
            void write_fs_version(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.fs_version), sizeof(this->header_data.fs_version));
            }
            void write_partition_LBA(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.partition_LBA), sizeof(this->header_data.partition_LBA));
            }
            void write_one_sector_size(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.one_sector_size), sizeof(this->header_data.one_sector_size));
            }
            void write_one_block_sector_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.one_block_sector_num), sizeof(this->header_data.one_block_sector_num));
            }
            void write_one_cluster_block_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.one_cluster_block_num), sizeof(this->header_data.one_cluster_block_num));
            }
            void write_partition_cluster_size(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.partition_cluster_size), sizeof(this->header_data.partition_cluster_size));
            }
            void write_bitmap_cluster_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.bitmap_cluster_num), sizeof(this->header_data.bitmap_cluster_num));
            }
            void write_bitmap_size(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.bitmap_size), sizeof(this->header_data.bitmap_size));
            }
            void write_back_up_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.back_up_num), sizeof(this->header_data.back_up_num));
            }
            void write_back_up_list_cluster_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.back_up_list_cluster_num), sizeof(this->header_data.back_up_list_cluster_num));
            }
            void write_directory_tree_cluster_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.directory_tree_cluster_num), sizeof(this->header_data.directory_tree_cluster_num));
            }
            void write_directory_tree_depth(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.directory_tree_depth), sizeof(this->header_data.directory_tree_depth));
            }
            void write_free_ID_tree_cluster_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.free_ID_tree_cluster_num), sizeof(this->header_data.free_ID_tree_cluster_num));
            }
            void write_free_ID_tree_depth(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.free_ID_tree_depth), sizeof(this->header_data.free_ID_tree_depth));
            }
            void seek_reservation_space_size(fstream &file){
                file.seekp(this->header_data.reservation_space_size, ios::cur);
            }
            void write_read_algorithm_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.read_algorithm_num), sizeof(this->header_data.read_algorithm_num));
            }
            void write_write_algorithm_num(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.write_algorithm_num), sizeof(this->header_data.write_algorithm_num));
            }
            void write_check_sum(fstream &file){
                file.write(reinterpret_cast<const char*>(&this->header_data.check_sum), sizeof(this->header_data.check_sum));
            }

            void write_super_block(fstream &file){
                this->write_magic_number(file);
                this->write_back_up_or_main(file);
                this->write_fs_version(file);
                this->write_partition_LBA(file);
                this->write_one_sector_size(file);
                this->write_one_block_sector_num(file);
                this->write_one_cluster_block_num(file);
                this->write_partition_cluster_size(file);
                this->write_bitmap_cluster_num(file);
                this->write_bitmap_size(file);
                this->write_back_up_num(file);
                this->write_back_up_list_cluster_num(file);
                this->write_directory_tree_cluster_num(file);
                this->write_directory_tree_depth(file);
                this->write_free_ID_tree_cluster_num(file);
                this->write_free_ID_tree_depth(file);
                this->seek_reservation_space_size(file);
                this->write_read_algorithm_num(file);
                this->write_write_algorithm_num(file);
                this->write_check_sum(file);
                
            }
            void write_make_bitmap_data(fstream &file){
                vector<uint8_t>bitmap_data((this->header_data.partition_cluster_size + ((sizeof(uint8_t) * 8) - 1)) / (sizeof(uint8_t) * 8), 0);
                uint64_t write_bit_count = 0;
                
                //write susper block bit
                bitmap_data.at(write_bit_count / (sizeof(uint8_t) * 8)) |= 1 << write_bit_count % (sizeof(uint8_t) * 8);
                write_bit_count++ ;

                //write bitmap data bit
                for(;write_bit_count <= this->header_data.bitmap_size;write_bit_count++){
                    bitmap_data.at(write_bit_count / (sizeof(uint8_t) * 8)) |= 1 << write_bit_count % (sizeof(uint8_t) * 8);
                }

                //write back up list
                bitmap_data.at(write_bit_count / (sizeof(uint8_t) * 8)) |= 1 << write_bit_count % (sizeof(uint8_t) * 8);
                write_bit_count++ ;

                //write directory tree
                bitmap_data.at(write_bit_count / (sizeof(uint8_t) * 8)) |= 1 << write_bit_count % (sizeof(uint8_t) * 8);
                write_bit_count++ ;

                //write free ID tree
                bitmap_data.at(write_bit_count / (sizeof(uint8_t) * 8)) |= 1 << write_bit_count % (sizeof(uint8_t) * 8);
                write_bit_count++ ;
                
                //backupの位置にフラグを立てる
                if(this->header_data.back_up_num > 0){
                    bitmap_data.at(bitmap_data.size() - 1) |= 1 << this->header_data.partition_cluster_size % (sizeof(uint8_t) * 8);
                    #if OUTPUT_FORMAT_LOG == 1
                    cout << "-------------bitmap flag data-----------" << endl;
                    #endif
                    for(int i = 1; i < this->header_data.back_up_num; i++){
                        uint64_t get_one_partition_bytes = this->header_data.partition_cluster_size;
                        write_bit_count = get_one_partition_bytes - (get_one_partition_bytes) / (i+1);
                        bitmap_data.at(write_bit_count / (sizeof(uint8_t) * 8)) |= 1 << write_bit_count % (sizeof(uint8_t) * 8);
                        cout << "write bit : " << write_bit_count % (sizeof(uint8_t) * 8) << endl;
                        #if OUTPUT_FORMAT_LOG == 1
                        cout << "bit map at : " << write_bit_count / (sizeof(uint8_t) * 8) << endl;
                        cout << "bit num :" << write_bit_count % (sizeof(uint8_t)) * 8 << endl;
                        #endif
                    }
                    #if OUTPUT_FORMAT_LOG == 1
                    cout << "----------------------------------------" << endl;
                    #endif
                }

                file.seekp((this->header_data.bitmap_cluster_num * this->get_one_cluster_bytes()), ios::beg);

                file.write(reinterpret_cast<const char*>(bitmap_data.data()), bitmap_data.size() * sizeof(uint8_t));

                #if OUTPUT_FORMAT_LOG == 1
                cout << "-------------bitmap datas-----------" << endl;
                for(int i =  0; i < bitmap_data.size(); i++){
                    cout << "(" << i << ")" << bitset<8 * sizeof(uint8_t)>(bitmap_data.at(i));
                }
                cout << "\n------------------------------------" << endl;
                #endif
            }
            void write_backups(fstream &file){
                if(this->header_data.back_up_num > 0){
                    this->header_data.back_up_or_main = 1;
                    #if OUTPUT_FORMAT_LOG == 1
                    cout << "-------------backup datas-----------" << endl;
                    cout << "seekp size : " << this->header_data.partition_cluster_size - 1 << endl;
                    #endif
                    file.seekp(((this->header_data.partition_cluster_size - 1) * this->get_one_cluster_bytes()), ios::beg);
                    this->write_super_block(file);
                    for(int i = 1; i < this->header_data.back_up_num; i++){
                        #if OUTPUT_FORMAT_LOG == 1
                        cout << "seekp size : " << this->header_data.partition_cluster_size - (this->header_data.partition_cluster_size) / (i+1) << endl;
                        #endif
                        file.seekp(((this->header_data.partition_cluster_size) - (this->header_data.partition_cluster_size) / (i+1) - 1)* this->get_one_cluster_bytes());
                        this->write_super_block(file);
                    }
                    #if OUTPUT_FORMAT_LOG == 1
                    cout << "------------------------------------" << endl;
                    #endif
                    this->header_data.back_up_or_main = 0;
                }
            }
            void write_back_up_list(fstream &file){
                if(this->header_data.back_up_num > 0){
                    vector<uint64_t> backup_lists(this->header_data.back_up_num, 0);
                    backup_lists.at(0) = this->header_data.partition_cluster_size - 1;
                    #if OUTPUT_FORMAT_LOG == 1
                    cout << "-------------backup list-----------" << endl;
                    cout << "seekp size : " << this->header_data.partition_cluster_size - 1 << endl;
                    #endif
                    for(int i = 1; i < this->header_data.back_up_num; i++){
                        backup_lists.at(i) = this->header_data.partition_cluster_size - (this->header_data.partition_cluster_size) / (i+1);
                        #if OUTPUT_FORMAT_LOG == 1
                        cout << "seekp size : " << backup_lists.at(i) << endl;
                        #endif
                    }
                    file.seekp(this->header_data.back_up_list_cluster_num * this->get_one_cluster_bytes(), ios::beg);
                    file.write(reinterpret_cast<const char*>(backup_lists.data()), backup_lists.size() * sizeof(uint64_t));
                    #if OUTPUT_FORMAT_LOG == 1
                    cout << "-----------------------------------" << endl;
                    #endif
                }
            }
        };

        static ERR_CODE::err_code check_header_datas(FORMAT_IPB_HMJN_FS & format_data){
            if(NO_PROBLEM != format_data.check_magich_number())return NOT_MAGIC_NUMBER;
            if(NO_PROBLEM != format_data.check_sum())return NOT_MATCH_CHECK_SUM;


            return NO_PROBLEM;
        }

        static void write_FS_datas(FORMAT_IPB_HMJN_FS &format_data){
            fstream file(output_filename, ios::out | ios::binary);
            format_data.write_super_block(file);
            format_data.write_make_bitmap_data(file);
            format_data.write_backups(file);
            format_data.write_back_up_list(file);

        }

        err_code format_fs(IPB_HMJN_FS::PARTITION_HEADER &header_data){
            FORMAT_IPB_HMJN_FS format_data = {header_data};
            
            {//err check
                cout << "check header data" << endl;
                ERR_CODE::err_code err = check_header_datas(format_data);
                if(err){
                    cout << "header data err !error code! : " << err << endl;
                    return err;
                }
                cout << "no header data err" << endl;
            }
            write_FS_datas(format_data);
            return NO_PROBLEM;
        }
    }

    namespace FS_TYPES{

        enum DATA_TYPE{
            dir = 1,
            file = 2,
            simbolic = 3,
            socket = 4,
            division = 5,
        };

        
        #pragma pack(push, 1)
        struct DIR_ENTRY{
            uint8_t data_type = 0;
            uint64_t ID = 0;
            uint64_t parent_ID = 0;
            uint64_t next_sibling_ID = 0;
            uint64_t prev_sibling_ID = 0;
            union{
                struct{
                    uint8_t pad[206] = {0};
                };
                struct{
                    uint64_t first_child_ID;
                    // uint8_t pad[247 - 25 - 8];
                }dir;
                struct{
                    uint64_t bit_flag;
                    uint64_t size;
                    uint64_t file_address;
                    // uint8_t pad[247 - 25 - 16];
                }file;
                struct{
                    uint64_t target_ID;
                }simbolic;
                struct{
                    uint64_t bit_flag;
                    uint64_t block_address;
                    uint64_t secter_address;
                }division;
            }another_type_data;
            uint64_t mode = 0;
            uint64_t last_updata_time = 0;
            uint8_t name_size = 0;
            uint8_t name[256] = {0};
        };
        #pragma pack(pop)

        struct DIR_ENTRY_INTERFACE{
            DIR_ENTRY &dir_entry;
            
            DIR_ENTRY_INTERFACE(DIR_ENTRY &dir_entry_data): dir_entry(dir_entry_data){}

            void set_name(const uint8_t* name_arr){
                for(uint64_t i = 0; i < this->dir_entry.name_size && i < 256; i++){
                    std::cout << "test : " << i << std::endl;
                    this->dir_entry.name[i] = name_arr[i];
                }
            }
        };
    }

    namespace EXECUTION{

        struct return_report{

        };

        namespace BIT_MAP_FUNCTIONS{
            using namespace std;

            const uint64_t bitmap_bit_size = 3;

            struct BITMAP_DATA_STRUCT{
                uint64_t bitsize;//乗数を入れる
                uint64_t bits = 0;
                uint8_t free_count = 0;
            };

            struct SELECT_BITMAP_DATA{
                uint64_t select_size = 0;
                uint64_t start_index = 0;
                uint64_t start_offset = 0;
            };

            struct RELOAD_STRUCT{
                uint64_t start_index = 0;
                uint64_t reload_index_num = 0;
            };

            struct BIT_MAP_FUNCTION: virtual public GET_DATA_METHOD_STRUCT{
                
                vector<uint8_t> bitmap;
                vector<BITMAP_DATA_STRUCT> bitmap_data;

                // // 戻り値として論理アドレス位置を返すようにしたい
                // // 断片化したときのことも考えてプログラムを組む必要がある
                // bitmapを配列として(型はBITMAP_DATA_STRUCT)
                vector<BITMAP_DATA_STRUCT> set_bitmap_datas(){
                    vector<BITMAP_DATA_STRUCT> bitmaps;
                    bitmaps.resize(this->bitmap.size());
                    uint64_t zero_count = 0;
                    uint64_t bit_shift_count = 0;
                    uint64_t one_scan_bit_num = __builtin_popcount(~0);
                    BITMAP_DATA_STRUCT bitmap_struct_data;
                    for(int index = 0; index < this->bitmap.size(); index++){
                        bitmap_struct_data.bitsize = bitmap_bit_size;
                        uint64_t one_bitmap_bit_size = 1 << bitmap_struct_data.bitsize;
                        uint8_t zero_count = __builtin_popcount(~bitmap.at(index));
                        bitmap_struct_data.free_count = one_bitmap_bit_size - (one_scan_bit_num - zero_count);
                        bitmap_struct_data.bits = bitmap.at(index);
                        bitmaps.at(index) = bitmap_struct_data;
                        #if OUTPUT_BITMAP_DATA_LOG == 1
                            cout << "----------bitmap data struct log---------" << endl;
                            cout << "index num : " << index << endl;
                            cout << "bitsize : " << bitmaps.at(index).bitsize << endl;
                            cout << "bits : " << static_cast<uint64_t>(bitmaps.at(index).bits) << endl;
                            cout << "real bits : " << bitset<sizeof(uint64_t)>(bitmap.at(index)) << endl;
                            cout << "free count : " << static_cast<uint64_t>(bitmaps.at(index).free_count) << endl;
                            cout << "-------------------------------------------" << endl;
                        #endif
                    }
                    return bitmaps;
                }

                BIT_MAP_FUNCTION(PARTITION_HEADER &header_data): GET_DATA_METHOD_STRUCT(header_data)
                , FS_STANDARD_STRUCT(header_data){
                    fstream file(IPB_HMJN_FS::output_filename, ios::in | ios::out | ios::binary);
                    bitmap.resize((this->header_data.partition_cluster_size - ((sizeof(uint8_t) * 8) - 1)) / (sizeof(uint8_t) * 8) + 1, 0);
                    file.seekg(this->header_data.bitmap_cluster_num * this->get_one_cluster_bytes(), ios::beg);
                    file.read(reinterpret_cast<char*>(bitmap.data()), bitmap.size() * sizeof(uint8_t));
                    #if OUTPUT_BITMAP_DATA_LOG == 1
                    for(const auto& bits : bitmap){
                        cout << "real bits : " << bitset<sizeof(uint8_t) * 8>(bits) << endl;
                    }
                    #endif
                    bitmap_data = set_bitmap_datas();
                }

                //連続した空き位置をビットマップから探す
                //断片化を前提の空き探索は未実装
                SELECT_BITMAP_DATA get_bitmap_data(uint64_t select_size){
                    const uint64_t one_bit_size = 1 << bitmap_data.at(0).bitsize;
                    uint64_t zero_count = 0;
                    uint64_t start_index = 0;
                    uint64_t start_offset = 0;
                    uint64_t one_bitmap_bit_size = 1 << bitmap_bit_size;
                    uint64_t one_scan_bit_num = __builtin_ctzll(0ULL);
                    SELECT_BITMAP_DATA result_bitmap_data;
                    for(int index = 0; index < bitmap_data.size(); index++){
                        if(bitmap_data.at(index).free_count >= select_size || one_bit_size <= select_size){
                            //ビットマップを実際に比較をする
                            
                            //bit反転をしているため空いている場所が0ではなく1として扱われるようになる
                            uint64_t bits = ~bitmap_data.at(index).bits;
                            for(uint64_t scan_count = 0; scan_count < one_bit_size;scan_count++){
                                
                                uint64_t shift_size = 0;
                                
                                //bitmapの調べる位置が埋まっていた場合の処理
                                if((bits & 1) == 0){
                                    zero_count = 0;
                                    
                                    //bitmapがすべて探索されたときの処理
                                    if(bits == 0){
                                        start_offset = 0;
                                        start_index = index;
                                        break;
                                    }
                                    
                                    shift_size = __builtin_ctzll(bits);
                                    // bits >> shift_size;
                                    start_offset = scan_count + shift_size;
                                    
                                }

                                //bitmapの調べる位置が開いていた場合の処理
                                else{
                                    shift_size = one_bitmap_bit_size - (one_scan_bit_num - __builtin_ctzll(~bits));
                                    zero_count += shift_size;
                                    // bits >> shift_size;
                                }
                                bits = bits >> shift_size;
                                scan_count += shift_size;
                            }

                            //欲しい空き容量が見つかった場合の処理
                            if(zero_count >= select_size){
                                result_bitmap_data.select_size = select_size;
                                result_bitmap_data.start_index = start_index;
                                result_bitmap_data.start_offset = start_offset;
                                #if OUTPUT_BITMAP_DATA_LOG == 1
                                cout << "----------get bitmap data---------" << endl;
                                cout << "select size : " << result_bitmap_data.select_size << endl;
                                cout << "start index : " << result_bitmap_data.start_index << endl;
                                cout << "start offset : " << result_bitmap_data.start_offset << endl;
                                cout << "----------------------------------" << endl;
                                #endif
                                break;
                            }
                        }
                        else{
                            start_index = index;
                        }
                    }
                    return result_bitmap_data;
                }

                RELOAD_STRUCT write_bit_flag(SELECT_BITMAP_DATA select_data){
                    vector<SELECT_BITMAP_DATA>reload_log;
                    uint64_t flag_bit = ((1 << select_data.select_size) - 1) << select_data.start_offset;
                    uint64_t scan_bit_size = 1 << bitmap_data.at(0).bitsize;
                    uint64_t loop_num = (select_data.select_size + (scan_bit_size - 1)) / scan_bit_size;
                    uint64_t write_bit_nums = 0;
                    uint64_t reload_index_num = 0;
                    uint64_t count = 1;
                    #if OUTPUT_BITMAP_DATA_LOG == 1
                    cout << "----------write bit flag----------" << endl;
                    cout << "select size : " << select_data.select_size << endl;
                    cout << "start index : " << select_data.start_index << endl;
                    cout << "start offset : " << select_data.start_offset << endl;
                    #endif
                    uint64_t bits = bitmap_data.at(select_data.start_index).bits;
                    
                    bits |= ((1 << select_data.select_size) - 1) << __builtin_ctzll(~bits);

                    bitmap_data.at(select_data.start_index).bits = bits;
                    bitmap_data.at(select_data.start_index).free_count = __builtin_ctzll(bits);
                    bitmap.at(select_data.start_index) = bits;
                    write_bit_nums += (1 << bitmap_data.at(select_data.start_index).bitsize) - select_data.start_offset;

                    if(~bitmap_data.at(select_data.start_index).bits == 0){

                        for(count = 1; (write_bit_nums + (1 << bitmap_data.at(select_data.start_index + count).bitsize)) < select_data.select_size; count++){
                            uint64_t bits = bitmap_data.at(select_data.start_index + count).bits;
                            bits |= (1 << (1 << bitmap_data.at(select_data.start_index + count).bitsize)) - 1 << __builtin_ctzll(~bits);
                            bitmap_data.at(select_data.start_index + count).bits = bits;
                            bitmap_data.at(select_data.start_index + count).free_count = __builtin_ctzll(bits);
                            bitmap.at(select_data.start_index + count) = bits;
                            write_bit_nums += (1 << bitmap_data.at(select_data.start_index + count).bitsize);
                        }
                        
                        bits = bitmap_data.at(select_data.start_index + count).bits;
                        bits |= ((1 << (select_data.select_size - write_bit_nums)) - 1) << __builtin_ctzll(~bits);
                        bitmap_data.at(select_data.start_index + count).bits = bits;
                        bitmap_data.at(select_data.start_index + count).free_count = __builtin_ctzll(bits);
                        bitmap.at(select_data.start_index + count) = bits;
                    }
                        RELOAD_STRUCT data_log;
                        data_log.start_index = select_data.start_index;
                        data_log.reload_index_num = loop_num;
                        #if OUTPUT_BITMAP_DATA_LOG == 1
                        cout << "----------------------------------" << endl;
                        #endif
                    return data_log;
                }

                void reload_bitmap(RELOAD_STRUCT reload_data, fstream& file){
                    file.seekp((this->header_data.bitmap_cluster_num * this->get_one_cluster_bytes()) + ((1 << bitmap_data.at(0).bitsize) * reload_data.start_index), ios::beg);
                    file.write(reinterpret_cast<char *>(bitmap.data() + ((sizeof(uint8_t) * 8) * reload_data.start_index)), ((sizeof(uint8_t) * 8) / (1 << bitmap_data.at(0).bitsize)) * (reload_data.reload_index_num + 1));
                    #if OUTPUT_BITMAP_DATA_LOG == 1
                    cout << "----------reload bitmap----------" << endl;
                    cout << "seek size : " << (this->header_data.bitmap_cluster_num * this->get_one_cluster_bytes()) + ((1 << bitmap_data.at(0).bitsize) * reload_data.start_index) << endl;
                    cout << "write data : " << ((sizeof(uint8_t) * 8) * reload_data.start_index) << endl;
                    cout << "write byte num : " << ((sizeof(uint8_t) * 8) / (1 << bitmap_data.at(0).bitsize)) * (reload_data.reload_index_num + 1) << endl;
                    cout << "---------------------------------" << endl;
                    #endif
                }

                RELOAD_STRUCT delete_bit_flag(SELECT_BITMAP_DATA select_data){
                    const uint64_t one_bitmap_bit_size = 1 << BIT_MAP_FUNCTIONS::bitmap_bit_size;
                    const uint64_t loop_num = (select_data.select_size  + (one_bitmap_bit_size - 1)) / one_bitmap_bit_size;
                    uint64_t bit_shift_num = 0;
                    bit_shift_num = 0;
                    uint8_t bits = ~bitmap_data.at(select_data.start_index).bits;
                    uint64_t delete_bit_num = 0;

                    bits |= ((1 << ((1 << bitmap_data.at(select_data.start_index).bitsize) - select_data.start_offset)) - 1) << select_data.start_offset;
                    delete_bit_num += (1 << bitmap_data.at(select_data.start_index).bitsize) - select_data.start_offset;

                    bitmap_data.at(select_data.start_index).bits = ~bits;
                    bitmap_data.at(select_data.start_index).free_count = __builtin_popcount(~bits);
                    bitmap.at(select_data.start_index) = ~bits;
                    
                    #if OUTPUT_BITMAP_DATA_LOG == 1
                    std::cout << "----------delete bit flag----------" << std::endl;
                    std::cout << "select_size : " << select_data.select_size << std::endl;
                    std::cout << "start_index : " << select_data.start_index << std::endl;
                    std::cout << "start_offset : " << select_data.start_offset << std::endl;
                    std::cout << "loop num : " << loop_num << std::endl;
                    std::cout << "delete bit num : " << delete_bit_num << std::endl;
                    #endif
                    for(uint64_t i = 1; i < loop_num; i++){
                        bits = ~bitmap_data.at(select_data.start_index + i).bits;
                        bits |= (1 << (1 << bitmap_data.at(select_data.start_index + i).bitsize)) - 1;
                        delete_bit_num += (1 << bitmap_data.at(select_data.start_index + i).bitsize);
                        
                        bitmap_data.at(select_data.start_index + i).bits = ~bits;
                        bitmap_data.at(select_data.start_index + i).free_count = __builtin_popcount(bits);
                        bitmap.at(select_data.start_index + i) = ~bits;

                        #if OUTPUT_BITMAP_DATA_LOG == 1
                        std::cout << "~~~~~~~~~~index : " << select_data.start_index + i << "~~~~~~~~~~" << endl;
                        std::cout << "delete bit num : " << delete_bit_num << std::endl;
                        std::cout << "real bit log : " <<  bitset<8>(~bits) << std::endl;
                        std::cout << "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~" << endl;
                        #endif
                    }
                    bits = ~bitmap_data.at(select_data.start_index + loop_num).bits;

                    bits |= (((1 << bitmap_data.at(select_data.start_index + loop_num).bitsize) - 1) << (bitmap_data.at(select_data.start_index + loop_num).bitsize) - select_data.select_size - delete_bit_num) - 1;
                    delete_bit_num += (select_data.select_size - delete_bit_num);
                    bitmap_data.at(select_data.start_index + loop_num).bits = ~bits;
                    bitmap_data.at(select_data.start_index + loop_num).free_count = __builtin_popcount(bits);
                    bitmap.at(select_data.start_index + loop_num) = ~bits;
                    RELOAD_STRUCT reload_data;
                    reload_data.start_index = select_data.start_index;
                    reload_data.reload_index_num = loop_num + 1;
                    // reload_bitmap(reload_data);
                    #if OUTPUT_BITMAP_DATA_LOG == 1
                    std::cout << "real bit log : " <<  bitset<8>(~bits) << std::endl;
                    std::cout << "delete bit num : " << delete_bit_num << std::endl;
                    std::cout << "-----------------------------------" << std::endl;
                    #endif
                    return reload_data;
                }

                uint64_t cast_address(SELECT_BITMAP_DATA result_data){
                    uint64_t one_bitmap_bit_size = 1 << bitmap_bit_size;
                    uint64_t select_address = (result_data.start_index * one_bitmap_bit_size) + result_data.start_offset;
                    return select_address;
                }

                SELECT_BITMAP_DATA cast_bitmap_data(uint64_t address){
                    SELECT_BITMAP_DATA result_data;
                    result_data.select_size = 1;
                    result_data.start_index = address / 8;
                    result_data.start_index = address % 8;
                    return result_data;
                }

            };
        }

        namespace TREE_FUNCTIONS{
            using namespace std;

            struct result_tree_data{
                //↓ツリーのたどるべき順路を表している
                vector<uint64_t>scan_order;
                //↓最終的な位置
                uint64_t address;
            };
            struct TREE_FUNCTION: virtual public GET_DATA_METHOD_STRUCT
            , virtual public BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION{

                TREE_FUNCTION(PARTITION_HEADER &header_data): GET_DATA_METHOD_STRUCT(header_data)
                , FS_STANDARD_STRUCT(header_data)
                , BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION(header_data){}
                // ツリーを更新する関数
                // どのツリーを更新したか？というのは知らないためスーパーブロックの更新は別で作る必要がある
                BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA reload_tree(uint64_t top_cluster_num, fstream &file){
                    BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA select_bit_data = this->get_bitmap_data(1);
                    uint64_t write_address_num = this->cast_address(select_bit_data);
                    uint64_t one_bitmap_bit_size = 1 << BIT_MAP_FUNCTIONS::bitmap_bit_size;
                    file.seekp(write_address_num * this->get_one_cluster_bytes(), ios::beg);
                    file.write(reinterpret_cast<char*>(&top_cluster_num), sizeof(uint64_t));
                    #if OUTPUT_TREE_DATA_LOG == 1
                    std::cout << "----------tree function----------" << std::endl;
                    std::cout << "write address num : " << write_address_num << std::endl;
                    std::cout << "---------------------------------" << std::endl;
                    #endif

                    return select_bit_data;
                }

                // uint64_t delete_tree(){
                //     return 0;
                // }

                result_tree_data scan_tree_data(const uint64_t top_tree_address, const uint64_t tree_deep, fstream& file){
                    result_tree_data result_data;
                    result_data.address = top_tree_address;
                    // result_data.scan_order.resize(tree_deep);
                    if(!tree_deep)return result_data;
                    result_data.scan_order.resize(tree_deep);
                    uint64_t before_address = 0;
                    uint64_t now_scan_address = top_tree_address;
                    const uint64_t have_address_num = this->get_one_cluster_bytes() / sizeof(uint64_t);
                    vector<uint64_t>address_nums(this->get_one_cluster_bytes() / sizeof(uint64_t) ,0);
                    for(uint64_t i = 0; i < tree_deep; i++){
                        file.seekg((this->get_one_cluster_bytes() * now_scan_address), ios::beg);
                        file.read(reinterpret_cast<char *>(address_nums.data()), address_nums.size() * sizeof(uint64_t));
                        uint64_t scan_low = 0;
                        uint64_t scan_top = have_address_num;
                        uint64_t scan_mid = (scan_top - scan_low) / 2;
                        uint64_t j = 0;
                        //要素が存在している位置を特定するための二分探索
                        for(j = 0; scan_top != scan_mid && scan_low != scan_mid || address_nums.size() < j; j++){
                            #if OUTPUT_TREE_DATA_LOG == 1
                            std::cout << "top : " << scan_top << std::endl;
                            std::cout << "mid : " << scan_mid << std::endl;
                            std::cout << "low : " << scan_low << std::endl;
                            #endif
                            if(address_nums.at(scan_mid) == 0) scan_top = scan_mid;
                            else scan_low = scan_mid;
                            scan_mid = (scan_top - scan_low) / 2 + scan_low;
                        }
                        result_data.scan_order.at(i) = scan_mid;
                        // address_nums.at(scan_top) = now_scan_address;
                        now_scan_address = address_nums.at(scan_mid);
                        #if OUTPUT_TREE_DATA_LOG == 1
                        std::cout << "----------tree function----------" << std::endl;
                        std::cout << "now scan address : " << now_scan_address << endl;
                        std::cout << "---------------------------------" << std::endl;
                        #endif
                    }
                    result_data.address = now_scan_address;
                    return result_data;
                }

            };
        }

        namespace DIRECTORY_TREE{
            using namespace std;
            struct TREE_STRUCT{
                bool existence_dir = false;
                uint64_t dir_tree_cluster = 0;
            };

            struct FS_DIRECTORY: virtual public GET_DATA_METHOD_STRUCT, virtual public TREE_FUNCTIONS::TREE_FUNCTION{

                FS_DIRECTORY(PARTITION_HEADER &header_data):GET_DATA_METHOD_STRUCT(header_data)
                , FS_STANDARD_STRUCT(header_data)
                , TREE_FUNCTIONS::TREE_FUNCTION(header_data)
                , BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION(header_data){}

                void reload_dir_tree(fstream& file){
                    IPB_HMJN_FS::EXECUTION::BIT_MAP_FUNCTIONS::SELECT_BITMAP_DATA new_dir_tree_data = reload_tree(this->header_data.directory_tree_cluster_num, file);
                    this->header_data.directory_tree_cluster_num = this->cast_address(new_dir_tree_data);
                    this->header_data.directory_tree_depth++;
                }

                // // 本当にIDを得たかったの？
                // // IDはディレクトリエントリだけでしか使わないからこれがあると
                // // ツリー探索用のクラスにした意味がなくなるから
                // // ほかの用途で作ろとしたのでは？
                uint64_t cast_cluster_ID(const TREE_FUNCTIONS::result_tree_data& data){
                    uint64_t result_address = 0;
                    for(uint32_t i = 0; i < data.scan_order.size(); i++){
                        result_address += pow((this->get_one_cluster_bytes() / sizeof(uint64_t)), i) * data.scan_order.at(data.scan_order.size() - 1 + i);
                    }
                    return result_address;
                };
                
                // dir treeの
                uint64_t scan_dir_entry(fstream& file){
                    TREE_FUNCTIONS::result_tree_data dir_tree_data = scan_tree_data(this->header_data.directory_tree_cluster_num, this->header_data.directory_tree_depth, file);
                    uint64_t loop_num = this->get_one_cluster_bytes() / sizeof(FS_TYPES::DIR_ENTRY);
                    uint64_t loop_count = 1;
                    FS_TYPES::DIR_ENTRY get_dir_entry_data;
                    file.seekg(dir_tree_data.address * this->get_one_block_bytes(), ios::beg);
                    bool true_or_flase = true;
                    for(loop_count = 1; loop_count <= loop_num || true_or_flase; loop_count++){
                        file.read((char*)&get_dir_entry_data, sizeof(get_dir_entry_data));
                        true_or_flase = get_dir_entry_data.data_type != 0;
                    }
                    if(!true_or_flase){
                        //空いているアドレスが次のツリー要素にあるかどうか？
                        //空きアドレスが次のツリーにある場合はツリーを新しく更新する
                        if(loop_count == loop_num){
                            uint64_t one_tree_child_nums = this->get_one_cluster_bytes() / sizeof(uint64_t);
                            for(uint64_t i = 0; i < dir_tree_data.scan_order.size(); i++){
                                // ツリーの繰り上がりが発生するかのチェック
                                if(dir_tree_data.scan_order.at(dir_tree_data.scan_order.size() - 1 - i) >= one_tree_child_nums){
                                    dir_tree_data.scan_order.at(dir_tree_data.scan_order.size() - 1 - i) = 0;
                                    // 探索中のツリー位置が既存のツリーを越しているかどうかを見る
                                    if(i >= (dir_tree_data.scan_order.size() - 1)){
                                        reload_dir_tree(file);
                                        dir_tree_data.scan_order.insert(dir_tree_data.scan_order.begin(), 1);
                                    }
                                    else{
                                        dir_tree_data.scan_order.at(dir_tree_data.scan_order.size() - i) += 1;
                                    }
                                }
                            }
                        }
                    }
                    uint64_t ID = cast_cluster_ID(dir_tree_data);
                    ID += loop_num;
                    return ID;
                }

                void addtion_dir_entry(fstream& file, uint64_t write_ID, const FS_TYPES::DIR_ENTRY& add_entry){
                    const uint64_t one_tree_child_nums = this->get_one_cluster_bytes() / sizeof(uint64_t);
                    uint64_t tree_search_num = 0;
                    uint64_t next_analysis_num = this->header_data.directory_tree_cluster_num;
                    uint64_t search_address = this->header_data.directory_tree_cluster_num;
                    #if OUTPUT_DIRECTORY_TREE_LOG == 1
                        cout << "----------addtion_dir_entry----------" << endl;
                        cout << "one tree child nums : " << one_tree_child_nums << endl;
                    #endif
                    for(uint64_t i = 0; i < this->header_data.directory_tree_depth; i++){
                        tree_search_num = next_analysis_num / one_tree_child_nums;
                        next_analysis_num = next_analysis_num % one_tree_child_nums;
                        file.seekg((search_address * this->get_one_cluster_bytes()) + tree_search_num * sizeof(uint64_t), ios::beg);
                        file.read(reinterpret_cast<char*>(&search_address), sizeof(uint64_t));
                        #if OUTPUT_DIRECTORY_TREE_LOG == 1
                            cout << "loop num : " << i << endl;
                            cout << "tree_search_num : " << tree_search_num << endl;
                            cout << "next_analysis_num : " << next_analysis_num << endl;
                            cout << "search_address : " << search_address << endl;
                        #endif
                    }
                    next_analysis_num = next_analysis_num % (this->get_one_cluster_bytes() / sizeof(FS_TYPES::DIR_ENTRY));
                    file.seekp((search_address * this->get_one_cluster_bytes()) + (sizeof(FS_TYPES::DIR_ENTRY) * next_analysis_num));
                    file.write((char*)&add_entry, sizeof(FS_TYPES::DIR_ENTRY));
                    cout << "entry type size : " << sizeof(FS_TYPES::DIR_ENTRY) << endl;
                    #if OUTPUT_DIRECTORY_TREE_LOG == 1
                        cout << "write byte num : " << (search_address * this->get_one_cluster_bytes()) + (sizeof(FS_TYPES::DIR_ENTRY) * next_analysis_num) << endl;
                        cout << "-------------------------------------" << endl;
                    #endif

                }
            };
        }

        struct FS_FUNCTIONS: virtual public FS_STANDARD_STRUCT, public DIRECTORY_TREE::FS_DIRECTORY, virtual public TREE_FUNCTIONS::TREE_FUNCTION, virtual public BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION{
            FS_FUNCTIONS(PARTITION_HEADER &header_data): FS_STANDARD_STRUCT(header_data)
            , TREE_FUNCTIONS::TREE_FUNCTION(header_data)
            , BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION(header_data)
            , FS_DIRECTORY(header_data)
            , GET_DATA_METHOD_STRUCT(header_data){}

            struct{
                void create_dir_entry(FS_TYPES::DIR_ENTRY entry_data){

                }
                
                void delete_dir_entry(){}

            }dir_operation;

        };

        class EXECUTION_STRUCT : private FORMAT::FORMAT_IPB_HMJN_FS, virtual public GET_DATA_METHOD_STRUCT, public FS_FUNCTIONS{
            private:



            public:
            EXECUTION_STRUCT(PARTITION_HEADER &header_data): FORMAT::FORMAT_IPB_HMJN_FS(header_data)
            , GET_DATA_METHOD_STRUCT(header_data)
            , TREE_FUNCTIONS::TREE_FUNCTION(header_data)
            , FS_STANDARD_STRUCT(header_data)
            , FS_FUNCTIONS(header_data)
            , BIT_MAP_FUNCTIONS::BIT_MAP_FUNCTION(header_data){}
            
            void create_file(FS_TYPES::DIR_ENTRY &direntry){

            }

            void test_write_super_block(std::fstream &file){
                file.seekp(0,std::ios::beg);
                this->write_super_block(file);
            }

        };

    }


}
