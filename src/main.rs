pub mod balance;
pub mod commands;
pub mod enemy;
pub mod fight;
pub mod health;
pub mod mine;

use std::io::{self, Write};

use crate::balance::{check_balance, get_balance};
use crate::commands::{fight, minar, shop, sleep, upgrade, work};
use crate::health::{check_hp, get_hp};

fn main() {
    loop {
        // Check balance if not exist
        check_balance();
        //Check Health
        check_hp();
        // Get balance value
        let bal = get_balance();
        // Get Health
        let hp = get_hp();
        // Clear Console
        print!("\x1B[2J\x1B[1;1H");
        // Menu
        println!("RustEconomyGame \n 1) Work \n 2) Shop \n 3) Minar \n 4) Upgrade \n 5) Fight \n 6) Exit");
        // Balance
        println!("Balance: ${}", bal);
        println!("HP: {}", hp);
        print!("Option: ");
        io::stdout().flush().unwrap();
        // let mut option = String::new();
        let mut option = String::new();

        std::io::stdin().read_line(&mut option).unwrap();

        match option.as_str().trim() {
            "1" => {
                work();
            }
            "2" => {
                shop();
            }
            "3" => {
                minar();
            }
            "4" => {
                upgrade();
            }
            "5" => {
                fight();
            }
            "6" => {
                println!("Adios!");
                break;
            }
            _ => {
                println!("Opción no valida");
                sleep();
            }
        }
    }
}
