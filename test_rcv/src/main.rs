extern crate sacn;
use sacn::DmxSource;
use sacn::recieve::{DmxReciever, ACN_SDT_MULTICAST_PORT, RCV_BUF_DEFAULT_SIZE, DMXData};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::io::{Error};

fn main() {
    let universe: u16 = 1;
    let mut reciever = DmxReciever::listen_universe(universe).unwrap();
    
    loop {
        match reciever.recv_data_blocking(){
            Ok(data) => displayData(data),
            Err(e) => displayErr(e)
        } 
    }
}

fn displayData(data: Vec<DMXData>){
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

fn displayErr(err: Error){
    println!("Error Encountered: {}", err);
}
