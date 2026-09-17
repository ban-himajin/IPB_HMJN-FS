
static mut CRC_TABLE: [u32; 256] = [0; 256];
static mut ALGORITHM_CRC_TABLE: u8 = 0;

fn init_crc32_table(){
    for i in 0..255 {
        let mut crc = i;
        for j in 0..7 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xed888320;
            }
            else{
                crc >>= 1;
            }
        }

        unsafe {
            CRC_TABLE[i] = crc as u32;
        }
    }
    unsafe {
        ALGORITHM_CRC_TABLE = 1;
    }
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xffffffff;

    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xff) as usize;
        crc = (crc >> 8) ^ unsafe{ CRC_TABLE[index] };
    }

    crc ^ 0xffffffff
}

pub fn CRC32(use_data: &[u8]) -> u64{
    if unsafe{ ALGORITHM_CRC_TABLE } == 0 {
        init_crc32_table();
    }
    crc32(use_data) as u64
}