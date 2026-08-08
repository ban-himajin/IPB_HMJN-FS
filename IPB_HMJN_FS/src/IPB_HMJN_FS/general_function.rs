use std::string::FromUtf16Error;


pub fn binary_pow(mut basic_num: u64, mut power_num: u64) -> u64{
    let mut result: u64 = 1;

    while power_num > 0{
        if power_num % 2 == 1{
            result *= basic_num;
        }
        
        basic_num *= basic_num;
        power_num /= 2;
    }

    result
}

pub fn round_up(base_num: u64, divisor_num: u64) -> u64{
    (base_num + divisor_num - 1) / divisor_num
}

pub fn str_analysis<'a>(str: &'a str, search_str: &'a str) -> Vec<&'a str>{
    let result: Vec<&str> = str.split(search_str).collect();
    result
}

pub fn to_fixed_array<const N: usize>(src: &[u8]) -> [u8; N]{
    let mut dst = [0u8; N];

    let len = src.len().min(N);
    dst[..len].copy_from_slice(&src[..len]);

    dst
}
