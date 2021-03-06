pub mod balance;
pub mod commands;
pub mod debug;
pub mod enemy;
pub mod fight;
pub mod health;
pub mod mine;

use std::io::{self, Write};

use crate::balance::{check_balance, get_balance};
use crate::commands::{fight, gapple, minar, shop, sleep, upgrade, work};
use crate::debug::debug_main;
use crate::health::{check_gapple, check_hp, get_gapple, get_hp, set_hp};

fn main() {
    loop {
        // Check balance if not exist
        check_balance();
        // Check Gapple
        check_gapple();
        //Check Health
        check_hp();
        // Get balance value
        let bal = get_balance();
        // Get Health
        let hp = get_hp();
        if hp >= 100 {
            set_hp(100);
        } else {
            println!("Error en los puntos de salud")
        }
        let hp = get_hp();
        // Get Gapples
        let gpam = get_gapple();
        // Clear Console
        print!("\x1B[2J\x1B[1;1H");
        // Menu
        println!("RustEconomyGame \n 1) Work \n 2) Shop \n 3) Mina \n 4) Upgrade \n 5) Gapple \n 6) Fight \n 7) Exit");
        // Balance
        println!("\nBalance: ${}", bal);
        println!("HP: {}", hp);
        println!("Gapples: {}", gpam);
        print!("\nOption: ");
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
                gapple();
            }
            "6" => {
                fight();
            }
            "7" => {
                println!("Adios!");
                break;
            }
            "232" => {
                debug_main();
            }
            _ => {
                println!("Opción no valida");
                sleep();
            }
        }
    }
}
