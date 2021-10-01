use std::{
    fs,
    io::{self, Write},
};

use rand::Rng;

use crate::{
    balance::{add_balance, get_balance, remove_balance},
    commands::sleep,
    enemy::{check_enemy, get_enemyhp, remove_enemyhp},
    health::{check_hp, get_hp, remove_health, set_gp},
};

pub fn fightmenu() {
    loop {
        check_hp();
        check_enemy();
        // Get Health
        let hp = get_hp();
        let balance = get_balance();
        let enemyhp = get_enemyhp();
        if enemyhp <= 0 {
            fs::remove_file(".objects/enemy").unwrap();
            break;
        }
        if hp <= 0 {
            remove_health(hp);
            remove_balance(balance);
            set_gp(0);
            fs::remove_file(".objects/enemy").unwrap();
            break;
        }
        // Clear Console
        print!("\x1B[2J\x1B[1;1H");
        // Menu
        println!("Fight! \n 1) Punch \n 2) Leave");
        println!("HP: {}", hp);
        println!("EnemyHP: {}", enemyhp);
        print!("Option: ");
        // If Death
        io::stdout().flush().unwrap();
        // let mut option = String::new();
        let mut option = String::new();

        std::io::stdin().read_line(&mut option).unwrap();

        match option.as_str().trim() {
            "1" => {
                punch();
                sleep();
            }
            "2" => {
                println!("COBARDE");
                sleep();
                break;
            }
            _ => {
                println!("Opción no valida");
                sleep();
            }
        }
    }
}

pub fn punch() {
    let mut rng = rand::thread_rng();
    let enemyhp = rng.gen_range(25..40);
    let hp = rng.gen_range(25..40);
    remove_enemyhp(enemyhp);
    let enhp = get_enemyhp();
    if enhp <= 0 {
        let mut rng = rand::thread_rng();
        let amount = rng.gen_range(5000..10000);
        add_balance(amount);
        println!("Derrotaste al enemigo y ganaste ${}", amount);
    } else {
        remove_health(hp);
        let myhp = get_hp();
        if myhp <= 0 {
            println!("Fuiste derrotado, quedandote sin dinero");
        } else {
            println!(
                "Has golpeado al enemigo quitandole {} de salud y el te quitó {} de salud",
                enemyhp, hp
            );
        }
    }
}
