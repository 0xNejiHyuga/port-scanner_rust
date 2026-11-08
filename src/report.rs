// Interface
#![allow(unused)]

// Using colored for beautiful output
use colored::Colorize;
use crate::scanner::ScanResult;

// Print banner + Author
pub fn greetings() -> () {
    let banner = r#" 
 __   __          _    ___     _   _       _ _ 
 \ \ / /   _ _ __(_)  ( _ )   | \ | | ___ (_|_)
  \ V / | | | '__| |  / _ \/\ |  \| |/ _ \| | |
   | || |_| | |  | | | (_>  < | |\  |  __/| | |
   |_| \__,_|_|  |_|  \___/\/ |_| \_|\___|/ |_|
                                        |__/   "#;

    println!("{}", banner.purple().bold());

        println!(
        "{}", 
        format!("   󰒍 [ Port Scanner v{} ]", env!("CARGO_PKG_VERSION")).cyan().bold()
    );
    println!("{}", "   󰒃 [ Author: 0xNejiHyuga and Yuri ]\n".cyan());
}

// Print result function
pub fn print_results(results: &[ScanResult]) -> () {
    // Делаем красивый заголовок с линиями
    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".purple());
    println!("{}", "             SCAN RESULTS                 ".purple().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".purple());

    if results.is_empty() {
        println!("{}", "  No open target ports were detected.  ".yellow().italic());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n".purple());
        return;
    }

    for result in results {
        if result.is_open {
            println!(
                " {} {:<6} | {} {:<10}", 
                "[+]".green().bold(), 
                result.port.to_string().cyan().bold(), 
                "".purple(), 
                result.service.magenta()
            );
        }
    }

    //Make statistics
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".purple());
    println!(
        " {} Total open ports: {}", 
        "::".purple().bold(), 
        results.len().to_string().green().bold()
    );
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n".purple());
}