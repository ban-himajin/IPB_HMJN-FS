use std::{result, error};


pub fn Output_err_log(err: Box<dyn error::Error>){
    println!("{}",err);
}