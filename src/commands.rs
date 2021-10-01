use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use rand::Rng;

use crate::{
    balance::{add_balance, get_balance},
    fight::fightmenu,
    health::{add_health, buy_gapple, get_gapple, get_hp, remove_gapple},
    mine::{price_pico, upgrade_pico},
};
use crate::{mine::buy_pico, mine::mine};

pub fn work() {
    let mut rng = rand::thread_rng();
    let amount = rng.gen_range(500..1000);
    add_balance(amount);
    println!("Trabajaste y ganaste ${}", amount);
    sleep();
}

pub fn shop() {
    loop {
        let bal = get_balance();
        print!("\x1B[2J\x1B[1;1H");
        println!("Shop \n 1) Pico \n 2) Gapple \n 3) Back");
        println!("Balance: ${}", bal);
        print!("Option: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();

        std::io::stdin().read_line(&mut option).unwrap();

        match option.as_str().trim() {
            "1" => {
                buy_pico();
                sleep();
            }
            "2" => {
                buy_gapple();
                sleep();
            }
            "3" => {
                break;
            }
            _ => {
                println!("Producto no encontrado");
                sleep();
            }
        }
    }
}

pub fn minar() {
    mine();
}

pub fn sleep() {
    thread::sleep(Duration::from_millis(1000));
}

pub fn upgrade() {
    loop {
        let bal = get_balance();
        print!("\x1B[2J\x1B[1;1H");
        let upg_amount = price_pico();
        println!("Upgrades \n 1) Pico: {} \n 2) Back", upg_amount);
        println!("Balance: ${}", bal);
        print!("Option: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();

        std::io::stdin().read_line(&mut option).unwrap();

        match option.as_str().trim() {
            "1" => {
                upgrade_pico();
                sleep();
            }
            "2" => {
                break;
            }
            _ => {
                println!("Producto no encontrado");
                sleep();
            }
        }
    }
}

pub fn fight() {
    let hp = get_hp();
    if hp <= 0 {
        println!("Necesitas curarte primero");
        sleep();
    } else if hp <= 50 && hp > 0 {
        println!("Sería recomendable que te cures ");
        sleep();
        fightmenu();
    } else {
        fightmenu();
    }
}

pub fn gapple() {
    let gpam = get_gapple();
    let hp = get_hp();

    if gpam <= 0 {
        println!("No tienes ninguna gapple");
        sleep();
    } else {
        if hp >= 100 {
            println!("Tienes la salud al maximo");
            sleep();
        } else {
            remove_gapple();
            add_health(10);
            println!("Has recuperado 10 puntos de salud");
            sleep();
        }
    }
}
