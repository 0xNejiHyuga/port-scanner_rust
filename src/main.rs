use colored::Colorize;

//Добавляем модули в проект
mod cli;
mod report;
mod scanner;
//Точка входа в програму
fn main() {
    // Banner
    report::greetings();
    // Импортируем cli для работы

    match cli::parse_args() { 

        //Парсим аргументы для программы
        
        Ok((ip, start_port, end_port)) => { // Если Ок и присылает нам кортеж, то идём дальше
            let signatures = scanner::load_signatures(); // <--- Ошибка здесь
            println!("Scanning {ip} from {start_port} to {end_port}");
            let results = scanner::check_port(ip, start_port, end_port, &signatures);
            report::print_results(&results);
        },
        Err(e) => eprintln!("{} {}", "[-] Error:".red().bold(), e),
    }
}
