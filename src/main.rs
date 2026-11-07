//Добавляем модули в проект
mod cli;
mod report;
mod scanner;
//Точка входа в програму
fn main() {
    // Импортируем cli для работы
    match cli::parse_args() {
        Ok((ip, start_port, end_port)) => {
            println!("Scanning {ip} from {start_port} to {end_port}");
            let results = scanner::check_port(ip, start_port, end_port);
            println!("Scan complete. Found at {}", results.len());
        },
        Err(e) => eprintln!("Program error! Error: {e}"),
    }
}
