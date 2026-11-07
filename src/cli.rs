#![allow(unused)]

use std::net::IpAddr;
//Создаём публичную функцию
pub fn parse_args() -> (Result<(IpAddr, u16, u16), String>) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        return Err(format!("Использование: {} <IP> <start_port> <end_port>", args[0]));
    }
    // Парсинг
    let ip = match args[1].parse::<IpAddr>() {
        Ok(ip) => ip,
        Err(e) => {
            println!("IP address parsing error!");
            return Err("Not valid IP!".to_string());
        },
    };
    let start_port = match args[2].parse::<u16>() {
        Ok(start_port) => start_port,
        Err(e) => {
            println!("Start port parsing error!");
            return Err("No valid start port!".to_string());
        },
    };
    let end_port = match args[3].parse::<u16>() {
        Ok(end_port) => end_port,
        Err(e) => {
            println!("End port parsing error!");
            return Err("No valid end port!".to_string());
        },
    };
    Ok((ip, start_port, end_port))
    

}