#![allow(unused)]

use std::net::{IpAddr, TcpStream};
use std::time::Duration;
const TIMEOUT: Duration = Duration::from_millis(500);
// Структура для результатов

pub struct ScanResult {
    pub port: u16,
    pub is_open: bool,
}

pub fn check_port(ip: IpAddr, start_port: u16, end_port: u16) -> Vec<ScanResult>{
    let mut results = Vec::new();
    for port in start_port..=end_port {
        let addr = std::net::SocketAddr::new(ip, port);
        match TcpStream::connect_timeout(&addr, TIMEOUT) {
            Ok(_) => {
                results.push(ScanResult { port: port, is_open: true});
            }
            Err(_) => {}
        }
    }
    results
}