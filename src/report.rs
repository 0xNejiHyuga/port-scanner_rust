// Interface
#![allow(unused)]
// Using colored for beatiful output
use colored::{ Colorize};

use crate::scanner::ScanResult;
// Print banner + Author
pub fn greetings() -> () {
    let banner = r#" __   __          _    ___     _   _       _ _ 
 \ \ / /   _ _ __(_)  ( _ )   | \ | | ___ (_|_)
  \ V / | | | '__| |  / _ \/\ |  \| |/ _ \| | |
   | || |_| | |  | | | (_>  < | |\  |  __/| | |
   |_| \__,_|_|  |_|  \___/\/ |_| \_|\___|/ |_|
                                        |__/   "#;

    println!("{}", banner.purple().bold());

    println!("{}", "   [ Port Scanner v1.0.1 ]".cyan()); //Version
    println!("{}", "   [ Author: 0xNejiHyuga and Yuri (blood3lust]\n".cyan());
}
// Print result function
pub fn print_results(results: &[ScanResult]) -> () {
    println!("\n=== Results ===");
    
    if results.is_empty() {
        println!("{}", "No open target ports were detected".yellow());
        return
    }
    for result in results {
        if result.is_open == true {
            println!("[+] Port {:<5} {}", result.port, "is open!".green());
        }
    }

}