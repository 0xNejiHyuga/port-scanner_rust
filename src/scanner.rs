#![allow(unused)]

use std::collections::HashMap;
use std::net::{IpAddr, TcpStream};
use std::time::Duration;
const TIMEOUT: Duration = Duration::from_millis(50);
const CHUNKSIZE: u16 = 1000;
use std::sync::mpsc;
use std::thread; 
// Структура для результатов

pub struct ScanResult {
    pub port: u16,
    pub is_open: bool,
    pub service: String,
}

pub fn check_port(ip: IpAddr, start_port: u16, end_port: u16, signatures: &HashMap<u16, String>) -> Vec<ScanResult> {
    // tx - sender, rx - receiver
    // https://doc.rust-lang.org/std/sync/mpsc/

    let (tx, rx) = mpsc::channel();
    
    // Final results
    let mut final_results: Vec<ScanResult> = Vec::new();

    // https://doc.rust-lang.org/std/thread/
    thread::scope(|s| {
        let mut current_start = start_port;

        while current_start <= end_port {

            let current_end: u16 = std::cmp::min(current_start.saturating_add(CHUNKSIZE - 1), end_port);
            
            // Клонируем передатчик для этого потока
            let tx_clone: mpsc::Sender<ScanResult> = tx.clone();
            
            // 5. ЗАПУСКАЕМ ПОТОК для этого куска
            s.spawn(move || {

                for port in current_start..=current_end {
                    let addr: std::net::SocketAddr = std::net::SocketAddr::new(ip, port);
                    
                    if TcpStream::connect_timeout(&addr, TIMEOUT).is_ok() {
                        let service_name = signatures.get(&port).cloned().unwrap_or_else(|| "Unknown".to_string());
                        
                        // Send result
                        let _ = tx_clone.send(ScanResult { port, is_open: true, service: service_name });
                    }
                }
            });
            
            if current_end == end_port {
                break;
            }
            current_start = current_end + 1;
        }
    });

    drop(tx); 


    for result in rx {
        final_results.push(result);
    }

    final_results.sort_by_key(|result| result.port);
    final_results
}

pub fn load_signatures() -> HashMap<u16, String> {
    let mut map: HashMap<u16, String> = HashMap::new();
    

    let file_content = include_str!("signatures.txt");

    for line in file_content.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        match line.split_once(':') {
            Some((port_str, service_str)) => {
                if let Ok(port) = port_str.trim().parse::<u16>() {
                    map.insert(port, service_str.trim().to_string());
                }
            }
            None => {}
        }
    }
    map
}