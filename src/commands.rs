use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use rand::Rng;

use crate::{
    balance::add_balance,
    fight::fightmenu,
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
    print!("\x1B[2J\x1B[1;1H");
    println!("Shop \n 1) Pico \n 2) Rod \n 3) Back");
    print!("Option: ");
    io::stdout().flush().unwrap();
    let mut option = String::new();

    std::io::stdin().read_line(&mut option).unwrap();

    if option.trim() == "1" {
        buy_pico();
        sleep();
    } else if option.trim() == "2" {
        println!("Compraste una Rod");
        sleep();
    } else if option.trim() == "3" {
        println!("Regresando...");
    } else {
        println!("Producto no encontrado");
        sleep();
    }
}

pub fn minar() {
    mine();
}

pub fn sleep() {
    thread::sleep(Duration::from_millis(1000));
}

pub fn upgrade() {
    print!("\x1B[2J\x1B[1;1H");
    let upg_amount = price_pico();
    println!("Upgrades \n 1) Pico: {} \n 2) Back", upg_amount);
    print!("Option: ");
    io::stdout().flush().unwrap();
    let mut option = String::new();

    std::io::stdin().read_line(&mut option).unwrap();

    if option.trim() == "1" {
        upgrade_pico();
        sleep();
    } else if option.trim() == "2" {
        println!("Regresando...");
    } else {
        println!("Producto no encontrado");
        sleep();
    }
}

pub fn fight() {
    fightmenu();
}
