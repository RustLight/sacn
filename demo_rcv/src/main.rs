#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sacn;
use sacn::recieve::DMXData;
use std::io::{Error};

fn main() {
    
}

fn _display_data(data: Vec<DMXData>){
    println!("START RECEIVED DATA");
    for d in data {
        println!("Universe: {}", d.universe);
        for v in d.values {
            print!("{}", v);
        }
        println!("");
    }
    println!("END RECEIVED DATA");
}

fn _display_err(err: Error){
    println!("Error Encountered: {}", err);
}
