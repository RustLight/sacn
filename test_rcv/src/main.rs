#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sacn;
use sacn::recieve::DMXData;
use std::io::{Error};

fn main() {
    // let universe: u16 = 1;
    // let mut reciever = DmxReciever::listen_universe(universe).unwrap();
    
    // loop {
    //     match reciever.recv_data_blocking(){
    //         Ok(data) => displayData(data),
    //         Err(e) => displayErr(e)
    //     } 
    // }
}

fn _display_data(data: Vec<DMXData>){
    println!("START RECEIVED DATA");
    for d in data {
        println!("Universe: {} Start Code: {}", d.universe, d.start_code);
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
